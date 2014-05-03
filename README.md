# ears [![Build Status](https://travis-ci.org/JeremyLetang/ears.png?branch=master)](https://travis-ci.org/jeremyletang/ears)


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
