# MandelWow

## Overview

MandelWow is a generalized version of Mandelbrot & Julia in which both C and Z0 vary in a 4-dimensional space.
On each frame we render a 3D slice of the MandelWow, varying the remaining dimension through time.

I wrote this demo to learn Rust & basic GLSL. Mind the mess.


## Requirements

The native binary requires OpenGL 3.1. Should perform well on reasonably modern
hardware. Tested on:

 * Linux with NVidia GTX 970 (Mesa 17.1 Nouveau driver)
 * Linux with Intel HD Graphics 4000 (Ivy Bridge)
 * Windows 10 with NVidia GTX 970
 * The asmjs and WebAssembly build works on Chromium 57 and Firefox 53

## Building from source

### Linux

Install cargo (either your distro's packaged version or via rustup), then go to the source root and type:

```
cd mandelwow
git submodule init
git submodule update
cargo run --release
```

### Windows

Mandelwow depends on [libxm](https://github.com/nukep/libxm-rs) and [SDL2](https://github.com/AngryLawyer/rust-sdl2) for sound.
Both are tricky to build using Rust's MSVC toolchain because they assume GCC, so I recommend using the GNU toolchain instead.
I followed these steps to build a native Windows binary:

* Install Cargo via [rustup](https://www.rustup.rs).
* Install mingw-w64. The [MSYS2](http://www.msys2.org/) installer provides the basic shell environment and the package manager.
* From the MSYS2 console, install the W64 native toolchain and libSDL2:
  ```
  pacman -Sy
  pacman -S mingw-w64-x86_64-gcc mingw-w64-x86_64-SDL2
  ```
* While you're at it, you may also want to install the git package: `pacman -S git`
* I had to manually add the mingw-w64 toolchain to your path:
  ```
  export PATH="/mingw64/bin:$PATH"
  export LIBRARY_PATH=/mingw64/lib
  ```
* In case `cargo` and `rustup` aren't in the shell path, add `$USERPROFILE/.cargo/bin` too.
* Next, install the Rust toolchain targeting GCC:
  ```
  rustup toolchain install stable-x86_64-pc-windows-gnu
  rustup default stable-x86_64-pc-windows-gnu
  ```
* Now you can proceed to mandelwow's source tree and build a native Windows binary without further hassles:
  ```
  cargo run --release
  ```

### WebAssembly / asm.js

Install emsdk:

```
cd ~
curl -O https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz
tar xf emsdk-portable.tar.gz
cd emsdk-portable
source ./emsdk-portable/emsdk_env.sh
emsdk install latest --build=MinSizeRel
emsdk activate latest --build=MinSizeRel
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

## License

This project is GPL 3.0.
The support/ directory contains some code forked from Glium, which was originally Apache 2.0.
