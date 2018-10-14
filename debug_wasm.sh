set -e
export CFLAGS="-g"
export CXXFLAGS="$CFLAGS"
cargo rustc --target wasm32-unknown-emscripten --bin mandelwow -- -C link-args='--emrun -s USE_SDL=2 -s USE_WEBGL2=1 -s GL_ASSERTIONS=1 -s ASSERTIONS=2 -s DEMANGLE_SUPPORT=1 --preload-file flora.xm'
cp -a target/wasm32-unknown-emscripten/debug/mandelwow.{js,wasm} .
cp -a target/wasm32-unknown-emscripten/debug/deps/mandelwow.data .
emrun .
