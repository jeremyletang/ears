// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

/*!
* Internal class to handle OpenAl context and device.
*
* Work as a Singleton, check_al_context must be called before each OpenAl object
* to be sure that the context is created.
*/

#![macro_escape]
#![allow(raw_pointer_deriving)]

use std::{local_data, ptr};
use openal::ffi;
use record_context;
use record_context::RecordContext;

static al_context: local_data::Key<~OpenAlData> = &local_data::Key;

#[deriving(Clone)]
pub struct OpenAlData {
    priv al_context: *ffi::ALCcontext,
    priv al_device: *ffi::ALCdevice,
    priv al_capt_device:  *ffi::ALCdevice
}

impl OpenAlData {
    /**
     * Create a new OpenAlData struct
     *
     * Private method.
     */
    fn new() -> Result<OpenAlData, ~str> {
        let device = unsafe { ffi::alcOpenDevice(ptr::null()) };
        if device.is_null() {
            return Err(~"Internal error: cannot open the default device.");
        }
        let context = unsafe { ffi::alcCreateContext(device, ptr::null()) };
        if context.is_null() {
            return Err(~"Internal error: cannot create the OpenAL context.");
        }
        if unsafe { ffi::alcMakeContextCurrent(context) } == ffi::ALC_FALSE {
            return Err(~"Internal error: cannot make the OpenAL context current.");
        }

        Ok(
            OpenAlData {
                al_context: context,
                al_device: device,
                al_capt_device: ptr::null()
            }
        )
    }

    /**
     * Check if the context is created.
     *
     * This function check is the OpenAl context is already created.
     * If context doesn't exist, create it, and store it in a local_data,
     * else get it from the local data and return it.
     *
     * # Return
     * A result containing nothing if the OpenAlData struct exist,
     * otherwise an error message.
     */
    pub fn check_al_context() -> Result<(), ~str> {
        if unsafe { ffi::alcGetCurrentContext() } != ptr::null() {
            return Ok(())
        }
        local_data::get(al_context, |openal_data| {
            match openal_data {
                Some(_)     => Ok(()),
                None        => {
                    match OpenAlData::new() {
                        Ok(al_data) => {
                            local_data::set(al_context, ~al_data); Ok(())
                        },
                        Err(err)    => Err(err)
                    }
                }
            }
        })
    }

    fn is_input_context_init() -> Result<RecordContext, ~str> {
        local_data::get_mut(al_context, |openal_data| {
            match openal_data {
                Some(d) => {
                    if d.al_capt_device.is_not_null() {
                        Ok(record_context::new(d.al_capt_device))
                    } else {
                        if "ALC_EXT_CAPTURE".with_c_str(|c_str| unsafe {
                            ffi::alcIsExtensionPresent(d.al_device, c_str) }) == ffi::ALC_FALSE {
                            return Err(~"Error: no input device available on your system.")
                        } else {
                            d.al_capt_device = unsafe {
                                ffi::alcCaptureOpenDevice(ptr::null(),
                                                          44100,
                                                          ffi::AL_FORMAT_MONO16,
                                                          44100) };
                            if d.al_capt_device.is_null() {
                                Err(~"Internal error: cannot open the default capture device.")
                            } else {
                                Ok(record_context::new(d.al_capt_device))
                            }
                        }
                    }
                }   ,
                None    => Err(~"Error: you must request the input context, in the task where you initialize ears.")
            }
        })
    }

    /**
     * Check if the input context is created.
     *
     * This function check if the input OpenAl context is already created.
     * The input openAL context need the normal AL context + its own extension.
     * So check if the context exist first, then load the input extension.
     *
     * # Return
     * A result containing nothing if the OpenAlData struct exist,
     * otherwise an error message.
     */
    pub fn check_al_input_context() -> Result<RecordContext, ~str> {
        if unsafe { ffi::alcGetCurrentContext() } != ptr::null() {
            OpenAlData::is_input_context_init()
        } else {
            match OpenAlData::check_al_context() {
                Ok(_)       => OpenAlData::is_input_context_init(),
                Err(err)    => Err(err)
            }
        }
    }
}

impl Drop for OpenAlData {
    fn drop(&mut self) {
        unsafe {
            ffi::alcDestroyContext(self.al_context);
            if self.al_capt_device.is_not_null() {
                ffi::alcCaptureCloseDevice(self.al_capt_device);
            }
            ffi::alcCloseDevice(self.al_device);
        }
    }
}

macro_rules! check_openal_context(
    ($def_ret:expr) => (
            match OpenAlData::check_al_context() {
                Ok(_)       => {},
                Err(err)    => { println!("{}", err); return $def_ret; }
            }
        );
)
