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
* The datas extracted from a sound file.
*
* Datas extracted from the file are dispatched between two struct, 
* a first who contains all the tags of the sound,
* a second who contains the samples.
* SoundDatas are made to be share between several Sound and play in the same time.
*
* # Example
* ```
* // Create a SoundData
* let snd_data = @SoundData::new(~"path/to/my/sound.wav").unwrap();
*
* // Create two Sound with the same SoundData
* let snd1 = Sound::new_with_data(snd_data).unwrap();
* let snd2 = Sound::new)with_data(snd_data).unwrap();
*
* // Play the sounds
* snd1.play();
* snd2.play();
* ```
*/

use sndfile::*;
use std::{vec, sys};
use openal::*;
use std::libc::c_void;

/**
* Structure containing the tags of a sound. 
*
* If the tags doesn't exist in the sound file, the string is ~"".
*/
#[deriving(Clone)]
pub struct SoundTags {
    /// The title of the sound as a ~str
    Title       : ~str,
    /// The Copyright of the sound as a ~str
    Copyright   : ~str,
    /// The name of the software used to create the sound as a ~str
    Software    : ~str,
    /// The name of the artist of the sound as a ~str
    Artist      : ~str,
    /// A comment as a ~str
    Comment     : ~str,
    /// The creation date of the sound as a ~str
    Date        : ~str,
    /// The name of the album where the sound come from as a ~str
    Album       : ~str,
    /// The license of the sound as a ~str
    License     : ~str,
    /// The tracknumber of the sound as a ~str
    TrackNumber : ~str,
    /// The genre of the sound as a ~str
    Genre       : ~str
}

/// Structure containing the data extracted from the sound file.
pub struct SoundData {
    /// The SoundTags who contains all the information of the sound
    priv sound_tags     : ~SoundTags,
    /// The sndfile samples information
    priv snd_info       : ~SndInfo,
    /// The total samples count of the Sound
    priv nb_sample      : i64,
    /// The OpenAl internal identifier for the buffer
    priv al_buffer      : u32
}

impl SoundData {
    #[fixed_stack_segment] #[inline(never)]
    pub fn new(path : ~str) -> Option<SoundData> {
        let mut file;

        match SndFile::new(path, Read) {
            Ok(file_) => file = file_,
            Err(err) => { println!("{}", err); return None; }
        };

        let infos = file.get_sndinfo();

        let nb_sample = infos.channels as i64 * infos.frames;

        let mut samples = vec::from_elem(nb_sample as uint, 0i16);
        file.read_i16(samples, nb_sample as i64);

        let mut buffer_id = 0;
        let len = sys::size_of::<i16>() * (samples.len());
        let format =  match infos.channels {
            1 => ffi::AL_FORMAT_MONO16,
            2 => ffi::AL_FORMAT_STEREO16,
            _ => { println!("Internal error : unrecognized format."); return None; }
        };

         unsafe {
            ffi::alGenBuffers(1, &mut buffer_id);
            ffi::alBufferData(buffer_id, format, vec::raw::to_ptr(samples) as *c_void, len as i32, infos.samplerate);
        }

        match openal_has_error() {
            Some(err)   => { println!("{}", err); return None; },
            None        => {}
        };

        let sound_data = SoundData {
            sound_tags  : SoundData::get_sound_tags(&file),
            snd_info    : infos,
            nb_sample   : nb_sample,
            al_buffer   : buffer_id
        };
        file.close();
        
        Some(sound_data)
    }

    fn get_sound_tags(file : &SndFile) -> ~SoundTags {
        ~SoundTags {
            Title       : file.get_string(Title).unwrap_or(~""),
            Copyright   : file.get_string(Copyright).unwrap_or(~""),
            Software    : file.get_string(Software).unwrap_or(~""),
            Artist      : file.get_string(Artist).unwrap_or(~""),
            Comment     : file.get_string(Comment).unwrap_or(~""),
            Date        : file.get_string(Date).unwrap_or(~""),
            Album       : file.get_string(Album).unwrap_or(~""),
            License     : file.get_string(License).unwrap_or(~""),
            TrackNumber : file.get_string(TrackNumber).unwrap_or(~""),
            Genre       : file.get_string(Genre).unwrap_or(~"")
        }
    }

    pub fn get_samplerate(&self) -> i32 {
        self.snd_info.samplerate
    }

    pub fn get_tags(&self) -> ~SoundTags {
        self.sound_tags.clone()
    }

    pub fn get_buffer(&self) -> u32 {
        self.al_buffer
    }

    pub fn get_sample_count(&self) -> i64 {
        self.nb_sample
    } 
    pub fn get_channel_count(&self) -> i32 {
        self.snd_info.channels
    }
}

impl Drop for SoundData {
    /**
    * Destroy all the resources attached to the SoundData.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) -> () {
        unsafe {
            ffi::alDeleteBuffers(1, &mut self.al_buffer);
        }
    }
}