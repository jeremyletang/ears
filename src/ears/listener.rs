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
* Module for manage the listener in the scene.
* 
*/

use internal::*;
use openal::ffi;

/**
* Set the global volume of the scene.
*
* A value of 1.0 means unattenuated. Each division by 2 equals an attenuation
* of about -6dB. Each multiplicaton by 2 equals an amplification of about
* +6dB.
*
* # Argument
* * `volume` - The global volume for the scene, should be between 0. and 1. 
*/
#[fixed_stack_segment] #[inline(never)]
pub fn set_volume(volume : f32) -> () {
    match OpenAlData::check_al_context() {
        Ok(_)       => {},
        Err(err)    => { println!("{}", err); return; }
    };
    unsafe {
        ffi::alListenerf(ffi::AL_GAIN, volume);
    }
}

/**
* Get the global volume of the scene.
*
* # Return
* The global volume of the scene between 0. and 1.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_volume() -> f32 {
    match OpenAlData::check_al_context() {
        Ok(_)       => {},
        Err(err)    => { println!("{}", err); return 0.; }
    };
    let mut volume : f32 = 0.;
    unsafe {
        ffi::alGetListenerf(ffi::AL_GAIN, &mut volume);
    }
    volume
}

/**
* Set the listener location in three dimensional space.
*
* OpenAL, like OpenGL, uses a right handed coordinate system, where in a
* frontal default view X (thumb) points right, Y points up (index finger), and
* Z points towards the viewer/camera (middle finger). 
* To switch from a left handed coordinate system, flip the sign on the Z
* coordinate.
*
* Default is [0., 0., 0.]. 
*
* # Argument
* * `position` - A three dimensional vector of f32 containing the position of the listener [x, y, z].
*/
#[fixed_stack_segment] #[inline(never)]
pub fn set_position(position : [f32, ..3]) -> () {
    match OpenAlData::check_al_context() {
        Ok(_)       => {},
        Err(err)    => { println!("{}", err); return; }
    };
    unsafe {
        ffi::alListenerfv(ffi::AL_POSITION, &position[0]);
    }
}

/**
* Get the location of the listener in three dimensional space.
*
* # Return
* A three dimensional vector of f32 containing the position of the listener [x, y, z].
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_position() -> [f32, ..3] {
    match OpenAlData::check_al_context() {
        Ok(_)       => {},
        Err(err)    => { println!("{}", err); return [0., ..3]; }
    };
    let mut position : [f32, ..3] = [0., ..3];
    unsafe {
        ffi::alGetListenerfv(ffi::AL_POSITION, &mut position[0]);
    }
    position
}

/**
* Set the orientation of the listener.
*
* Default orientation is : at[0.0, 0.0, -1.0] - up[0.0, 1.0, 0.0]
*
* # Arguments
* * `orientation_at` - The front as a three dimensional vector [x, y, z].
* * `orientation_up` - The top as a three dimensional vector [x, y, z].
*/
#[fixed_stack_segment] #[inline(never)]
pub fn set_orientation(orientation_at : [f32, ..3], orientation_up : [f32, ..3]) -> () {
    match OpenAlData::check_al_context() {
        Ok(_)       => {},
        Err(err)    => { println!("{}", err); return; }
    };
    let orientation : [f32, ..6] = [orientation_at[0], orientation_at[1], orientation_at[2], orientation_up[0], orientation_up[1], orientation_up[2]];
    unsafe {
        ffi::alListenerfv(ffi::AL_ORIENTATION, &orientation[0]);
    }
}

/**
* Get the orientation of the listener.
*
* # Return
* A tuple containing the orientation as two three dimensional vector [x, y, z].
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_orientation() -> ([f32, ..3], [f32, ..3]) {
    match OpenAlData::check_al_context() {
        Ok(_)       => {},
        Err(err)    => { println!("{}", err); return ([0., ..3], [0., ..3]); }
    };
    let mut orientation : [f32, ..6] = [0., ..6];
    unsafe {
        ffi::alGetListenerfv(ffi::AL_ORIENTATION, &mut orientation[0]);
    }
    ([orientation[0], orientation[1], orientation[2]], [orientation[3], orientation[4], orientation[5]])
}

