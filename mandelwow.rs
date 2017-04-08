// Wow. Such fractal.

use cube::Cube;
use glium;
use glium::index::PrimitiveType;
use glium::Surface;
use support;


/*
fn mand(cx: f32, cy: f32) -> [f32; 3] {
    let maxiter = 64;
    let mut iter = maxiter;
    let mut zx = cx;
    let mut zy = cy;
    while iter > 0 {
        let zx2 = zx * zx;
        let zy2 = zy * zy;
        if zx2 + zy2 > 4.0 {
            return [iter as f32 / maxiter as f32, 1.0, 1.0];
        }
        zy = zx * zy * 2.0 + cy;
        zx = zx2 - zy2 + cx;
        iter -= 1;
    }

    [0.0, 0.0, 0.0]
}
*/

pub fn program(display: &glium::Display) -> glium::Program {
    return program!(display,
        140 => {
            vertex: r#"
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
            "#,

            fragment: r#"
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
            "#
        }).unwrap();
}

fn mandel<U>(display: &glium::Display,
          frame: &mut glium::Frame,
          program: &glium::Program,
          uniforms: &U,
          bounds: &Cube,
          z: [f32; 2]) where U: glium::uniforms::Uniforms {

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        color: [f32; 3],
    }
    implement_vertex!(Vertex, position, color);

    let xmin = bounds.xmin;
    let xmax = bounds.xmax;
    let ymin = bounds.ymin;
    let ymax = bounds.ymax;

    let width = xmax - xmin;
    let height = ymax - ymin;
    let xres: usize = 1;
    let yres: usize = 1;
    let xstep = width / (xres as f32);
    let ystep = height / (yres as f32);
    let vb_size = (xres * 2 + 4) * yres;
    let mut v : Vec<Vertex> = Vec::with_capacity(vb_size);
    v.resize(vb_size, Vertex { position: [0.0, 0.0, -1.0], color: [0.0, 0.0, 0.0] });
    let mut i: usize = 0;
    let mut vy = ymin;
    let vz = z[1];
    for _ in 0..yres {
        let mut vx = xmin;
        let c = [0.0, 0.0, 1.0];
        v[i] = Vertex { position: [vx, vy+ystep, vz], color: c }; i += 1;
        v[i] = Vertex { position: [vx, vy,       vz], color: c }; i += 1;
        for _ in 0..xres {
            //let c = mand(vx, vy);
            v[i] = Vertex { position: [vx+xstep, vy+ystep, vz], color: c }; i += 1;
            v[i] = Vertex { position: [vx+xstep, vy,       vz], color: c }; i += 1;
            vx += xstep;
        }
        v[i] = Vertex { position: [vx,   vy, vz], color: c }; i += 1;
        v[i] = Vertex { position: [xmin, vy, vz], color: c }; i += 1;
        vy += ystep;
    }

    //let vb = glium::VertexBuffer::empty_persistent(display, width*height*3).unwrap();
    let vb = glium::VertexBuffer::new(display, &v).unwrap();

    let indices = glium::index::NoIndices(PrimitiveType::TriangleStrip);
    //let indices = glium::index::NoIndices(PrimitiveType::LineStrip);
    //let indices = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList,
    //                                      &[0u16, 1, 2]).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        blend: glium::Blend::alpha_blending(),
        ..Default::default()
    };

    frame.draw(&vb, &indices, program, uniforms, &params).unwrap();
}

pub fn draw(display: &glium::Display,
             mut frame: &mut glium::Frame,
             program: &glium::Program,
             model: [[f32; 4]; 4],
             camera: &support::camera::CameraState,
             bounds: &Cube,
             mandel_w: f32) {
    let mut z0 = [mandel_w, 0f32];
    let zres = 30;
    let zmin = bounds.zmin;
    let zmax = bounds.zmax;
    let zstep = (zmax - zmin) / zres as f32;
    let mut zy = zmin;
    // zres + 1 to reach the other face of the cube (fencepost error)
    for _ in 0..(zres + 1) {
        z0[1] = zy;
        zy += zstep;

        let uniforms = uniform! {
            z0: z0,
            model: model,
            view:  camera.get_view(),
            perspective: camera.get_perspective(),
        };

        mandel(&display, &mut frame, &program, &uniforms, bounds, z0);
    }
}
