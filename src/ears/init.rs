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
* __ears__ initialization (optional).
*
* This module provide a unique function to initialize __ears__. 
* Use this function in the case of you don't use __ears__ for the first time 
* in you program in the main task. This prevent that the context was created and destroyed
* in a another task.
*/

use internal::OpenAlData;

/**
* Initialize the internal context
*
* # Return
* true if initialization is made with success, false otherwise
*/
pub fn init() -> bool {
    match OpenAlData::check_al_context() {
        Ok(_)       => true,
        Err(err)    => { println!("{}", err); false }
    }
}

#[cfg(test)]
mod test {
    use init::ears_init;

    #[test]
    fn test_init_ears_OK() -> () {
        assert_eq!(init(), true)
    }
}