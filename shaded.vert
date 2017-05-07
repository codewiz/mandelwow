#version 300 es
precision lowp float;

in vec3 position;
in vec3 normal;
out vec4 color;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main() {
    mat4 modelview = view * model;
    mat4 m = perspective * modelview;
    vec3 dark = vec3(0.0, 0.0, 0.1);
    vec3 bright = vec3(0.0, 0.0, 0.9);
    vec3 u_light = vec3(-0.5, -0.7, -0.6);
    vec3 v_normal = transpose(inverse(mat3(model))) * normal;
    float brightness = max(dot(normalize(v_normal), normalize(u_light)), 0.0);
    color = vec4(mix(dark, bright, brightness), 1.0);
    gl_Position = m * vec4(position, 1.0);
}
