#version 100
precision highp float;
varying vec2 c;
varying vec2 z;

void main() {
    float zx = z.x;
    float zy = z.y;
    const int maxiter = 64;
    for (int iter = maxiter; iter > 0; iter--) {
        float zx2 = zx * zx;
        float zy2 = zy * zy;
        if (zx2 * zy2 > 4.0) {
          float index = float(iter) / float(maxiter);
          gl_FragColor = vec4(index, 0.1, 1.0 - index / 2.0, 0.8 - index * index);
          return;
        }
        zy = zx * zy * 2.0 + c.y;
        zx = zx2 - zy2 + c.x;
    }
    gl_FragColor = vec4((sin(z.y) + 1.0) / 4.0,
                        (sin(z.x) + 1.0) / 4.0,
                        (sin(c.x) + 1.0) / 4.0,
                        1.0);
}
