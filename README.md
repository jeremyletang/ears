# ears

__ears__ is a simple library for play Sounds and Musics in Rust.

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
# Compile examples

Like __ears__ you can build the examples with `rustpkg`, just do :

```Shell
> rustpkg install examples/an_example
```
