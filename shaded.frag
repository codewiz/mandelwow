#version 300 es
precision lowp float;

flat in vec4 color;
out vec4 color_out;

void main() {
    color_out = color;
}
