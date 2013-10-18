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
* They are not associated to a MusicData like Musics.
*
* # Examples 
* ```
* extern mod ears;
* use ears::Music;
*
* fn main() -> () {
*    // Load a Music
*   let msc = Music::new(~"path/to/my/Music.flac").unwrap();
*
*   // Play it
*   msc.play();
* }
* ```
*/

use std::rt::io::timer::sleep;
use std::{vec, mem};
use std::libc::c_void;
use std::task::*;

use internal::*;
use openal::{ffi, al};
use sndfile::*;
use states::*;
use audio_controller::AudioController;
use audio_tags::*;

/// Class for play Musics
pub struct Music {
    /// The internal OpenAL source identifier
    priv al_source      : u32,
    /// The internal OpenAL buffers
    priv al_buffers     : [u32, ..2],
    /// The file open with libmscfile
    priv file           : Option<~SndFile>,
    /// Information of the file
    priv file_infos     : SndInfo,
    /// Quantity of sample to read each time
    priv sample_to_read : i32,
    /// Format of the sample
    priv sample_format  : i32,
    /// Audio tags
    priv sound_tags     : Tags
}

impl Music {
    /**
    * Create a new Music
    *
    * # Argument
    * * `path` - The path of the file to load the music
    *
    * # Return
    * An Option containing Some(Music) on success, None otherwise
    */
    pub fn new(path : &str) -> Option<Music> {
        // Check that OpenAL is launched
        check_openal_context!(None);
        // Retrieve File and Music datas
        let file = match SndFile::new(path, Read) {
            Ok(file)    => ~file,
            Err(err)    => { println!("{}", err); return None; }
        };
        let infos = file.get_sndinfo();

        // create the source and the buffers
        let mut source_id = 0;
        let mut buffer_ids = [0, ..2];
        // create the source
        al::alGenSources(1, &mut source_id);
        // create the buffers
        al::alGenBuffers(2, &mut buffer_ids[0]);

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

        let sound_tags = get_sound_tags(file);

        Some( Music {
            al_source       : source_id,
            al_buffers      : buffer_ids,
            file            : Some(file),
            file_infos      : infos,
            sample_to_read  : 50000,
            sample_format   : format,
            sound_tags      : sound_tags
        })        
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
        let mut len = mem::size_of::<i16>() * self.file.get_mut_ref().read_i16(samples, sample_t_r as i64) as uint;
        al::alBufferData(al_buffers[0], sample_format, vec::raw::to_ptr(samples) as *c_void, len as i32, sample_rate);
          
        // full buff2
        samples.clear();
        len = mem::size_of::<i16>() * self.file.get_mut_ref().read_i16(samples, sample_t_r as i64) as uint;
        al::alBufferData(al_buffers[1], sample_format, vec::raw::to_ptr(samples) as *c_void, len as i32, sample_rate);

        // Queue the buffers
        al::alSourceQueueBuffers(al_source, 2, &al_buffers[0]);
       
        // Launche the Music
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
                        read = file.read_i16(samples, sample_t_r as i64) * mem::size_of::<i16>() as i64;
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

}

impl AudioTags for Music {
    /**
    * Get the tags of a Sound.
    *
    * # Return
    * A borrowed pointer to the internal struct SoundTags
    */
    fn get_tags<'r>(&'r self) -> &'r Tags {
        &self.sound_tags
    }
}

impl AudioController for Music {
    /**
    * Play or resume the Music.
    */
    fn play(&mut self) -> () {
        check_openal_context!(());

        match self.get_state() {
            Paused   => { al::alSourcePlay(self.al_source); return; },
            _       => {
                if self.is_playing() {
                    al::alSourceStop(self.al_source);
                    
                    // wait a bit for openal terminate
                    sleep(50);
                }
                self.file.get_mut_ref().seek(0, SeekSet);
                self.process_music();
            }
        }
    }

    /**
    * Pause the Music.
    */
    fn pause(&mut self) -> () {
        check_openal_context!(());

        al::alSourcePause(self.al_source)
    }

    /**
    * Stop the Music.
    */
    fn stop(&mut self) -> () {
        check_openal_context!(());

        al::alSourceStop(self.al_source);
        sleep(50)
    }

    /**
    * Check if the Music is playing or not.
    *
    * # Return
    * True if the Music is playing, false otherwise.
    */
    fn is_playing(&self) -> bool {
        match self.get_state() {
            Playing     => true,
            _           => false
        }
    }

    /**
    * Get the current state of the Music
    *
    * # Return
    * The state of the music as a variant of the enum State
    */
    fn get_state(&self) -> State {
        check_openal_context!(Initial);

        let state  = al::alGetState(self.al_source);

        match state {
            ffi::AL_INITIAL     => Initial,
            ffi::AL_PLAYING     => Playing,
            ffi::AL_PAUSED      => Paused,
            ffi::AL_STOPPED     => Stopped,
            _                   => unreachable!()
        }
        
    }

