#version 140
uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform vec2 z0;
in vec3 position;
out vec2 c;
out vec2 z;

void main() {
    mat4 modelview = view * model;
    gl_Position = perspective * modelview * vec4(position, 1.0);
    c = vec2(position.x, position.y);
    z = vec2(z0.x, z0.y);
}
