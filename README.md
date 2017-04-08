# MandelWow

## Overview

MandelWow is a generalized version of Mandelbrot & Julia in which both C and Z0 vary in a 4-dimensional space.
In each frame, we render a 3D slice of the MandelWow, varying the remaining dimension through time.

I wrote this hack to learn Rust & basic GLSL. Mind the mess.


## Building from source

Install cargo, then simply type:

```
cargo run --release
```


## Requirements

Requires OpenGL 3.1. Should perform well on reasonably modern hardware.
Tested on Linux with Intel HD Graphics 4000 (Ivy Bridge) and NVidia GTX 970.

## License

This project is GPL 3.0.
The support/ directory contains some code forked from Glium, which was originally Apache 2.0.
