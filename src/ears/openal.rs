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
* Minimal binding for OpenAL.
* Bind only functions which are needed by lib sailor
*/



#[link_args="-lopenal"]
extern {}

pub mod ffi {

    use std::libc::{c_char, c_void};

    /// OpenAL types
    pub type ALCboolean = c_char;
    pub static ALC_TRUE :               ALCboolean  = 1;
    pub static ALC_FALSE :              ALCboolean  = 0;

    /// Sound modifier
    pub static AL_GAIN :                i32         = 0x100A;
    pub static AL_PITCH :               i32         = 0x1003;
    pub static AL_SOURCE_RELATIVE :     i32         = 0x202;
    pub static AL_POSITION :            i32         = 0x1004;
    pub static AL_ORIENTATION :         i32         = 0x100F;
    pub static AL_DIRECTION :           i32         = 0x1005;
    pub static AL_LOOPING :             i32         = 0x1007;
    pub static AL_MIN_GAIN :            i32         = 0x100D;
    pub static AL_MAX_GAIN :            i32         = 0x100E;

    /// Sound format
    pub static AL_FORMAT_MONO16 :       i32         = 0x1101;
    pub static AL_FORMAT_STEREO16 :     i32         = 0x1103;

    /// Source params
    pub static AL_BUFFER :              i32         = 0x1009;

    /// Error identifiers
    pub static AL_NO_ERROR :            i32         = 0;
    pub static AL_INVALID_NAME :        i32         = 0xA001;
    pub static AL_INVALID_ENUM :        i32         = 0xA002;
    pub static AL_INVALID_VALUE :       i32         = 0xA003;
    pub static AL_INVALID_OPERATION :   i32         = 0xA004;
    pub static AL_OUT_OF_MEMORY  :      i32         = 0xA005;

    /// Source states
    pub static AL_SOURCE_STATE :        i32         = 0x1010;
    pub static AL_INITIAL :             i32         = 0x1011;
    pub static AL_PLAYING :             i32         = 0x1012;
    pub static AL_PAUSED :              i32         = 0x1013;
    pub static AL_STOPPED :             i32         = 0x1014;



    extern "C" {
        /// Context functions
        pub fn alcCreateContext(device : *ALCdevice, attrlist : *i32) -> *ALCcontext;
        pub fn alcMakeContextCurrent(context : *ALCcontext) -> ALCboolean;
        pub fn alcDestroyContext(context : *ALCcontext);

        /// Device functions
        pub fn alcOpenDevice(devicename : *c_char) -> *ALCdevice;
        pub fn alcCloseDevice(device : *ALCdevice) -> ALCboolean;
    
        /// Listener functions 
        pub fn alListenerf(param : i32, value : f32) -> ();
        pub fn alListener3f(param : i32, value1 : f32, value2 : f32, value3 : f32) -> ();
        pub fn alGetListenerf(param : i32, value : *mut f32) -> ();
        pub fn alGetListener3f(param : f32, value1 : *mut f32, value2 : *mut f32, value3 : *mut f32) -> ();
        pub fn alListenerfv(param : i32, values : *f32) -> ();
        pub fn alGetListenerfv(param : i32, values : *mut f32) -> ();

        /// Sources functions
        pub fn alGenSources(n : i32, sources : *mut u32) -> ();
        pub fn alDeleteSources(n : i32, buffers : *mut u32) -> ();
        pub fn alSourcei(source : u32, param : i32, value : i32) -> ();
        pub fn alSourcef(source : u32, param : i32, value : f32) -> ();
        pub fn alSourcePlay(source : u32) -> ();
        pub fn alSourcePause(source : u32) -> ();
        pub fn alSourceStop(source : u32) -> ();
        pub fn alGetSourcei(source : u32, param : i32, value : *mut i32) -> ();
        pub fn alGetSourcef(source : u32, param : i32, value : *mut f32) -> ();
        pub fn alSourcefv(source : u32, param : i32, value : *f32) -> ();
        pub fn alGetSourcefv(source : u32, param : i32, value : *mut f32) -> ();


        /// Buffers functions
        pub fn alGenBuffers(n : i32, buffers : *mut u32) -> ();
        pub fn alDeleteBuffers(n : i32, buffers : *mut u32);
        pub fn alBufferData(buffer : u32, format : i32, data : *c_void, size : i32, freq : i32) -> ();

        /// Error
        pub fn alGetError() -> i32;
    }

    pub struct ALCdevice;
    pub struct ALCcontext;
}

#[fixed_stack_segment] #[inline(never)]
pub fn openal_has_error() -> Option<~str> {
     match unsafe { ffi::alGetError() } {
        ffi::AL_NO_ERROR            => None,
        ffi::AL_INVALID_NAME        => Some(~"OpenAL error : Invalid name paramater passed to AL call."),
        ffi::AL_INVALID_ENUM        => Some(~"OpenAL error : Invalid enum parameter passed to AL call."),
        ffi::AL_INVALID_VALUE       => Some(~"OpenAL error : Invalid value parameter passed to AL call."),
        ffi::AL_INVALID_OPERATION   => Some(~"OpenAL error : Illegal AL call."),
        ffi::AL_OUT_OF_MEMORY       => Some(~"OpenAL error : Not enough memory."),
        _                           => Some(~"OpenAL internal error : Unknow error.")
    }
}

