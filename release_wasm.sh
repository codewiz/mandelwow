set -e
cargo rustc --target wasm32-unknown-emscripten --release --bin mandelwow -- -C link-args='-s USE_SDL=2 --preload-file flora.xm'
cp -a target/wasm32-unknown-emscripten/release/mandelwow.js .
cp -a target/wasm32-unknown-emscripten/release/deps/mandelwow-*.{wasm,data} .
emrun .
