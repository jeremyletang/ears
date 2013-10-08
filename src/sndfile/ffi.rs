
use super::*;
use std::libc::{c_char, c_void};

pub type SF_MODE = i32;
pub static SFM_READ : SF_MODE   = 0x10;
pub static SFM_WRITE : SF_MODE  = 0x20;
pub static SFM_RDWR : SF_MODE   = 0x30;

pub type SF_ERR = i32;
pub static SF_ERR_NO_ERROR : SF_ERR             = 0;
pub static SF_ERR_UNRECOGNISED_FORMAT : SF_ERR  = 1;
pub static SF_ERR_SYSTEM : SF_ERR               = 2;
pub static SF_ERR_MALFORMED_FILE : SF_ERR       = 3;
pub static SF_ERR_UNSUPPORTED_ENCODING : SF_ERR = 4;

pub type SF_STR = i32;
pub static SF_STR_TITLE : SF_STR        = 0x01;
pub static SF_STR_COPYRIGHT : SF_STR    = 0x02;
pub static SF_STR_SOFTWARE : SF_STR     = 0x03;
pub static SF_STR_ARTIST : SF_STR       = 0x04;
pub static SF_STR_COMMENT : SF_STR      = 0x05;
pub static SF_STR_DATE : SF_STR         = 0x06;
pub static SF_STR_ALBUM : SF_STR        = 0x07;
pub static SF_STR_LICENSE : SF_STR      = 0x08;
pub static SF_STR_TRACKNUMBER : SF_STR  = 0x09;
pub static SF_STR_GENRE : SF_STR        = 0x10;

pub type SF_BOOL = i32;
pub static SF_FALSE : SF_BOOL   = 0;
pub static SF_TRUE : SF_BOOL    = 1;

pub type SEEK_MODE = i32;
pub static SEEK_SET : SEEK_MODE = 0;
pub static SEEK_CUR : SEEK_MODE = 1;
pub static SEEK_END : SEEK_MODE = 2;

