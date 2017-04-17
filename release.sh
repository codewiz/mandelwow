set -e
cargo build --target asmjs-unknown-emscripten --release
cp target/asmjs-unknown-emscripten/release/mandelwow.js .
emrun .
