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

use std::cell::RefCell;
use std::ptr;
use openal::ffi;
use record_context;
use record_context::RecordContext;

// static al_context: local_data::Key<Box<OpenAlData>> = &local_data::Key;
local_data_key!(al_context: RefCell<Box<OpenAlData>>)

#[deriving(Clone)]
pub struct OpenAlData {
    al_context: *mut ffi::ALCcontext,
    al_device: *mut ffi::ALCdevice,
    al_capt_device: *mut ffi::ALCdevice
}

impl OpenAlData {
    /**
     * Create a new OpenAlData struct
     *
     * Private method.
     */
    fn new() -> Result<OpenAlData, String> {
        let device = unsafe { ffi::alcOpenDevice(ptr::mut_null()) };
        if device.is_null() {
            return Err("Internal error: cannot open the default device.".to_string());
        }
        let context = unsafe { ffi::alcCreateContext(device, ptr::mut_null()) };
        if context.is_null() {
            return Err("Internal error: cannot create the OpenAL context.".to_string());
        }
        if unsafe { ffi::alcMakeContextCurrent(context) } == ffi::ALC_FALSE {
            return Err("Internal error: cannot make the OpenAL context current.".to_string());
        }

        Ok(
            OpenAlData {
                al_context: context,
                al_device: device,
                al_capt_device: ptr::mut_null()
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
    pub fn check_al_context() -> Result<(), String> {
        if unsafe { ffi::alcGetCurrentContext().is_not_null() } {
            return Ok(())
        }
        match al_context.get() {
            Some(_) => Ok(()),
            None    => {
                match OpenAlData::new() {
                    Ok(al_data) => {
                        al_context.replace(Some(RefCell::new(box al_data))); Ok(())
                    },
                    Err(err)    => Err(err)
                }
            }
        }
    }

    fn is_input_context_init() -> Result<RecordContext, String> {
        let is_some = al_context.get().is_some();
        if is_some {
            let mut new_context = *(*al_context.get().unwrap()).borrow().clone();
            if new_context.al_capt_device.is_not_null() {
                Ok(record_context::new(new_context.al_capt_device))
            } else {
                if "ALC_EXT_CAPTURE".with_c_str(|c_str| unsafe {
                    ffi::alcIsExtensionPresent(new_context.al_device, c_str) }) == ffi::ALC_FALSE {
                    return Err("Error: no input device available on your system.".to_string())
                } else {
                    new_context.al_capt_device = unsafe {
                        ffi::alcCaptureOpenDevice(ptr::mut_null(),
                                                  44100,
                                                  ffi::AL_FORMAT_MONO16,
                                                  44100) };
                    if new_context.al_capt_device.is_null() {
                        Err("Internal error: cannot open the default capture device.".to_string())
                    } else {
                        let cap_device = new_context.al_capt_device;
                        al_context.replace(Some(RefCell::new(box new_context)));
                        Ok(record_context::new(cap_device))
                    }
                }
            }
        } else {
            Err("Error: you must request the input context, \
                           in the task where you initialize ears.".to_string())
        }
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
    pub fn check_al_input_context() -> Result<RecordContext, String> {
        if unsafe { ffi::alcGetCurrentContext().is_not_null() } {
            OpenAlData::is_input_context_init()
        } else {
            match OpenAlData::check_al_context() {
                Ok(_)    => OpenAlData::is_input_context_init(),
                Err(err) => Err(err)
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
                Ok(_)    => {},
                Err(err) => { println!("{}", err); return $def_ret; }
            }
        );
)
