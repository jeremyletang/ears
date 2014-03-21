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
* Libsndfile is a library designed to allow the reading and writing of many
* different sampled sound file formats (such as MS Windows WAV and
* the Apple/SGI AIFF format) through one standard library interface.
*
* During read and write operations, formats are seamlessly converted between the
* format the application program has requested or supplied and the file's data
* format. The application programmer can remain blissfully unaware of issues
* such as file endian-ness and data format
*/

#![allow(dead_code)]

use std::{str, ptr};

#[doc(hidden)]
#[cfg(target_os="macos")]
#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod libsndfile {
    #[link(name = "sndfile")]
    extern {}
}

#[doc(hidden)]
#[path = "sndfile_ffi.rs"]
mod ffi;

/// The SndInfo structure is for passing data between the calling
/// function and the library when opening a file for reading or writing.
#[deriving(Clone)]
pub struct SndInfo {
    frames : i64,
    samplerate : i32,
    channels : i32,
    format : i32,
    sections : i32,
    seekable : i32
}

/// Modes availables for the open function.
///
/// * Read - Read only mode
/// * Write - Write only mode
/// * ReadWrite - Read and Write mode
pub enum OpenMode {
    Read    = ffi::SFM_READ as i32,
    Write   = ffi::SFM_WRITE as i32,
    ReadWrite    = ffi::SFM_RDWR as i32
}

/// Type of strings available for method get_string()
pub enum StringSoundType {
    Title       = ffi::SF_STR_TITLE as i32,
    Copyright   = ffi::SF_STR_COPYRIGHT as i32,
    Software    = ffi::SF_STR_SOFTWARE as i32,
    Artist      = ffi::SF_STR_ARTIST as i32,
    Comment     = ffi::SF_STR_COMMENT as i32,
    Date        = ffi::SF_STR_DATE as i32,
    Album       = ffi::SF_STR_ALBUM as i32,
    License     = ffi::SF_STR_LICENSE as i32,
    TrackNumber = ffi::SF_STR_TRACKNUMBER as i32,
    Genre       = ffi::SF_STR_GENRE as i32
}

/// Types of error who can be return by API functions
#[repr(C)]
pub enum Error {
    NoError             = ffi::SF_ERR_NO_ERROR as i32,
    UnrecognizedFormat  = ffi::SF_ERR_UNRECOGNISED_FORMAT as i32,
    SystemError         = ffi::SF_ERR_SYSTEM as i32,
    MalformedFile       = ffi::SF_ERR_MALFORMED_FILE as i32,
    UnsupportedEncoding = ffi::SF_ERR_UNSUPPORTED_ENCODING as i32,
}


/// Enum to set the offset with method seek
///
/// * SeekSet - The offset is set to the start of the audio data plus offset (multichannel) frames.
/// * SeekCur - The offset is set to its current location plus offset (multichannel) frames.
/// * SeekEnd - The offset is set to the end of the data plus offset (multichannel) frames.
pub enum SeekMode {
    SeekSet = ffi::SEEK_SET as i32,
    SeekCur = ffi::SEEK_CUR as i32,
    SeekEnd = ffi::SEEK_END as i32
}

