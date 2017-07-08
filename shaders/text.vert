#version 140

uniform mat4 model;
uniform mat4 perspview;
uniform int index;

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coords;
out ivec4 v_fgcolor;

void main() {
    gl_Position = perspview * model * vec4(position, 0.0, 1.0);

    // Characters are arranged in a 16x16 square.
    int xpos = index % 16;
    int ypos = index / 16;
    v_tex_coords = (tex_coords + vec2(xpos, ypos)) / 16.;
}
