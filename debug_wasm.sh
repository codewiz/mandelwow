set -e
export CFLAGS="-g"
export CXXFLAGS="$CFLAGS"
cargo rustc --target wasm32-unknown-emscripten --bin mandelwow -- -C link-args='-s USE_SDL=2 --preload-file flora.xm'
cp -a target/wasm32-unknown-emscripten/release/mandelwow.{js,wasm} .
cp -a target/wasm32-unknown-emscripten/release/deps/mandelwow.data .
emrun .
