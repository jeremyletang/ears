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
* Class for play Sounds.
*
* Simple class to play sound easily in 2 lines.
*
* # Examples 
* ```
* let snd = Sound::new(~"path/to/my/sound.ogg").unwrap();
* snd.play();
* ```
*/

use internal::*;
use sound_data::*;
use openal::ffi;
use states::*;

/// Class for play sounds
pub struct Sound {
    priv al_source : u32,
    priv sound_data : @SoundData
}

impl Sound {
    /**
    * Default constructor for Sound struct.
    *
    * Create a new struct and an associated SoundData.
    * 
    * # Argument
    * `path` - The path of the sound file to create the SoundData.
    *
    * # Return
    * An Option with Some(Sound) if the Sound is created properly, or None if un error has occured.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new(path : ~str) -> Option<Sound> {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return None; }
        };

        let s_data = match SoundData::new(path) {
            Some(s_d)   => @s_d,
            None        => return None
        };

        Sound::new_with_data(s_data)
    }

    /**
    * Create a new struct with a SoundData to associate.
    * 
    * # Argument
    * `sound_data` - The sound_data to associate to the Sound.
    *
    * # Return
    * An Option with Some(Sound) if the Sound is created properly, or None if un error has occured.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new_with_data(sound_data : @SoundData) -> Option<Sound> {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return None; }
        };

        let mut source_id = 0;
        unsafe {
            // create the source
            ffi::alGenSources(1, &mut source_id);
            // set the buffer
            ffi::alSourcei(source_id, ffi::AL_BUFFER, sound_data.get_buffer() as i32);
        }

        let snd = Sound {
            al_source : source_id,
            sound_data : sound_data
        };
        
        Some(snd)
    }

    /**
    * Get the sound datas.
    *
    * # Return
    * The SoundData associated to this Sound.
    */
    pub fn get_datas(&self) -> @SoundData {
        self.sound_data
    }

    /**
    * Play the Sound.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn play(&mut self) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };

        unsafe {
             ffi::alSourcePlay(self.al_source)
        }
    }

    /**
    * Pause the Sound.
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
    * Stop the Sound.
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
    * Get the state of the Sound
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
    * Set the pitch of the source.
    * 
    * A multiplier for the frequency (sample rate) of the source's buffer.
    * Default pitch is 1.0.
    * 
    * # Argument
    * * `new_pitch` - The new pitch of the sound in the range [0.5 - 2.0]
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_pitch(&mut self, new_pitch : f32) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };

        unsafe {
            ffi::alSourcef(self.al_source, ffi::AL_PITCH, new_pitch)
        }
    }

    /**
    * Set the pitch of the source.
    * 
    * # Return
    * The pitch of the sound in the range [0.5 - 2.0]
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_pitch(&self) -> f32 {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return 0.; }
        };

        let mut pitch = 0.;
        unsafe {
            ffi::alGetSourcef(self.al_source, ffi::AL_PITCH, &mut pitch)
        }
        pitch
    }

    /**
    * Set the position of the sound relative to the listener or absolute.
    *
    * Default position is absolute.
    *
    * # Argument
    * `relative` - True to set sound relative to the listener false to set the sound position absolute.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_relative(&mut self, relative : bool) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };
        unsafe {
            match relative {
                true    => ffi::alSourcei(self.al_source, ffi::AL_SOURCE_RELATIVE, ffi::ALC_TRUE as i32),
                false   => ffi::alSourcei(self.al_source, ffi::AL_SOURCE_RELATIVE, ffi::ALC_FALSE as i32)
            };
        }
    }

    /**
    * Is the sound relative to the listener or not ?
    *
    * # Return
    * True if the sound is relative to the listener false otherwise
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_relative(&mut self) -> bool {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return false; }
        };

        let mut boolean = 0; 
        unsafe {
            ffi::alGetSourcei(self.al_source, ffi::AL_SOURCE_RELATIVE, &mut boolean);
        }
        match boolean as i8 {
            ffi::ALC_TRUE       => true,
            ffi::ALC_FALSE      => false,
            _                   => unreachable!()
        }
    }
}

#[unsafe_destructor]
impl Drop for Sound {
    /**
    * Destroy all the resources attached to the Sound.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) -> () {
        unsafe {
            ffi::alDeleteSources(1, &mut self.al_source);            
        }
    }
}