// Wow. Such fractal.

#[macro_use]

extern crate glium;
extern crate glutin;
extern crate libxm;
extern crate sdl2;

use cube::Cube;
use glium::{DisplayBuild, Surface};
use glium::index::{IndexBuffer, PrimitiveType};
use glutin::ElementState::Pressed;
use glutin::Event::KeyboardInput;
use glutin::VirtualKeyCode;

mod cube;
mod mandelwow;
mod sound;
mod support;

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
            ..Default::default()
        },
        blend: glium::Blend::alpha_blending(),
        ..Default::default()
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

fn main() {
    sound::start();

    let display = glium::glutin::WindowBuilder::new()
        //.with_dimensions(1024, 768)
        .with_fullscreen(glutin::get_primary_monitor())
        .with_depth_buffer(24)
        .with_vsync()
        .with_title(format!("MandelWow"))
        .build_glium()
        .unwrap();

    let mandelwow_program = mandelwow::program(&display);
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
            zmin: -1.1,
            zmax:  1.1,
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

        // Draw the bounding box before the fractal, when the Z-buffer is still clear,
        // so the lines behind the semi-translucent areas will be drawn.
        if bounding_box_enabled {
            let uniforms = uniform! {
                model: model,
                view:  camera.get_view(),
                perspective: camera.get_perspective(),
            };
            bounding_box(&display, &mut frame, &bounding_box_program, &uniforms, &bounds);
        }

        mandelwow::draw(&display, &mut frame, &mandelwow_program, model, &camera, &bounds, wow);
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
