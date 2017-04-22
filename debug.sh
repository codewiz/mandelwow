set -e
cargo rustc --target asmjs-unknown-emscripten -- -C link-args='-s USE_SDL=2 --preload-file flora.xm'
cp target/asmjs-unknown-emscripten/debug/mandelwow.js .
cp target/asmjs-unknown-emscripten/debug/deps/mandelwow-ec0c575eb8c2718a.data .
emrun .
