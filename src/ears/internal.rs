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

#[macro_escape];

use std::{local_data, ptr};
use openal::ffi;

static al_context : local_data::Key<~OpenAlData> = &local_data::Key;

#[deriving(Clone)]
pub struct OpenAlData {
    priv al_context : *ffi::ALCcontext,
    priv al_device : *ffi::ALCdevice
}

impl OpenAlData {
    /**
    * Create a new OpenAlData struct
    *
    * Private method.
    */
    fn new() -> Result<OpenAlData, ~str> {
        let device = unsafe { ffi::alcOpenDevice(ptr::null()) };
        if device.is_null() { return Err(~"Internal error : cannot open the default device."); }  
        let context = unsafe { ffi::alcCreateContext(device, ptr::null()) };
        if context.is_null() { return Err(~"Internal error : cannot create the OpenAL context."); }
        if unsafe { ffi::alcMakeContextCurrent(context) } == ffi::ALC_FALSE {
            return Err(~"Internal error : cannot make the OpenAL context current.");
        }
        
        Ok(
            OpenAlData {
                al_context : context,
                al_device : device  
            }
        )
    }

    /**
    * Check if the context is created.
    *
    * This function check is the OpenAl context is already created. If context doesn't exist, create it,
    * and store it in a local_data, else get it from the local data and return it.
    *
    * # Return
    * A result containing nothing if the OpenAlData struct exist, otherwise an error message.
    */
    pub fn check_al_context() -> Result<(), ~str> {
        if unsafe { ffi::alcGetCurrentContext() } != ptr::null() {
            return Ok(())
        }
        local_data::get(al_context, |openal_data| {
            match openal_data {
                Some(_)    => Ok(()),
                None        => {
                    match OpenAlData::new() {
                        Ok(al_data) => { local_data::set(al_context, ~al_data); Ok(()) },
                        Err(err)    => Err(err)
                    }
                }
            }
        })
    }
}

impl Drop for OpenAlData {
    fn drop(&mut self) {
        unsafe {
            ffi::alcDestroyContext(self.al_context);
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