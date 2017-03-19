#[macro_use]

extern crate glium;

use glium::DisplayBuild;
use glium::Surface;
//use glium::index::PrimitiveType;

mod support;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

fn mand(cx: f32, cy: f32) -> [f32; 3] {
    let mut maxiter = 64;
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

fn mandel<U>(display: &glium::Display,
          frame: &mut glium::Frame,
          uniforms: &U,
          t: f32) where U: glium::uniforms::Uniforms {
    let program = program!(display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 perspective;
                uniform mat4 view;
                uniform mat4 model;
                in vec3 position;
                in vec3 color;
                out vec2 c;
                void main() {
                    mat4 modelview = view * model;
                    gl_Position = perspective * modelview * vec4(position, 1.0);
                    c = vec2(gl_Position.x, gl_Position.y);
                }
            ",

            fragment: "
                #version 140
                precision mediump float;
                in vec2 c;
                out vec4 f_color;

                void main() {
                    float zx = c.x;
                    float zy = c.y;
                    int iter = 64;
                    while (iter > 0) {
                        float zx2 = zx * zx;
                        float zy2 = zy * zy;
                        if (zx2 * zy2 > 4.0) {
                          f_color = vec4(0, 0, 0, 0);
                          return;
                        }
                        zy = zx * zy * 2.0 + c.y;
                        zx = zx2 - zy2 + c.x;
                        iter -= 1;
                    }
                    f_color = vec4(1.0, 1.0, 1.0, 1.0);
                    //f_color = vec4(vColor, 1.0);
                }
            "
        }).unwrap();

    let xmin = -1.3;
    let xmax =  0.7;
    let ymin = -1.0;
    let ymax =  1.0;
    let width = xmax - xmin;
    let height = ymax - ymin;
    let xres: usize = 100;
    let yres: usize = 100;
    let xstep = width / xres as f32;
    let ystep = height / yres as f32;
    let vb_size = (xres * 2 + 4) * yres;
    let mut v : Vec<Vertex> = Vec::with_capacity(vb_size);
    v.resize(vb_size, Vertex { position: [0.0, 0.0, 1.0], color: [0.0, 0.0, 0.0] });
    let mut i: usize = 0;
    let mut vy = ymin;
    for _ in 0..yres {
        let mut vx = xmin;
        v[i] = Vertex { position: [vx, vy+ystep, 1.0], color: [0.0, 0.0, 0.0] }; i+=1;
        v[i] = Vertex { position: [vx, vy, 1.0], color: [vx, vy, 0.0] }; i+=1;
        for _ in 0..xres {
            let c = mand(vx, vy);
            v[i] = Vertex { position: [vx+xstep, vy+ystep, 1.0], color: c }; i += 1;
            v[i] = Vertex { position: [vx+xstep, vy      , 1.0], color: c }; i += 1;
            vx += xstep;
        }
        v[i] = Vertex { position: [vx+xstep, vy, 1.0], color: [0.0, 0.0, 0.0] }; i+=1;
        v[i] = Vertex { position: [vx+xstep, vy, 1.0], color: [0.0, 0.0, 0.0] }; i+=1;
        vy += ystep;
    }
    //let vb = glium::VertexBuffer::empty_persistent(display, width*height*3).unwrap();
    let vb = glium::VertexBuffer::new(display, &v).unwrap();

    //let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
    //let indices = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList,
    //                                      &[0u16, 1, 2]).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    //let mut s = display.draw();
    frame.draw(&vb, &indices, &program, uniforms, &params).unwrap();
}

fn main() {

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_depth_buffer(24)
        .with_title(format!("Mandel"))
        .build_glium()
        .unwrap();

    let mut camera = support::camera::CameraState::new();

    //let mut t: f32 = -0.5;
    let mut t: f32 = 0.0;
    support::start_loop(|| {
        camera.update();

        //t += 0.002;
        //println!("t={}", t);

        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.5, 1.0), 1.0);

        let model = [
            [ (t*5.0).cos(), (t*3.0).sin(), 0.0, 0.0],
            [-t.sin(),       -t.cos(),      0.0, 0.0],
            [     0.0,                      0.0, 1.0, 0.0],
            [       t,                      0.0, 0.0, 1.0f32]
        ];

        let uniforms = uniform! {
            model: model,
            view:  camera.get_view(), // view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]),
            perspective: camera.get_perspective(), // perspective,
        };

        mandel(&display, &mut frame, &uniforms, t);

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return support::Action::Stop,
                ev => camera.process_input(&ev),
            }
        }

        frame.finish().unwrap();
        support::Action::Continue
    });
}
