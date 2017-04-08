#version 140
precision highp float;
in vec2 c;
in vec2 z;
out vec4 f_color;

void main() {
    float zx = z.x;
    float zy = z.y;
    int maxiter = 64;
    int iter = maxiter;
    while (iter > 0) {
        float zx2 = zx * zx;
        float zy2 = zy * zy;
        if (zx2 * zy2 > 4.0) {
          float index = float(iter) / float(maxiter);
          f_color = vec4(index, 0.1, 0.5 - index / 2, 0.8 - index);
          return;
        }
        zy = zx * zy * 2.0 + c.y;
        zx = zx2 - zy2 + c.x;
        iter -= 1;
    }
    f_color = vec4((sin(z.y) + 1.0) / 2,
                   (sin(c.y) + 1.0) / 2,
                   (sin(c.x) + 1.0) / 2,
                   1.0);
}
