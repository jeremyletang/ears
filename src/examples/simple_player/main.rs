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

extern crate ears;

use std::io::stdin;
use std::io::stdio::flush;

use ears::{Music, AudioController, Playing, Stopped, Paused};

fn main() {

    // Read the inputs
    let mut stdin = stdin();

    print!("Insert the path to an audio file : ");
    flush();

    let mut line = StrBuf::from_str(stdin.read_line().unwrap());
    unsafe { line.pop_byte(); }

    // Try to create the music
    let mut music = match Music::new(line.into_owned()) {
        Some(music) => music,
        None        => fail!("Cannot load the music.")
    };

    // Play it
    music.play();

    loop {
        // Make your choice
        println!("Commands :\n\tPlay  : l\n\tPause : p\n\tStop  : s\n\tExit  : x\n");
        match stdin.read_line().unwrap().as_slice() {
            "l\n"    => music.play(),
            "p\n"    => music.pause(),
            "s\n"    => music.stop(),
            "x\n"    => { music.stop(); break; },
            _       => println!("Unknwon command.")
        }
        match music.get_state() {
            Playing => println!("State : Playing"),
            Stopped => println!("State : Stopped"),
            Paused  => println!("State : Paused"),
            _       => unreachable!()
        };
    }
    println!("Goodbye!");
}