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
# ears

__ears__ is a simple library for play Sounds and Musics in Rust.

__ears__ is build on the top of OpenAL and libsndfile.

* Provide anaccess to the OpenAL spatialization functionality in a simple way.
* Accept a lot of audio formats thanks to libsndfile.

# A simple example

```Rust
extern crate ears;
use ears::Sound;

fn main() {
	// Create a new Sound.
	let snd = Sound::new(~"path/to/my/sound.ogg").unwrap();

	// Play the Sound
	snd.play();

	// Wait until the end of the sound
	while snd.is_playing() {}
}
```

# Functionnality

__ears__ provide two way for play audio files.

* The Sound class, which represent light sounds who can share a buffer of samples with another Sound.
* The Music class, which is a bigger sound and who can't share sample buffer.

# Use ears

As said before, __ears__ require OpenAL and libsndfile, you need to install these two librarieson your system.

__ears__ compiles against the last Rust compiler, so if it doesn't work on your computer you may need to update your compiler.

__ears__ is built using make, so just type `make` at the root of the __ears__ repository, this command
build __ears__, the examples, and the documentation.

You can build them separately too with the dedicated commands:

```Shell
> make ears
> make examples
> make doc
```

then import stuff from __ears__ in your project, you can import all the stuff :

```Rust
#[feature(globs)];
extern crate ears;

use ears::*;
```

or a specific one:

```Rust
extern crate ears;

use ears::Music;
```
*/

#![crate_id = "ears#0.3"]
#![desc = "Easy Api in Rust for Sounds"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![warn(missing_doc)]
#![allow(dead_code)]
#![feature(macro_rules)]

extern crate libc;

// Reexport public API
pub use einit::{init, init_in};
pub use music::Music;
pub use sound::Sound;
pub use states::{State, Initial, Playing, Paused, Stopped};
pub use sound_data::SoundData;
pub use audio_controller::AudioController;
pub use audio_tags::{AudioTags, Tags};
pub use recorder::Recorder;
pub use record_context::RecordContext;


// Hidden internal bindings
mod internal;
mod openal;
mod sndfile;

// The public ears API

#[path = "init.rs"]
mod einit;
pub mod listener;
mod sound;
mod music;
mod sound_data;
mod states;
mod audio_controller;
mod audio_tags;
mod recorder;
mod record_context;
