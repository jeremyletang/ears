echo "Build ears."
mkdir -p build/lib
rustc --out-dir build/lib src/ears/lib.rs

echo "Build examples."
mkdir -p 'build/examples/simple_player'
rustc -L build/lib src/examples/simple_player/main.rs -o build/examples/simple_player/simple_player
mkdir -p 'build/examples/many_sounds'
rustc -L build/lib src/examples/many_sounds/main.rs -o build/examples/many_sounds/many_sounds

echo "Build tests."
rustc --test -o bin/ears_tests src/ears/lib.rs

echo "Execute tests."
cd bin
./ears_tests