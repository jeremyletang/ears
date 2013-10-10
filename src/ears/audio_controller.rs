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
* The functionnality that a Sound or a Music should implement.
*/

use states::State;

pub trait AudioController {
	fn play(&mut self) -> ();
	fn pause(&mut self) -> ();
	fn stop(&mut self) -> ();
	fn is_playing(&self) -> bool;
	fn get_state(&self) -> State;
	fn set_volume(&mut self, volume : f32) -> ();
	fn get_volume(&self) -> f32;
	fn set_min_volume(&mut self, min_volume : f32) -> ();
	fn get_min_volume(&self) -> f32;
	fn set_max_volume(&mut self, max_volume : f32) -> ();
	fn get_max_volume(&self) -> f32;
	fn set_looping(&mut self, looping : bool) -> ();
	fn is_looping(&self) -> bool;
	fn set_pitch(&mut self, pitch : f32) -> ();
	fn get_pitch(&self) -> f32;
	fn set_relative(&mut self, relative : bool) -> ();
	fn is_relative(&mut self) -> bool;
	fn set_position(&mut self, position : [f32, ..3]) -> ();
	fn get_position(&self) -> [f32, ..3];
	fn set_direction(&mut self, direction : [f32, ..3]) -> ();
	fn get_direction(&self)  -> [f32, ..3];
	fn set_max_distance(&mut self, max_distance : f32) -> ();
	fn get_max_distance(&self) -> f32;
	fn set_reference_distance(&mut self, ref_distance : f32) -> ();
	fn get_reference_distance(&self) -> f32;
	fn set_attenuation(&mut self, attenuation : f32) -> ();
	fn get_attenuation(&self) -> f32;
}