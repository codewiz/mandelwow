set -e
cargo rustc --target asmjs-unknown-emscripten -- -C link-args='-s USE_SDL=2 --preload-file flora.xm'
cp -a target/asmjs-unknown-emscripten/debug/mandelwow.js .
cp -a target/asmjs-unknown-emscripten/debug/deps/mandelwow-*.data .
emrun .
