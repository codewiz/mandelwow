#version 300 es
precision lowp float;

uniform sampler2D tex;
uniform vec4 bgcolor;
uniform vec4 fgcolor;

in vec2 v_tex_coords;
out vec4 f_color;

void main() {
    f_color = texture(tex, v_tex_coords).x == 0. ? bgcolor : fgcolor;
}
