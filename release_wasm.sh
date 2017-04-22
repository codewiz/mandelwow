set -e
cargo build --target wasm32-unknown-emscripten --release
cp target/wasm32-unknown-emscripten/release/mandelwow.js .
cp target/wasm32-unknown-emscripten/release/deps/mandelwow-*.wasm .
emrun .
