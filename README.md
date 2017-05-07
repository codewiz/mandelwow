# MandelWow

## Overview

MandelWow is a generalized version of Mandelbrot & Julia in which both C and Z0 vary in a 4-dimensional space.
In each frame, we render a 3D slice of the MandelWow, varying the remaining dimension through time.

I wrote this hack to learn Rust & basic GLSL. Mind the mess.


## Building from source

### Native

Install cargo, then simply type:

```
cargo run --release
```

### WebAssembly / asm.js

Install emsdk 1.36.14:

```
cd ~
curl -O https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz
tar xf emsdk-portable.tar.gz
source emsdk-portable/emsdk_env.sh
emsdk install emscripten-1.37.9 --build=MinSizeRel
emsdk activate emscripten-1.37.9 --build=MinSizeRel
```

Install rustc:

```
cd ~
curl https://sh.rustup.rs -sSf | sh
source ~/.cargo/env
rustup toolchain install nightly
rustup default nightly
rustup target install asmjs-unknown-emscripten
rustup target install wasm32-unknown-emscripten
```

Build and run mandelwow:

```
cd mandelwow
./release_asmjs.sh
```

Build the WebAssembly binary:

```
cd mandelwow
./release_wasm.sh
```

## Requirements

The native binary requires OpenGL 3.1. Should perform well on reasonably modern
hardware. Tested on Linux with Intel HD Graphics 4000 (Ivy Bridge) and NVidia GTX 970.

The asmjs and WebAssembly versions were tested on Chromium 57 and Firefox 53.

## License

This project is GPL 3.0.
The support/ directory contains some code forked from Glium, which was originally Apache 2.0.