/// Enum who contains the list of the supported audio format
///
/// * FormatWav - Microsoft WAV format (little endian)
/// * FormatAiff - Apple/SGI AIFF format (big endian)
/// * FormatAu - Sun/NeXT AU format (big endian)
/// * FormatRaw - RAW PCM data
/// * FormatPaf - Ensoniq PARIS file format
/// * FormatSvx - Amiga IFF / SVX8 / SV16 format
/// * FormatNist - Sphere NIST format
/// * FormatVoc - VOC files
/// * FormatIrcam - Berkeley/IRCAM/CARL
/// * FormatW64 - Sonic Foundry's 64 bit RIFF/WAV
/// * FormatMat4 - Matlab (tm) V4.2 / GNU Octave 2.0
/// * FormatMat5 - Matlab (tm) V5.0 / GNU Octave 2.1
/// * FormatPvf - Portable Voice Format
/// * FormatXi - Fasttracker 2 Extended Instrument
/// * FormatHtk - HMM Tool Kit format
/// * FormatSds - Midi Sample Dump Standard
/// * FormatAvr - Audio Visual Research
/// * FormatWavex - MS WAVE with WAVEFORMATEX
/// * FormatSd2 - Sound Designer 2
/// * FormatFlac - FLAC lossless file format
/// * FormatCaf - Core Audio File format
/// * FormatWve - Psion WVE format
/// * FormatOgg - Xiph OGG container
/// * FormatMpc2k - Akai MPC 2000 sampler
/// * FormatRf64 - RF64 WAV file
/// * FormatPcmS8 - Signed 8 bit data
/// * FormatPcm16 - Signed 16 bit data
/// * FormatPcm24 - Signed 24 bit data
/// * FormatPcm32 - Signed 32 bit data
/// * FormatPcmU8 - Unsigned 8 bit data (WAV and RAW only)
/// * FormatFloat - 32 bit float data
/// * FormatDouble - 64 bit float data
/// * FormatUlaw - U-Law encoded
/// * FormatAlaw - A-Law encoded
/// * FormatImaAdpcm - IMA ADPCM
/// * FormatApcm - Microsoft ADPCM
/// * FormatGsm610 - GSM 6.10 encoding
/// * FormatVoxAdpcm - Oki Dialogic ADPCM encoding
/// * FormatG72132 - 32kbs G721 ADPCM encoding
/// * FormatG72324 - 24kbs G723 ADPCM encoding
/// * FormatG72340 - 40kbs G723 ADPCM encoding
/// * FormatDww12 - 12 bit Delta Width Variable Word encoding
/// * FormatDww16 - 16 bit Delta Width Variable Word encoding
/// * FormatDww24 - 24 bit Delta Width Variable Word encoding
/// * FormatDwwN - N bit Delta Width Variable Word encoding
/// * FormatDpcm8 - 8 bit differential PCM (XI only)
/// * FormatDpcm16 - 16 bit differential PCM (XI only)
/// * FormatVorbis - Xiph Vorbis encoding
/// * EndianFile - Default file endian-ness
/// * EndianLittle - Force little endian-ness
/// * EndianBig - Force big endian-ness
/// * EndianCpu - Force CPU endian-ness
#[repr(C)]
#[deriving(Show, Clone, Ord, Eq)]
pub enum FormatType {
    FormatWav = ffi::SF_FORMAT_WAV as i32,
    FormatAiff = ffi::SF_FORMAT_AIFF as i32,
    FormatAu = ffi::SF_FORMAT_AU as i32,
    FormatRaw = ffi::SF_FORMAT_RAW as i32,
    FormatPaf = ffi::SF_FORMAT_PAF as i32,
    FormatSvx = ffi::SF_FORMAT_SVX as i32,
    FormatNist = ffi::SF_FORMAT_NIST as i32,
    FormatVoc = ffi::SF_FORMAT_VOC as i32,
    FormatIrcam = ffi::SF_FORMAT_IRCAM as i32,
    FormatW64 = ffi::SF_FORMAT_W64 as i32,
    FormatMat4 = ffi::SF_FORMAT_MAT4 as i32,
    FormatMat5 = ffi::SF_FORMAT_MAT5 as i32,
    FormatPvf = ffi::SF_FORMAT_PVF as i32,
    FormatXi = ffi::SF_FORMAT_XI as i32,
    FormatHtk = ffi::SF_FORMAT_HTK as i32,
    FormatSds = ffi::SF_FORMAT_SDS as i32,
    FormatAvr = ffi::SF_FORMAT_AVR as i32,
    FormatWavex = ffi::SF_FORMAT_WAVEX as i32,
    FormatSd2 = ffi::SF_FORMAT_SD2 as i32,
    FormatFlac = ffi::SF_FORMAT_FLAC as i32,
    FormatCaf = ffi::SF_FORMAT_CAF as i32,
    FormatWve = ffi::SF_FORMAT_WVE as i32,
    FormatOgg = ffi::SF_FORMAT_OGG as i32,
    FormatMpc2k = ffi::SF_FORMAT_MPC2K as i32,
    FormatRf64 = ffi::SF_FORMAT_RF64 as i32,
    FormatPcmS8 = ffi::SF_FORMAT_PCM_S8 as i32,
    FormatPcm16 = ffi::SF_FORMAT_PCM_16 as i32,
    FormatPcm24 = ffi::SF_FORMAT_PCM_24 as i32,
    FormatPcm32 = ffi::SF_FORMAT_PCM_32 as i32,
    FormatPcmU8 = ffi::SF_FORMAT_PCM_U8 as i32,
    FormatFloat = ffi::SF_FORMAT_FLOAT as i32,
    FormatDouble = ffi::SF_FORMAT_DOUBLE as i32,
    FormatUlaw = ffi::SF_FORMAT_ULAW as i32,
    FormatAlaw = ffi::SF_FORMAT_ALAW as i32,
    FormatImaAdpcm = ffi::SF_FORMAT_IMA_ADPCM as i32,
    FormatApcm = ffi::SF_FORMAT_MS_ADPCM  as i32,
    FormatGsm610 = ffi::SF_FORMAT_GSM610 as i32,
    FormatVoxAdpcm = ffi::SF_FORMAT_VOX_ADPCM as i32,
    FormatG72132 = ffi::SF_FORMAT_G721_32 as i32,
    FormatG72324 = ffi::SF_FORMAT_G723_24 as i32,
    FormatG72340 = ffi::SF_FORMAT_G723_40 as i32,
    FormatDww12 = ffi::SF_FORMAT_DWVW_12 as i32,
    FormatDww16 = ffi::SF_FORMAT_DWVW_16 as i32,
    FormatDww24 = ffi::SF_FORMAT_DWVW_24 as i32,
    FormatDwwN = ffi::SF_FORMAT_DWVW_N as i32,
    FormatDpcm8 = ffi::SF_FORMAT_DPCM_8 as i32,
    FormatDpcm16 = ffi::SF_FORMAT_DPCM_16 as i32,
    FormatVorbis = ffi::SF_FORMAT_VORBIS as i32,
    EndianFile = ffi::SF_ENDIAN_FILE as i32,
    EndianLittle = ffi::SF_ENDIAN_LITTLE as i32,
    EndianBig = ffi::SF_ENDIAN_BIG as i32,
    EndianCpu = ffi::SF_ENDIAN_CPU as i32,
    FormatSubMask = ffi::SF_FORMAT_SUBMASK as i32,
    FormatTypeMask = ffi::SF_FORMAT_TYPEMASK as i32,
}

