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

__ears__ is a simple library for play Audio in Rust.

__ears__ is build on the top of OpenAL and libsndfile.

* Provide an acces to the OpenAL spatialization functionnality in a simple way.
* Accept a lot of audio format thanks to libsndfile.

# A simple example

```Rust
extern mod ears;
use ears::Sound;
 
fn main() -> {
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

As said before, __ears__ require OpenAL and libsndfile, you need to install these two libraries in your system.

__ears__ compile against the last Rust compiler, so if it doesn't work on your computer you may need to update your compiler.

__ears__ work fully with `rustpkg`, so just install __ears__ in your project workspace like this :

```Shell
> rustpkg install github.com/JeremyLetang/ears
```

then import stuff from __ears__ in your project :

```Rust
extern mod ears;
use ears::*;
```

*/

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

pub use sound::Sound;
pub use music::Music;
pub use sound_data::{SoundData, SoundTags};
pub use states::{State, Initial, Playing, Paused, Stopped};

#[doc(hidden)]
mod internal;

pub mod listener;
pub mod sound;
pub mod music;
pub mod sound_data;
pub mod states;

#[doc(hidden)]
mod openal;
#[doc(hidden)]
mod sndfile;