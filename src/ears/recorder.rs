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

//! Record audio


#[allow(missing_doc)];

use std::task;
use std::comm::Data;
use std::cast;

use record_context::RecordContext;
use record_context;
use openal::ffi;
use sndfile::{SndInfo, SndFile, FormatWav, FormatPcm16, Write};

/**
 * Record audio
 *
 * This class provide easy audio recording using. The Recorder allow the user
 * to record sound, then save it in a file, or create a SoundData object to play the 
 * recorded sound in the same program.
 * A special context, RecordContext is needed to create the Recorder object.
 * The Recorder work in it's own task.
 *
 * # Examples
 * ```Rust
 * extern mod ears;
 * use ears::{RecordContext, Recorder};
 *
 * fn main() -> () {
 *     // Create a new context to record audio
 *     let context = ears::init_in().unwrap();
 *     // Create the recorder
 *     let recorder = Recorder::new(context);
 *     // Start to record something
 *     recorder.start();
 *
 *     // Do some other stuff here //
 *
 *     // Stop the recorder
 *     recorder.stop();
 *     // Then store the recorded data in a file
 *     recorder.save_to_file("hello_file");
 * }
 * ```
 */
pub struct Recorder {
    priv ctxt: RecordContext,
    priv stop_chan: Option<Chan<bool>>,
    priv data_port: Option<Port<~[i16]>>,
    priv samples: ~[i16]
}

impl Recorder {
    /// Create a new audio recorder
    pub fn new(record_context: RecordContext) -> Recorder {
        Recorder {
            ctxt: record_context,
            stop_chan: None,
            data_port: None,
            samples: ~[]

        }
    }

    pub fn start(&mut self) {
        let (stop_port, stop_chan) = Chan::new();
        let (data_port, data_chan) = Chan::new();
        let r_c = self.ctxt.clone();

        self.stop_chan = Some(stop_chan);
        self.data_port = Some(data_port);

        task::spawn(proc() {
            let mut terminate = false;
            let ctxt = record_context::get(r_c);
            unsafe { ffi::alcCaptureStart(ctxt); }
            let mut available_samples = 0;
            let mut samples: ~[i16] = ~[];

            while !terminate {
                unsafe {
                    ffi::alcGetIntegerv(ctxt,
                                        ffi::ALC_CAPTURE_SAMPLES,
                                        1,
                                        &mut available_samples)
                };

                if available_samples != 0 {
                    let tmp_buf =
                        ::std::vec::from_elem(available_samples as uint, 0i16);
                    unsafe {
                        ffi::alcCaptureSamples(ctxt,
                                               cast::transmute(&tmp_buf[0]),
                                               available_samples);
                    }
                    samples.push_all_move(tmp_buf);
                }

                match stop_port.try_recv() {
                    Data(_) => {
                        unsafe { ffi::alcCaptureStop(ctxt); }
                        terminate = true;
                    },
                    _       => {}
                }
            }
            data_chan.send(samples);
        });
    }

    pub fn stop(&mut self) -> bool {
        match self.stop_chan {
            Some(ref s_c) => {
                s_c.send(true);
                match self.data_port {
                    Some(ref d_p) => {
                        self.samples = d_p.recv();
                        true
                    },
                    None          => false
                }
            },
            None      => false
        }
    }

    pub fn save_to_file(&mut self, filename: &str) -> bool {
        if self.samples.len() == 0 {
            false
        } else {
            let infos = ~SndInfo {
                frames : self.samples.len() as i64,
                samplerate : 44100,
                channels : 1,
                format : (FormatPcm16 | FormatWav) as i32,
                sections : 0,
                seekable : 0
            };

            let mut file_ext = filename.to_owned();
            file_ext.push_str(".wav");
            match SndFile::new_with_info(file_ext, Write, infos) {
                Ok(mut f) => {
                    let len = self.samples.len() as i64;
                    f.write_i16(self.samples, len);
                    f.close();
                    true
                },
                Err(e) => { println!("{}", e); false }
            }
        }
    }
}