pub type FORMAT_TYPE = i32;
pub static SF_FORMAT_WAV : FORMAT_TYPE          = 0x010000;   /// Microsoft WAV format (little endian)
pub static SF_FORMAT_AIFF : FORMAT_TYPE         = 0x020000;   /// Apple/SGI AIFF format (big endian)
pub static SF_FORMAT_AU : FORMAT_TYPE           = 0x030000;   /// Sun/NeXT AU format (big endian)
pub static SF_FORMAT_RAW : FORMAT_TYPE          = 0x040000;   /// RAW PCM data
pub static SF_FORMAT_PAF : FORMAT_TYPE          = 0x050000;   /// Ensoniq PARIS file format
pub static SF_FORMAT_SVX : FORMAT_TYPE          = 0x060000;   /// Amiga IFF / SVX8 / SV16 format
pub static SF_FORMAT_NIST : FORMAT_TYPE         = 0x070000;   /// Sphere NIST format
pub static SF_FORMAT_VOC : FORMAT_TYPE          = 0x080000;   /// VOC files
pub static SF_FORMAT_IRCAM : FORMAT_TYPE        = 0x0A0000;   /// Berkeley/IRCAM/CARL
pub static SF_FORMAT_W64 : FORMAT_TYPE          = 0x0B0000;   /// Sonic Foundry's 64 bit RIFF/WAV
pub static SF_FORMAT_MAT4 : FORMAT_TYPE         = 0x0C0000;   /// Matlab (tm) V4.2 / GNU Octave 2.0
pub static SF_FORMAT_MAT5 : FORMAT_TYPE         = 0x0D0000;   /// Matlab (tm) V5.0 / GNU Octave 2.1
pub static SF_FORMAT_PVF : FORMAT_TYPE          = 0x0E0000;   /// Portable Voice Format
pub static SF_FORMAT_XI : FORMAT_TYPE           = 0x0F0000;   /// Fasttracker 2 Extended Instrument
pub static SF_FORMAT_HTK : FORMAT_TYPE          = 0x100000;   /// HMM Tool Kit format
pub static SF_FORMAT_SDS : FORMAT_TYPE          = 0x110000;   /// Midi Sample Dump Standard
pub static SF_FORMAT_AVR : FORMAT_TYPE          = 0x120000;   /// Audio Visual Research
pub static SF_FORMAT_WAVEX : FORMAT_TYPE        = 0x130000;   /// MS WAVE with WAVEFORMATEX
pub static SF_FORMAT_SD2 : FORMAT_TYPE          = 0x160000;   /// Sound Designer 2
pub static SF_FORMAT_FLAC : FORMAT_TYPE         = 0x170000;   /// FLAC lossless file format
pub static SF_FORMAT_CAF : FORMAT_TYPE          = 0x180000;   /// Core Audio File format
pub static SF_FORMAT_WVE : FORMAT_TYPE          = 0x190000;   /// Psion WVE format
pub static SF_FORMAT_OGG : FORMAT_TYPE          = 0x200000;   /// Xiph OGG container
pub static SF_FORMAT_MPC2K : FORMAT_TYPE        = 0x210000;   /// Akai MPC 2000 sampler
pub static SF_FORMAT_RF64 : FORMAT_TYPE         = 0x220000;   /// RF64 WAV file
/* Subtypes from here on. */
pub static SF_FORMAT_PCM_S8 : FORMAT_TYPE       = 0x0001;     /// Signed 8 bit data
pub static SF_FORMAT_PCM_16 : FORMAT_TYPE       = 0x0002;     /// Signed 16 bit data
pub static SF_FORMAT_PCM_24 : FORMAT_TYPE       = 0x0003;     /// Signed 24 bit data
pub static SF_FORMAT_PCM_32 : FORMAT_TYPE       = 0x0004;     /// Signed 32 bit data
pub static SF_FORMAT_PCM_U8 : FORMAT_TYPE       = 0x0005;     /// Unsigned 8 bit data (WAV and RAW only)
pub static SF_FORMAT_FLOAT : FORMAT_TYPE        = 0x0006;     /// 32 bit float data
pub static SF_FORMAT_DOUBLE : FORMAT_TYPE       = 0x0007;     /// 64 bit float data
pub static SF_FORMAT_ULAW : FORMAT_TYPE         = 0x0010;     /// U-Law encoded
pub static SF_FORMAT_ALAW : FORMAT_TYPE         = 0x0011;     /// A-Law encoded
pub static SF_FORMAT_IMA_ADPCM : FORMAT_TYPE    = 0x0012;     /// IMA ADPCM
pub static SF_FORMAT_MS_ADPCM : FORMAT_TYPE     = 0x0013;     /// Microsoft ADPCM
pub static SF_FORMAT_GSM610 : FORMAT_TYPE       = 0x0020;     /// GSM 6.10 encoding
pub static SF_FORMAT_VOX_ADPCM : FORMAT_TYPE    = 0x0021;     /// Oki Dialogic ADPCM encoding
pub static SF_FORMAT_G721_32 : FORMAT_TYPE      = 0x0030;     /// 32kbs G721 ADPCM encoding
pub static SF_FORMAT_G723_24 : FORMAT_TYPE      = 0x0031;     /// 24kbs G723 ADPCM encoding
pub static SF_FORMAT_G723_40 : FORMAT_TYPE      = 0x0032;     /// 40kbs G723 ADPCM encoding
pub static SF_FORMAT_DWVW_12 : FORMAT_TYPE      = 0x0040;     /// 12 bit Delta Width Variable Word encoding
pub static SF_FORMAT_DWVW_16 : FORMAT_TYPE      = 0x0041;     /// 16 bit Delta Width Variable Word encoding
pub static SF_FORMAT_DWVW_24 : FORMAT_TYPE      = 0x0042;     /// 24 bit Delta Width Variable Word encoding
pub static SF_FORMAT_DWVW_N : FORMAT_TYPE       = 0x0043;     /// N bit Delta Width Variable Word encoding
pub static SF_FORMAT_DPCM_8 : FORMAT_TYPE       = 0x0050;     /// 8 bit differential PCM (XI only)
pub static SF_FORMAT_DPCM_16 : FORMAT_TYPE      = 0x0051;     /// 16 bit differential PCM (XI only)
pub static SF_FORMAT_VORBIS : FORMAT_TYPE       = 0x0060;     /// Xiph Vorbis encoding

/* Endian-ness options. */