impl BitOr<FormatType, i32> for FormatType {
    fn bitor(&self, _rhs: &FormatType) -> i32 {
        (*self as i32) | (*_rhs as i32)
    }
}

/// SndFile object, used to load/store sound from a file path or an fd.
pub struct SndFile {
    priv handle : *ffi::SNDFILE,
    priv info : ~SndInfo
}

impl Clone for SndFile {
    fn clone(&self) -> SndFile {
        SndFile {
            handle : self.handle,
            info : self.info.clone()
        }
    }
}

impl SndFile {
    /**
     * Construct SndFile object with the path to the music and a mode to open it.
     *
     * # Arguments
     * * path - The path to load the music
     * * mode - The mode to open the music
     *
     * Return Ok() containing the SndFile on success, a string representation of
     * the error otherwise.
     */
    pub fn new(path : &str, mode : OpenMode) -> Result<SndFile, ~str> {
        let info : ~SndInfo = ~SndInfo {
            frames : 0,
            samplerate : 0,
            channels : 0,
            format : 0,
            sections : 0,
            seekable : 0
        };
        let tmp_sndfile = path.with_c_str(|c_path| {
            unsafe {ffi::sf_open(c_path, mode as i32, &*info) }
        });
        if tmp_sndfile.is_null() {
            Err(unsafe { str::raw::from_c_str(ffi::sf_strerror(ptr::null())) })
        } else {
            Ok(SndFile {
                handle :    tmp_sndfile,
                info :      info
            })
        }
    }