    /**
    * Set the volume of the Music.
    *
    * A value of 1.0 means unattenuated. Each division by 2 equals an attenuation
    * of about -6dB. Each multiplicaton by 2 equals an amplification of about
    * +6dB.
    *
    * # Argument
    * * `volume` - The volume of the Music, should be between 0. and 1. 
    */
    fn set_volume(&mut self, volume : f32) -> () {
        check_openal_context!(());

        al::alSourcef(self.al_source, ffi::AL_GAIN, volume);
    }

    /**
    * Get the volume of the Music.
    *
    * # Return
    * The volume of the Music between 0. and 1.
    */
    fn get_volume(&self) -> f32 {
        check_openal_context!(0.);
    
        let mut volume : f32 = 0.;
        al::alGetSourcef(self.al_source, ffi::AL_GAIN, &mut volume);
        volume
    }

    /**
    * Set the minimal volume for a Music.
    *
    * The minimum volume allowed for a music, after distance and cone attenation is
    * applied (if applicable).
    *
    * # Argument
    * * `min_volume` - The new minimal volume of the Music should be between 0. and 1. 
    */
    fn set_min_volume(&mut self, min_volume : f32) -> () {
        check_openal_context!(());
        
        al::alSourcef(self.al_source, ffi::AL_MIN_GAIN, min_volume);
    }

    /**
    * Get the minimal volume of the Music.
    *
    * # Return
    * The minimal volume of the Music between 0. and 1.
    */
    fn get_min_volume(&self) -> f32 {
        check_openal_context!(0.);
        
        let mut volume : f32 = 0.;
        al::alGetSourcef(self.al_source, ffi::AL_MIN_GAIN, &mut volume);
        volume
    }

    /**
    * Set the maximal volume for a Music.
    *
    * The maximum volume allowed for a Music, after distance and cone attenation is
    * applied (if applicable).
    *
    * # Argument
    * * `max_volume` - The new maximal volume of the Music should be between 0. and 1. 
    */
    fn set_max_volume(&mut self, max_volume : f32) -> () {
        check_openal_context!(());
        
        al::alSourcef(self.al_source, ffi::AL_MAX_GAIN, max_volume);
    }

    /**
    * Get the maximal volume of the Music.
    *
    * # Return
    * The maximal volume of the Music between 0. and 1.
    */
    fn get_max_volume(&self) -> f32 {
        check_openal_context!(0.);
   
        let mut volume : f32 = 0.;
        al::alGetSourcef(self.al_source, ffi::AL_MAX_GAIN, &mut volume);
        volume
    }

    /**
    * Set the Music looping or not
    *
    * The default looping is false.
    *
    * # Arguments
    * `looping` - The new looping state.
    */
    fn set_looping(&mut self, looping : bool) -> () {
        check_openal_context!(());

        match looping {
            true    => al::alSourcei(self.al_source, ffi::AL_LOOPING, ffi::ALC_TRUE as i32),
            false   => al::alSourcei(self.al_source, ffi::AL_LOOPING, ffi::ALC_FALSE as i32)
        };
    }

    /**
    * Check if the Music is looping or not
    *
    * # Return
    * True if the Music is looping, false otherwise.
    */
    fn is_looping(&self) -> bool {
        check_openal_context!(false);
      
        let mut boolean = 0; 
        al::alGetSourcei(self.al_source, ffi::AL_LOOPING, &mut boolean);
        match boolean as i8 {
            ffi::ALC_TRUE       => true,
            ffi::ALC_FALSE      => false,
            _                   => unreachable!()
        }
    }

    /**
    * Set the pitch of the Music.
    * 
    * A multiplier for the frequency (sample rate) of the Music's buffer.
    *
    * Default pitch is 1.0.
    * 
    * # Argument
    * * `new_pitch` - The new pitch of the Music in the range [0.5 - 2.0]
    */
    fn set_pitch(&mut self, pitch : f32) -> () {
        check_openal_context!(());

        al::alSourcef(self.al_source, ffi::AL_PITCH, pitch)
    }

    /**
    * Set the pitch of the Music.
    * 
    * # Return
    * The pitch of the Music in the range [0.5 - 2.0]
    */
    fn get_pitch(&self) -> f32 {
        check_openal_context!(0.);

        let mut pitch = 0.;
        al::alGetSourcef(self.al_source, ffi::AL_PITCH, &mut pitch);
        pitch
    }

