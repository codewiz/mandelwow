set -e
export CFLAGS="-O2"
export CXXFLAGS="$CFLAGS"
#cargo rustc --target wasm32-unknown-emscripten --release --bin mandelwow -- -C link-args='-s USE_WEBGL2=1 -s USE_SDL=2 --preload-file flora.xm'
cargo rustc --target wasm32-unknown-emscripten --release --bin mandelwow -- -C link-args='-s USE_SDL=2 --preload-file flora.xm'
cp -a target/wasm32-unknown-emscripten/release/mandelwow.{js,wasm} .
cp -a target/wasm32-unknown-emscripten/release/deps/mandelwow.data .
emrun .
