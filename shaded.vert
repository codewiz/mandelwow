#version 300 es
precision lowp float;

in vec3 position;
in vec3 normal;
out vec4 color;  // Shaded color

uniform mat4 model;
uniform mat4 perspview;
uniform vec3 col;

void main() {
    mat4 m = perspview * model;
    vec3 dark = col * 0.1;
    vec3 u_light = vec3(-0.5, -0.7, -0.6);
    vec3 v_normal = transpose(inverse(mat3(model))) * normal;

    float distance = model[3][2];
    float attenuation = 1. / (1. + distance * distance * 0.05);
    float brightness = max(dot(normalize(v_normal), normalize(u_light)) * attenuation, 0.0);
    color = vec4(mix(dark, col, brightness), 1.0);

    gl_Position = m * vec4(position, 1.0);
}