    /**
    * Set the position of the Music relative to the listener or absolute.
    *
    * Default position is absolute.
    *
    * # Argument
    * `relative` - True to set Music relative to the listener false to set the Music position absolute.
    */
    fn set_relative(&mut self, relative : bool) -> () {
        check_openal_context!(());

        match relative {
            true    => al::alSourcei(self.al_source, ffi::AL_SOURCE_RELATIVE, ffi::ALC_TRUE as i32),
            false   => al::alSourcei(self.al_source, ffi::AL_SOURCE_RELATIVE, ffi::ALC_FALSE as i32)
        };
    }

    /**
    * Is the Music relative to the listener or not ?
    *
    * # Return
    * True if the Music is relative to the listener false otherwise
    */
    fn is_relative(&mut self) -> bool {
        check_openal_context!(false);

        let mut boolean = 0; 
        al::alGetSourcei(self.al_source, ffi::AL_SOURCE_RELATIVE, &mut boolean);
        match boolean as i8 {
            ffi::ALC_TRUE       => true,
            ffi::ALC_FALSE      => false,
            _                   => unreachable!()
        }
    }

    /**
    * Set the Music location in three dimensional space.
    *
    * OpenAL, like OpenGL, uses a right handed coordinate system, where in a
    * frontal default view X (thumb) points right, Y points up (index finger), and
    * Z points towards the viewer/camera (middle finger). 
    * To switch from a left handed coordinate system, flip the sign on the Z
    * coordinate.
    *
    * Default position is [0., 0., 0.]. 
    *
    * # Argument
    * * `position` - A three dimensional vector of f32 containing the position of the listener [x, y, z].
    */
    fn set_position(&mut self, position : [f32, ..3]) -> () {
        check_openal_context!(());
     
        al::alSourcefv(self.al_source, ffi::AL_POSITION, &position[0]);
    }

    /**
    * Get the position of the Music in three dimensional space.
    *
    * # Return
    * A three dimensional vector of f32 containing the position of the listener [x, y, z].
    */
    fn get_position(&self) -> [f32, ..3] {
        check_openal_context!([0., ..3]);
      
        let mut position : [f32, ..3] = [0., ..3];
        al::alGetSourcefv(self.al_source, ffi::AL_POSITION, &mut position[0]);
        position
    }

    /**
    * Set the direction of the Music.
    *
    * Specifies the current direction in local space.
    *
    * The default direction is: [0., 0., 0.]
    *
    * # Argument
    * `direction` - The new direction of the Music.
    */
    fn set_direction(&mut self, direction : [f32, ..3]) -> () {
        check_openal_context!(());

        al::alSourcefv(self.al_source, ffi::AL_DIRECTION, &direction[0]);
    }

    /**
    * Get the direction of the Music.
    *
    * # Return
    * The current direction of the Music.
    */
    fn get_direction(&self)  -> [f32, ..3] {
        check_openal_context!([0., ..3]);
       
        let mut direction : [f32, ..3] = [0., ..3];
        al::alGetSourcefv(self.al_source, ffi::AL_DIRECTION, &mut direction[0]);
        direction
    }

    /**
    * Set the maximum distance of the Music.
    *
    * The distance above which the source is not attenuated any further with a
    * clamped distance model, or where attenuation reaches 0.0 gain for linear
    * distance models with a default rolloff factor.
    * 
    * The default maximum distance is +inf.
    *
    * # Argument
    * `max_distance` - The new maximum distance in the range [0., +inf]
    */
    fn set_max_distance(&mut self, max_distance : f32) -> () {
        check_openal_context!(());
       
        al::alSourcef(self.al_source, ffi::AL_MAX_DISTANCE, max_distance);
    }

    /**
    * Get the maximum distance of the Music.
    *
    * # Return
    * The maximum distance of the Music in the range [0., +inf]
    */
    fn get_max_distance(&self) -> f32 {
        check_openal_context!(0.);
        
        let mut max_distance = 0.;
        al::alGetSourcef(self.al_source, ffi::AL_MAX_DISTANCE, &mut max_distance);
        max_distance
    }

    /**
    * Set the reference distance of the Music.
    *
    * The distance in units that no attenuation occurs.
    * At 0.0, no distance attenuation ever occurs on non-linear attenuation models.
    *
    * The default distance reference is 1.
    *
    * # Argument
    * * `ref_distance` - The new reference distance of the Music.
    */
    fn set_reference_distance(&mut self, ref_distance : f32) -> () {
        check_openal_context!(());
     
        al::alSourcef(self.al_source, ffi::AL_REFERENCE_DISTANCE, ref_distance);
    }

    /**
    * Get the reference distance of the Music.
    *
    * # Return
    * The current reference distance of the Music.
    */
    fn get_reference_distance(&self) -> f32 {
        check_openal_context!(1.);

        let mut ref_distance = 0.;
        al::alGetSourcef(self.al_source, ffi::AL_REFERENCE_DISTANCE, &mut ref_distance);
        ref_distance
    }