pub static SF_ENDIAN_FILE : FORMAT_TYPE         = 0x00000000; /// Default file endian-ness
pub static SF_ENDIAN_LITTLE : FORMAT_TYPE       = 0x10000000; /// Force little endian-ness
pub static SF_ENDIAN_BIG : FORMAT_TYPE          = 0x20000000; /// Force big endian-ness
pub static SF_ENDIAN_CPU : FORMAT_TYPE          = 0x30000000; /// Force CPU endian-ness

pub static SF_FORMAT_SUBMASK : FORMAT_TYPE      = 0x0000FFFF;
pub static SF_FORMAT_TYPEMASK : FORMAT_TYPE     = 0x0FFF0000;
pub static SF_FORMAT_ENDMASK : FORMAT_TYPE      = 0x30000000;

pub type SNDFILE = c_void;

pub struct FormatInfo {
    format : i32,
    name : *c_char,
    extension : *c_char
}

extern "C" {
    pub fn sf_open(path : *c_char, mode : SF_MODE, info : *SndInfo) -> *SNDFILE;
    pub fn sf_open_fd(fd : i32, mode : SF_MODE, info : *SndInfo, close_desc : SF_BOOL) -> *SNDFILE;
    pub fn sf_format_check(info : *SndInfo) -> SF_BOOL;
    
    pub fn sf_seek(sndfile : *SNDFILE, frames : i64, whence : i32) -> i64;
    pub fn sf_command(sndfile : *SNDFILE, cmd : i32, data : *c_void, datasize : i32) -> Error;

    pub fn sf_error(sndfile : *SNDFILE) -> Error;
    pub fn sf_strerror(sndfile : *SNDFILE) -> *c_char;
    pub fn sf_error_number(errnum : i32) -> *c_char;

    pub fn sf_perror(sndfile : *SNDFILE) -> Error;
    pub fn sf_error_str(sndfile : *SNDFILE, string : *c_char, len : i64) ;

    pub fn sf_close(sndfile : *SNDFILE) -> Error;
    pub fn sf_write_sync(sndfile : *SNDFILE) -> ();

    pub fn sf_read_short(sndfile : *SNDFILE, ptr : *mut i16, items : i64) -> i64;
    pub fn sf_read_int(sndfile : *SNDFILE, ptr : *mut i32, items : i64) -> i64;
    pub fn sf_read_float(sndfile : *SNDFILE, ptr : *mut f32, items : i64) -> i64;
    pub fn sf_read_double(sndfile : *SNDFILE, ptr : *mut f64, items : i64) -> i64;

    pub fn sf_readf_short(sndfile : *SNDFILE, ptr : *mut i16, frames : i64) -> i64;
    pub fn sf_readf_int(sndfile : *SNDFILE, ptr : *mut i32, frames : i64) -> i64;
    pub fn sf_readf_float(sndfile : *SNDFILE, ptr : *mut f32, frames : i64) -> i64;
    pub fn sf_readf_double(sndfile : *SNDFILE, ptr : *mut f64, frames : i64) -> i64;

    pub fn sf_write_short(sndfile : *SNDFILE, ptr : *mut i16, items : i64) -> i64;
    pub fn sf_write_int(sndfile : *SNDFILE, ptr : *mut i32, items : i64) -> i64;
    pub fn sf_write_float(sndfile : *SNDFILE, ptr : *mut f32, items : i64) -> i64;
    pub fn sf_write_double(sndfile : *SNDFILE, ptr : *mut f64, items : i64) -> i64;

    pub fn sf_writef_short(sndfile : *SNDFILE, ptr : *mut i16, frames : i64) -> i64;
    pub fn sf_writef_int(sndfile : *SNDFILE, ptr : *mut i32, frames : i64) -> i64;
    pub fn sf_writef_float(sndfile : *SNDFILE, ptr : *mut f32, frames : i64) -> i64;
    pub fn sf_writef_double(sndfile : *SNDFILE, ptr : *mut f64, frames : i64) -> i64;

    pub fn sf_read_raw(sndfile : *SNDFILE, ptr : *c_void, bytes : i64) -> i64;
    pub fn sf_write_raw(sndfile : *SNDFILE, ptr : *c_void, bytes : i64) -> i64;

    pub fn sf_get_string(sndfile : *SNDFILE, str_type : i32) -> *c_char;
    pub fn sf_set_string(sndfile : *SNDFILE, str_type : i32, string : *c_char) -> Error;

}