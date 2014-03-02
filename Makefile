# The MIT License (MIT)
# 
# Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
# 
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the "Software"), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
# the Software, and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
# 
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
# 
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
# FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
# COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
# IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

CARGO_OUT_DIR ?= lib
CARGO_RUSTFLAGS ?= -O -g

all: ears examples docs

ears:
	mkdir -p $(CARGO_OUT_DIR)
	rustc --out-dir=$(CARGO_OUT_DIR) $(CARGO_RUSTFLAGS) src/ears/lib.rs

docs:
	mkdir -p doc
	rustdoc -o doc src/ears/lib.rs

examples:
	rustc -o bin/many_sounds -L ./lib src/examples/many_sounds/main.rs
	rustc -o bin/simple_player -L ./lib src/examples/simple_player/main.rs


clean:
	rm -rf lib
	rm -rf doc
	rm -rf bin/many_sounds
	rm -rf bin/simple_player