    /**
    * Set the attenuation of a Music.
    *
    * Multiplier to exaggerate or diminish distance attenuation.
    * At 0.0, no distance attenuation ever occurs.
    *
    * The default attenuation is 1.
    *
    * # Arguments
    * `attenuation` - The new attenuation for the Music in the range [0., 1.].
    */
    fn set_attenuation(&mut self, attenuation : f32) -> () {
        check_openal_context!(());
       
        al::alSourcef(self.al_source, ffi::AL_ROLLOFF_FACTOR, attenuation);
    }

    /**
    * Get the attenuation of a Music.
    *
    * # Return
    * The current attenuation for the Music in the range [0., 1.].
    */
    fn get_attenuation(&self) -> f32 {
        check_openal_context!(1.);
       
        let mut attenuation = 0.;
        al::alGetSourcef(self.al_source, ffi::AL_ROLLOFF_FACTOR, &mut attenuation);
        attenuation
    } 

}


impl Drop for Music {
    /**
    * Destroy all the resources of the Music.
    */
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

#[cfg(test)]
mod test {
    use super::*;
    use states::*;

    #[test]
    fn music_create_OK() -> () {
        let msc = Music::new("shot.wav");

        match msc {
            Some(_) => {},
            None    => fail!()
        }
    }

    #[test]
    fn music_create_FAIL() -> () {
        let msc = Music::new("toto.wav");

        match msc {
            Some(_) => fail!(),
            None    => {}
        }
    }

    // TODO fix this
    // #[test]
    // fn music_play_OK() -> () {
    //     let mut msc = Music::new("shot.wav").expect("Cannot create Music");

    //     msc.play();
    //     assert_eq!(msc.get_state() as i32, Playing as i32);
    //     msc.stop();
    // }

    // TODO fix this
    // #[test]
    // fn music_pause_OK() -> () {
    //     let mut msc = Music::new("shot.wav").expect("Cannot create Music");

    //     msc.play();
    //     msc.pause();
    //     assert_eq!(msc.get_state() as i32, Paused as i32);
    //     msc.stop();
    // }

    #[test]
    fn music_stop_OK() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.play();
        msc.stop();
        assert_eq!(msc.get_state() as i32, Stopped as i32);
        msc.stop();
    }


    // TODO fix this
    // #[test]
    // fn music_is_playing_TRUE() -> () {
    //     let mut msc = Music::new("shot.wav").expect("Cannot create Music");
        
    //     msc.play();
    //     assert_eq!(msc.is_playing(), true);
    //     msc.stop();
    // }

    // TODO fix this
    // #[test]
    // fn music_is_playing_FALSE() -> () {
    //     let mut msc = Music::new("shot.wav").expect("Cannot create Music");
        
    //     assert_eq!(msc.is_playing(), false);
    //     msc.stop();
    // }

    #[test]
    fn music_set_volume_OK() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_volume(0.7);        
        assert_eq!(msc.get_volume(), 0.7);
    }

    #[test]
    fn music_set_min_volume_OK() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_min_volume(0.1);        
        assert_eq!(msc.get_min_volume(), 0.1);
    }

    #[test]
    fn music_set_max_volume_OK() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_max_volume(0.9);      
        assert_eq!(msc.get_max_volume(), 0.9);
    }

    #[test]
    fn music_is_looping_TRUE() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_looping(true);
        assert_eq!(msc.is_looping(), true);
    }

    #[test]
    fn music_is_looping_FALSE() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_looping(false);      
        assert_eq!(msc.is_looping(), false);
    }

    #[test]
    fn music_set_pitch_OK() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_pitch(1.5);
        assert_eq!(msc.get_pitch(), 1.5);
    }

     #[test]
    fn music_set_relative_TRUE() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_relative(true);
        assert_eq!(msc.is_relative(), true);
    }

    #[test]
    fn music_set_relative_FALSE() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_relative(false);      
        assert_eq!(msc.is_relative(), false);
    }

    #[test]
    fn music_set_position_OK() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_position([50., 150., 250.]);      
        assert_eq!(msc.get_position(), [50., 150., 250.]);
    }

    #[test]
    fn music_set_direction_OK() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_direction([50., 150., 250.]);      
        assert_eq!(msc.get_direction(), [50., 150., 250.]);
    }

    #[test]
    fn music_set_max_distance() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_max_distance(70.);      
        assert_eq!(msc.get_max_distance(), 70.);
    }

    #[test]
    fn music_set_reference_distance() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_reference_distance(70.);      
        assert_eq!(msc.get_reference_distance(), 70.);
    }

    #[test]
    fn music_set_attenuation() -> () {
        let mut msc = Music::new("shot.wav").expect("Cannot create Music");

        msc.set_attenuation(70.);      
        assert_eq!(msc.get_attenuation(), 70.);
    }
}