    /**
     * Construct SndFile object with the path to the music and a mode to open it.
     *
     * # Arguments
     * * path - The path to load the music
     * * mode - The mode to open the music
     * * info - The SndInfo to pass to the file
     *
     * Return Ok() containing the SndFile on success, a string representation of
     * the error otherwise.
     */
    pub fn new_with_info(path : &str, mode : OpenMode, info: ~SndInfo) -> Result<SndFile, ~str> {
        let tmp_sndfile = path.with_c_str(|c_path| {
            unsafe {ffi::sf_open(c_path, mode as i32, &*info) }
        });
        if tmp_sndfile.is_null() {
            Err(unsafe { str::raw::from_c_str(ffi::sf_strerror(ptr::null())) })
        } else {
            Ok(SndFile {
                handle :    tmp_sndfile,
                info :      info
            })
        }
    }

    /**
     * Construct SndFile object with the fd of the file containing the music
     * and a mode to open it.
     *
     * # Arguments
     * * fd - The fd to load the music
     * * mode - The mode to open the music
     * * close_desc - Should SndFile close the fd at exit?
     *
     * Return Ok() containing the SndFile on success, a string representation
     * of the error otherwise.
     */
    pub fn new_with_fd(fd : i32,
                       mode : OpenMode,
                       close_desc : bool)
                       -> Result<SndFile, ~str> {
        let info : ~SndInfo = ~SndInfo {
            frames : 0,
            samplerate : 0,
            channels : 0,
            format : 0,
            sections : 0,
            seekable : 0
        };
        let tmp_sndfile = match close_desc {
            true    => unsafe {
                ffi::sf_open_fd(fd, mode as i32, &*info, ffi::SF_TRUE)
            },
            false   => unsafe {
                ffi::sf_open_fd(fd, mode as i32, &*info, ffi::SF_FALSE)
            }
        };
        if tmp_sndfile.is_null() {
            Err(unsafe { str::raw::from_c_str(ffi::sf_strerror(ptr::null())) })
        } else {
            Ok(SndFile {
                handle :    tmp_sndfile,
                info :      info
            })
        }
    }

    /// Return the SndInfo struct of the current music.
    pub fn get_sndinfo(&self) -> SndInfo {
        *self.info.clone()
    }

