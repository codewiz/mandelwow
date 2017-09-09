set -e
cargo rustc --target asmjs-unknown-emscripten --release --no-default-features --bin mandelwow -- -C link-args='-s USE_SDL=2'
cp -a target/asmjs-unknown-emscripten/release/mandelwow.js .
emrun .
