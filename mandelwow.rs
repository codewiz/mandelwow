// Wow. Such fractal.

#[macro_use]

extern crate glium;
extern crate glutin;
extern crate libxm;
extern crate sdl2;

use glium::{DisplayBuild, Surface};
use glium::index::{IndexBuffer, PrimitiveType};
use glutin::ElementState::Pressed;
use glutin::Event::KeyboardInput;
use glutin::VirtualKeyCode;
use libxm::XMContext;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use std::fs::File;
use std::io::Read;

mod support;

#[derive(Copy, Clone)]
struct Cube {
    xmin: f32,
    ymin: f32,
    zmin: f32,
    xmax: f32,
    ymax: f32,
    zmax: f32,
}

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

fn mandelwow_program(display: &glium::Display) -> glium::Program {
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
                          float index = 1.0 - float(iter) / float(maxiter);
                          f_color = vec4(index, index * 0.5, index, index * 0.5);
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

fn solid_fill_program(display: &glium::Display) -> glium::Program {
    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    return glium::Program::from_source(display,
                                       vertex_shader_src,
                                       fragment_shader_src,
                                       None).unwrap();
}

fn bounding_box<U>(display: &glium::Display,
                   frame: &mut glium::Frame,
                   program: &glium::Program,
                   uniforms: &U,
                   cube: &Cube) where U: glium::uniforms::Uniforms {

    #[derive(Copy, Clone)]
    struct Vertex { position: [f32; 3] }
    implement_vertex!(Vertex, position);

    let cube = [
        Vertex { position: [cube.xmin, cube.ymin, cube.zmin] },
        Vertex { position: [cube.xmax, cube.ymin, cube.zmin] },
        Vertex { position: [cube.xmax, cube.ymax, cube.zmin] },
        Vertex { position: [cube.xmin, cube.ymax, cube.zmin] },
        Vertex { position: [cube.xmin, cube.ymin, cube.zmax] },
        Vertex { position: [cube.xmax, cube.ymin, cube.zmax] },
        Vertex { position: [cube.xmax, cube.ymax, cube.zmax] },
        Vertex { position: [cube.xmin, cube.ymax, cube.zmax] },
    ];
    let vb = glium::VertexBuffer::new(display, &cube).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    let front_indices = IndexBuffer::new(display, PrimitiveType::LineLoop,
                                         &[0, 1, 2, 3u16]).unwrap();
    frame.draw(&vb, &front_indices, program, uniforms, &params).unwrap();

    let back_indices = IndexBuffer::new(display, PrimitiveType::LineLoop,
                                        &[4, 5, 6, 7u16]).unwrap();
    frame.draw(&vb, &back_indices, program, uniforms, &params).unwrap();

    let sides_indices = IndexBuffer::new(display, PrimitiveType::LinesList,
                                         &[0, 4, 1, 5, 2, 6, 3, 7u16]).unwrap();
    frame.draw(&vb, &sides_indices, program, uniforms, &params).unwrap();
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

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    //let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
    //let indices = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList,
    //                                      &[0u16, 1, 2]).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    frame.draw(&vb, &indices, program, uniforms, &params).unwrap();
}

fn mandelwow(display: &glium::Display,
             mut frame: &mut glium::Frame,
             program: &glium::Program,
             model: [[f32; 4]; 4],
             camera: &support::camera::CameraState,
             bounds: &Cube,
             mandel_w: f32) {
    let mut z0 = [mandel_w, 0f32];
    let zres = 50;
    let zmin = bounds.zmin;
    let zmax = bounds.zmax;
    let zstep = (zmax - zmin) / zres as f32;
    let mut zy = zmin;
    for _ in 0..zres {
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

struct XmCallback {
    xm: XMContext,
}

impl AudioCallback for XmCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        self.xm.generate_samples(out);
    }
}

struct SoundPlayer {
    _device: AudioDevice<XmCallback>,
}

fn play_xm(raw_xm: &[u8]) -> SoundPlayer {
    let sdl_context = sdl2::init().unwrap();
    let sdl_audio = sdl_context.audio().unwrap();

    let sample_rate = 48000u32;
    let desired_spec = AudioSpecDesired {
        freq: Some(sample_rate as i32),
        channels: Some(2u8),
        samples: None,
    };
    let device = sdl_audio.open_playback(None, &desired_spec, |actual_spec| {
        let xm = XMContext::new(&raw_xm, actual_spec.freq as u32).unwrap();

        XmCallback {
            xm: xm,
        }
    }).unwrap();

    device.resume();

    SoundPlayer {
        _device: device,
    }
}

fn main() {
    let mut xm = Vec::new();
    File::open("flora.xm").unwrap().read_to_end(&mut xm).unwrap();
    let _sound_player = play_xm(&xm);

    let display = glium::glutin::WindowBuilder::new()
        //.with_dimensions(1024, 768)
        .with_fullscreen(glutin::get_primary_monitor())
        .with_depth_buffer(24)
        .with_vsync()
        .with_title(format!("MandelWow"))
        .build_glium()
        .unwrap();

    let program = mandelwow_program(&display);
    let bounding_box_program = solid_fill_program(&display);

    let mut camera = support::camera::CameraState::new();
    let mut t: f32 = 0.0;
    let mut pause = false;
    let mut bounding_box_enabled = true;
    let mut fullscreen = true;

    support::start_loop(|| {
        camera.update();

        if !pause {
            // Increment time
            t += 0.01;
        }

        // These are the bounds of the 3D Mandelwow section which we render in 3-space.
        let bounds = Cube {
            xmin: -2.0,
            xmax:  0.7,
            ymin: -1.0,
            ymax:  1.0,
            zmin: -1.2,
            zmax:  1.2,
        };

        // Vary the wow factor to slice the Mandelwow along its 4th dimension.
        let wmin = -0.8;
        let wmax =  0.8;
        let wsize = wmax - wmin;
        let wow = (((t * 0.7).sin() + 1.0) / 2.0) * wsize + wmin;

        //println!("t={} w={:?} camera={:?}", t, w, camera.get_pos());

        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let z_trans = -2.0;  // Send the model back a little bit so it fits the screen.
        let model = [
            [ t.cos(),  t.sin(),  0.0,     0.0],
            [-t.sin(),  t.cos(),  0.0,     0.0],
            [     0.0,  0.0,      1.0,     0.0],
            [     0.0,  0.0,      z_trans, 1.0f32]
        ];

        // Draw the bounding box before the fractal, when the Z-buffer is still clear, so the lines
        // behind the semi-translucent areas will be drawn.
        if bounding_box_enabled {
            let uniforms = uniform! {
                model: model,
                view:  camera.get_view(),
                perspective: camera.get_perspective(),
            };
            bounding_box(&display, &mut frame, &bounding_box_program, &uniforms, &bounds);
        }

        mandelwow(&display, &mut frame, &program, model, &camera, &bounds, wow);
        frame.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed |
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Escape)) |
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::Q)) => {
                    return support::Action::Stop
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::B)) => {
                    bounding_box_enabled ^= true;
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::F)) => {
                    fullscreen ^= true;
                    if fullscreen {
                        glutin::WindowBuilder::new()
                            .with_fullscreen(glutin::get_primary_monitor())
                            .rebuild_glium(&display).unwrap();
                    } else {
                        glutin::WindowBuilder::new()
                            .rebuild_glium(&display).unwrap();
                    }
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::P)) => {
                    pause ^= true;
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::PageUp)) => {
                    t += 0.01;
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::PageDown)) => {
                    t -= 0.01;
                },
                ev => camera.process_input(&ev),
            }
        }

        support::Action::Continue
    });

}
