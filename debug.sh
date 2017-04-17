set -e
cargo build --target asmjs-unknown-emscripten
cp target/asmjs-unknown-emscripten/debug/mandelwow.js .
emrun .
