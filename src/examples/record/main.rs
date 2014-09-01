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

#![crate_name = "record"]

extern crate ears;

use std::time::Duration;
use std::io::timer::sleep;

fn main() -> () {
    // call ears_init() function to ensure that the ears context is not destroyed by a task.
    ears::init();

    // initialize the RecordContext
    let ctxt = ears::init_in().expect("initialization error !");

    // Create a new Recorder using the RecordContext
    let mut recorder = ears::Recorder::new(ctxt);
    recorder.start();
    sleep(Duration::milliseconds(5000i64));
    recorder.stop();
    match recorder.save_to_file("hello") {
        true => println!("Save okay !"),
        false => println!("Cannot save ...")
    }
}