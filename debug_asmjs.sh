set -e
export CFLAGS="-g"
export CXXFLAGS="$CFLAGS"
cargo rustc --target asmjs-unknown-emscripten --bin mandelwow -- -C link-args='--emrun -s USE_WEBGL2=1 -s USE_SDL=2 -s ASSERTIONS=2 --preload-file flora.xm'
cp -a target/asmjs-unknown-emscripten/debug/mandelwow.js .
emrun .
