set -e
export CFLAGS="-O2"
export CXXFLAGS="$CFLAGS"
cargo rustc --target wasm32-unknown-emscripten --release --bin mandelwow -- \
  -C link-args='--emrun -s ASSERTIONS=2 --preload-file flora.xm'
cp -a target/wasm32-unknown-emscripten/release/mandelwow.{js,wasm} .
cp -a target/wasm32-unknown-emscripten/release/deps/mandelwow.data .
emrun .