    /**
     * Retrieve a tag contained by the music.
     *
     * # Argument
     * * string_type - The type of the tag to retrieve
     *
     * Return Some() ~str if the tag is found, None otherwise.
     */
    pub fn get_string(&self, string_type : StringSoundType) -> Option<~str> {
        let c_string = unsafe {
            ffi::sf_get_string(self.handle, string_type as i32)
        };
        if c_string.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(c_string) })
        }
    }

    /**
     * Set a tag on the music file.
     *
     * # Arguments
     * * string_type - The type of the tag to set
     * * string - The string to set.
     *
     * Return NoError on success, an other error code otherwise
    */
    pub fn set_string(&mut self,
                      string_type : StringSoundType,
                      string : ~str) -> Error {
        unsafe {
            ffi::sf_set_string(self.handle,
                               string_type as i32,
                               string.to_c_str().unwrap())
        }
    }

    /**
     * Check if the format of the SndInfo struct is valid.
     *
     * # Argument
     * * info - The SndInfo struct to test
     *
     * Return true if the struct is valid, false otherwise.
     */
    pub fn check_format<'r>(info : &'r SndInfo) -> bool {
        match unsafe {ffi::sf_format_check(info) } {
            ffi::SF_TRUE    => true,
            ffi::SF_FALSE   => false,
            _               => unreachable!()
        }
    }


    /**
     * Close the SndFile object.
     *
     * This function must be called before the exist of the program to destroy
     * all the resources.
     *
     * Return NoError if destruction success, an other error code otherwise.
     */
    pub fn close(&self) -> Error {
        unsafe {
            ffi::sf_close(self.handle)
        }
    }

    /**
     * If the file is opened Write or ReadWrite, call the operating system's
     * function to force the writing of all file cache buffers to disk.
     * If the file is opened Read no action is taken.
     */
    pub fn write_sync(&mut self) -> () {
        unsafe {
            ffi::sf_write_sync(self.handle)
        }
    }

    pub fn seek(&mut self, frames : i64, whence : SeekMode) -> i64{
        unsafe {
            ffi::sf_seek(self.handle, frames, whence as i32)
        }
    }

    /**
     * Read items of type i16
     *
     * # Arguments
     * * array - The array to fill with the items.
     * * items - The max capacity of the array.
     *
     * Return the count of items.
     */
    pub fn read_i16<'r>(&'r mut self, array : &'r mut [i16], items : i64) -> i64 {
        unsafe {
            ffi::sf_read_short(self.handle, array.as_mut_ptr(), items)
        }
    }

    /**
     * Read items of type i32
     *
     * # Arguments
     * * array - The array to fill with the items.
     * * items - The max capacity of the array.
     *
     * Return the count of items.
     */
    pub fn read_i32<'r>(&'r mut self, array : &'r mut [i32], items : i64) -> i64 {
        unsafe {
            ffi::sf_read_int(self.handle, array.as_mut_ptr(), items)
        }
    }

    /**
     * Read items of type f32
     *
     * # Arguments
     * * array - The array to fill with the items.
     * * items - The max capacity of the array.
     *
     * Return the count of items.
     */
    pub fn read_f32<'r>(&'r mut self, array : &'r mut [f32], items : i64) -> i64 {
        unsafe {
            ffi::sf_read_float(self.handle, array.as_mut_ptr(), items)
        }
    }

    /**
     * Read items of type f64
     *
     * # Arguments
     * * array - The array to fill with the items.
     * * items - The max capacity of the array.
     *
     * Return the count of items.
     */
    pub fn read_f64<'r>(&'r mut self, array : &'r mut [f64], items : i64) -> i64 {
        unsafe {
            ffi::sf_read_double(self.handle, array.as_mut_ptr(), items)
        }
    }

    /**
     * Read frames of type i16
     *
     * # Arguments
     * * array - The array to fill with the frames.
     * * items - The max capacity of the array.
     *
     * Return the count of frames.
     */
    pub fn readf_i16<'r>(&'r mut self, array : &'r mut [i16], frames : i64) -> i64 {
        unsafe {
            ffi::sf_readf_short(self.handle, array.as_mut_ptr(), frames)
        }
    }

    /**
     * Read frames of type i32
     *
     * # Arguments
     * * array - The array to fill with the frames.
     * * items - The max capacity of the array.
     *
     * Return the count of frames.
     */
    pub fn readf_i32<'r>(&'r mut self, array : &'r mut [i32], frames : i64) -> i64 {
        unsafe {
            ffi::sf_readf_int(self.handle, array.as_mut_ptr(), frames)
        }
    }

    /**
     * Read frames of type f32
     *
     * # Arguments
     * * array - The array to fill with the frames.
     * * items - The max capacity of the array.
     *
     * Return the count of frames.
     */
    pub fn readf_f32<'r>(&'r mut self, array : &'r mut [f32], frames : i64) -> i64 {
        unsafe {
            ffi::sf_readf_float(self.handle, array.as_mut_ptr(), frames)
        }
    }

    /**
     * Read frames of type f64
     *
     * # Arguments
     * * array - The array to fill with the frames.
     * * items - The max capacity of the array.
     *
     * Return the count of frames.
     */
    pub fn readf_f64<'r>(&'r mut self, array : &'r mut [f64], frames : i64) -> i64 {
        unsafe {
            ffi::sf_readf_double(self.handle, array.as_mut_ptr(), frames)
        }
    }

    /**
     * Write items of type i16
     *
     * # Arguments
     * * array - The array of items to write.
     * * items - The number of items to write.
     *
     * Return the count of wrote items.
     */
    pub fn write_i16<'r>(&'r mut self, array : &'r mut [i16], items : i64) -> i64 {
        unsafe {
            ffi::sf_write_short(self.handle, array.as_mut_ptr(), items)
        }
    }

    /**
     * Write items of type i32
     *
     * # Arguments
     * * array - The array of items to write.
     * * items - The number of items to write.
     *
     * Return the count of wrote items.
     */
    pub fn write_i32<'r>(&'r mut self, array : &'r mut [i32], items : i64) -> i64 {
        unsafe {
            ffi::sf_write_int(self.handle, array.as_mut_ptr(), items)
        }
    }

    /**
     * Write items of type f32
     *
     * # Arguments
     * * array - The array of items to write.
     * * items - The number of items to write.
     *
     * Return the count of wrote items.
     */
    pub fn write_f32<'r>(&'r mut self, array : &'r mut [f32], items : i64) -> i64 {
        unsafe {
            ffi::sf_write_float(self.handle, array.as_mut_ptr(), items)
        }
    }

    /**
     * Write items of type f64
     *
     * # Arguments
     * * array - The array of items to write.
     * * items - The number of items to write.
     *
     * Return the count of wrote items.
     */
    pub fn write_f64<'r>(&'r mut self, array : &'r mut [f64], items : i64) -> i64 {
        unsafe {
            ffi::sf_write_double(self.handle, array.as_mut_ptr(), items)
        }
    }

    /**
     * Write frames of type i16
     *
     * # Arguments
     * * array - The array of frames to write.
     * * items - The number of frames to write.
     *
     * Return the count of wrote frames.
     */
    pub fn writef_i16<'r>(&'r mut self, array : &'r mut [i16], frames : i64) -> i64 {
        unsafe {
            ffi::sf_writef_short(self.handle, array.as_mut_ptr(), frames)
        }
    }

    /**
     * Write frames of type i32
     *
     * # Arguments
     * * array - The array of frames to write.
     * * items - The number of frames to write.
     *
     * Return the count of wrote frames.
     */
    pub fn writef_i32<'r>(&'r mut self, array : &'r mut [i32], frames : i64) -> i64 {
        unsafe {
            ffi::sf_writef_int(self.handle, array.as_mut_ptr(), frames)
        }
    }

    /**
     * Write frames of type f32
     *
     * # Arguments
     * * array - The array of frames to write.
     * * items - The number of frames to write.
     *
     * Return the count of wrote frames.
     */
    pub fn writef_f32<'r>(&'r mut self, array : &'r mut [f32], frames : i64) -> i64 {
        unsafe {
            ffi::sf_writef_float(self.handle, array.as_mut_ptr(), frames)
        }
    }

    /**
     * Write frames of type f64
     *
     * # Arguments
     * * array - The array of frames to write.
     * * items - The number of frames to write.
     *
     * Return the count of wrote frames.
     */
    pub fn writef_f64<'r>(&'r mut self, array : &'r mut [f64], frames : i64) -> i64 {
        unsafe {
            ffi::sf_writef_double(self.handle, array.as_mut_ptr(), frames)
        }
    }

    /**
     * Get the last error
     *
     * Return the last error as a variant of the enum Error.
     */
    pub fn error(&self) -> Error {
        unsafe {
            ffi::sf_error(self.handle)
        }
    }

    /**
     * Get the last error as a string
     *
     * Return an owned str containing the last error.
     */
    pub fn string_error(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(ffi::sf_strerror(self.handle))
        }
    }

    /**
     * Get an error as a string from a variant of enum Error
     *
     * Return an owned str containing the error.
     */
    pub fn error_number(error_num : Error) -> ~str {
        unsafe {
            str::raw::from_c_str(ffi::sf_error_number(error_num as i32))
        }
    }

}

