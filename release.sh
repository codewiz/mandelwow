set -e
cargo rustc --target asmjs-unknown-emscripten --release -- -C link-args='-s USE_SDL=2'
cp target/asmjs-unknown-emscripten/release/mandelwow.js .
emrun .
