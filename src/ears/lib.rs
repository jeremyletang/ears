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

#[feature(globs)];

#[link(name = "ears",
       vers = "0.0.1",
       author = "letang.jeremy@gmail.com",
       uuid = "D0271EC6-5F27-4C72-BBEA-1341DC665F34",
       url = "https://github.com/JeremyLetang/ears")];

#[desc = "Easy Api in Rust for Sounds"];
#[license = "MIT"];
#[crate_type = "lib"];

extern mod extra;
extern mod sndfile;


pub use sound::Sound;
pub use sound_data::{SoundData, SoundTags};
pub use states::{State, Initial, Playing, Paused, Stopped};

#[doc(hidden)]
mod internal;

pub mod listener;
pub mod sound;
pub mod sound_data;
pub mod states;

#[doc(hidden)]
mod openal;
