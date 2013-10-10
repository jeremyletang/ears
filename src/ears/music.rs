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
* Play Music easily.
*
* Simple class to play musics easily in 2 lines.
*
* The musics are played in them own task and load the samples progressively using circular buffers.
* They are not associated to a SoundData like Sounds.
*
* # Examples 
* ```
* extern mod ears;
* use ears::Music;
*
* fn main() -> () {
*    // Load a Music
*   let msc = Music::new(~"path/to/my/sound.ogg").unwrap();
*
*   // Play it
*   msc.play();
* }
* ```
*/

use internal::*;
use openal::{ffi, al};
use sndfile::*;
use states::*;
use std::rt::io::timer::sleep;

use std::{vec, sys};
use std::libc::c_void;
use std::task::*;

/// Class for play Musics
pub struct Music {
    /// The internal OpenAL source identifier
    priv al_source  : u32,
    /// The internal OpenAL buffers
    priv al_buffers : [u32, ..2],
    /// The file open with libsndfile
    priv file : Option<~SndFile>,
    /// Information of the file
    priv file_infos : ~SndInfo,
    /// Quantity of sample to read each time
    priv sample_to_read : i32,
    /// Format of the sample
    priv sample_format : i32
}

impl Music {
    #[fixed_stack_segment] #[inline(never)]
    pub fn new(path : &str) -> Option<Music> {
        // Check that OpenAL is launched
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return None; }
        };
        // Retrieve File and sound datas
        let file = match SndFile::new(path, Read) {
            Ok(file)    => ~file,
            Err(err)    => { println!("{}", err); return None; }
        };
        let infos = file.get_sndinfo();

        // create the source and the buffers
        let mut source_id = 0;
        let mut buffer_ids = [0, ..2];
        unsafe {
            // create the source
            ffi::alGenSources(1, &mut source_id);
            // create the buffers
            ffi::alGenBuffers(2, &mut buffer_ids[0]);
        }

        // Retrieve format informations
        let format =  match al::get_channels_format(infos.channels) {
            Some(fmt) => fmt,
            None => { println!("Internal error : unrecognized format."); return None; }
        };

        // Check if there is OpenAL internal error
        match al::openal_has_error() {
            Some(err) => { println!("{}", err); return None; },
            None => {} 
        };

        Some( Music {
            al_source : source_id,
            al_buffers : buffer_ids,
            file : Some(file),
            file_infos : infos,
            sample_to_read : 50000,
            sample_format : format
        })        
    }

    #[fixed_stack_segment] #[inline(never)]
    pub fn play(&mut self) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };

        match self.get_state() {
            Paused   => { al::alSourcePlay(self.al_source); return; },
            _       => {
                if self.is_playing() {
                    unsafe { ffi::alSourceStop(self.al_source) };
                    // wait a bit for openal terminate
                    sleep(50);
                }
                self.file.get_mut_ref().seek(0, SeekSet);
                self.process_music();
            }
        }

        
        
    }

    fn process_music(&mut self) -> () {
        let (port, chan) = stream();
        let sample_t_r = self.sample_to_read;
        let sample_rate = self.file_infos.samplerate;
        let sample_format = self.sample_format;
        let al_source = self.al_source;
        let al_buffers = self.al_buffers;

        // create buff
        let mut samples = vec::from_elem(sample_t_r as uint, 0i16);

        // full buff1
        let mut len = sys::size_of::<i16>() * self.file.get_mut_ref().read_i16(samples, sample_t_r as i64) as uint;
        al::alBufferData(al_buffers[0], sample_format, vec::raw::to_ptr(samples) as *c_void, len as i32, sample_rate);
          
        // full buff2
        samples.clear();
        len = sys::size_of::<i16>() * self.file.get_mut_ref().read_i16(samples, sample_t_r as i64) as uint;
        al::alBufferData(al_buffers[1], sample_format, vec::raw::to_ptr(samples) as *c_void, len as i32, sample_rate);

        // Queue the buffers
        al::alSourceQueueBuffers(al_source, 2, &al_buffers[0]);
       
        // Launche the sound
        al::alSourcePlay(al_source);

        do spawn {

            match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err);}
            };

            let mut file : ~SndFile = port.recv();
            let mut samples = vec::from_elem(sample_t_r as uint, 0i16);
            let mut status = ffi::AL_PLAYING;
            let mut i = 0;
            let mut buf = 0;
            let mut read;
            
            while status != ffi::AL_STOPPED {
                // wait a bit 
                sleep(50);
                if status == ffi::AL_PLAYING {
                   
                    
                    al::alGetSourcei(al_source, ffi::AL_BUFFERS_PROCESSED, &mut i);
                    if i != 0 {

                        samples.clear();
                        al::alSourceUnqueueBuffers(al_source, 1, &mut buf);
                        read = file.read_i16(samples, sample_t_r as i64) * sys::size_of::<i16>() as i64;
                        al::alBufferData(buf, sample_format, vec::raw::to_ptr(samples) as *c_void, read as i32, sample_rate);
                        al::alSourceQueueBuffers(al_source, 1, &buf);

                    }
                }
                // Get source status
                status = al::alGetState(al_source);
            }
            al::alSourcei(al_source, ffi::AL_BUFFER, 0);
        }
        
        let file = self.file.get_ref().clone();
        chan.send(file);
    }

    /**
    * Pause the Music.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn pause(&mut self) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };

        unsafe {
            ffi::alSourcePause(self.al_source)
        }
    }

    /**
    * Stop the Music.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn stop(&mut self) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };

        unsafe {
            ffi::alSourceStop(self.al_source)
        }
    }

    /**
    * Get the current state of the Sound
    *
    * # Return
    * The state of the sound as a variant of the enum State
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_state(&self) -> State {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return Initial; }
        };

        let mut state : i32 = 0;
        unsafe {
            ffi::alGetSourcei(self.al_source, ffi::AL_SOURCE_STATE, &mut state);
        }
        match state {
            ffi::AL_INITIAL     => Initial,
            ffi::AL_PLAYING     => Playing,
            ffi::AL_PAUSED      => Paused,
            ffi::AL_STOPPED     => Stopped,
            _                   => unreachable!()
        }
        
    }
 
    /**
    * Check if the Sound is playing or not.
    *
    * # Return
    * True if the Sound is playing, false otherwise.
    */
    pub fn is_playing(&self) -> bool {
        match self.get_state() {
            Playing     => true,
            _           => false
        }
    }
}

impl Drop for Music {
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) -> () {
        unsafe {
            al::alSourcei(self.al_source, ffi::AL_BUFFER, 0);
            ffi::alDeleteBuffers(2, &mut self.al_buffers[0]);
            ffi::alDeleteSources(1, &mut self.al_source);
            // self.file.take_unwrap().close();         
        }
    }
}