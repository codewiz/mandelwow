// Wow. Such fractal.

#[macro_use]

extern crate glium;
extern crate glutin;
extern crate image;
extern crate libxm;
extern crate sdl2;

use cube::Cube;
use glium::{DisplayBuild, Surface};
use glutin::ElementState::Pressed;
use glutin::Event::KeyboardInput;
use glutin::VirtualKeyCode;

mod bounding_box;
mod cube;
mod mandelwow;
mod sound;
mod support;

fn screenshot(display : &glium::Display) {
    let image: glium::texture::RawImage2d<u8> = display.read_front_buffer();
    let image = image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned()).unwrap();
    let image = image::DynamicImage::ImageRgba8(image).flipv();
    let mut output = std::fs::File::create(&std::path::Path::new("screenshot.png")).unwrap();
    image.save(&mut output, image::ImageFormat::PNG).unwrap();
}

fn main() {
    let _soundplayer = sound::start();

    let display = glium::glutin::WindowBuilder::new()
        //.with_dimensions(1024, 768)
        .with_fullscreen(glutin::get_primary_monitor())
        .with_depth_buffer(24)
        .with_vsync()
        .with_title(format!("MandelWow"))
        .build_glium()
        .unwrap();

    let mandelwow_program = mandelwow::program(&display);
    let bounding_box_program = bounding_box::solid_fill_program(&display);

    let mut camera = support::camera::CameraState::new();
    let mut t: f32 = 0.0;
    let mut pause = false;
    let mut bounding_box_enabled = true;
    let mut fullscreen = true;

    // These are the bounds of the 3D Mandelwow section which we render in 3-space.
    let bounds = Cube {
        xmin: -2.0,
        xmax:  0.7,
        ymin: -1.0,
        ymax:  1.0,
        zmin: -1.1,
        zmax:  1.1,
    };

    support::start_loop(|| {
        camera.update();

        if !pause {
            // Increment time
            t += 0.01;
        }

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
            bounding_box::draw(&display, &mut frame, &bounding_box_program, &uniforms, &bounds);
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
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::P)) => {
                    pause ^= true;
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::PageUp)) => {
                    t += 0.01;
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::PageDown)) => {
                    t -= 0.01;
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::F10)) => {
                    screenshot(&display);
                },
                KeyboardInput(Pressed, _, Some(VirtualKeyCode::F11)) => {
                    fullscreen ^= true;
                    if fullscreen {
                        // Not implemented on Linux
                        glutin::WindowBuilder::new()
                            .with_fullscreen(glutin::get_primary_monitor())
                            .with_depth_buffer(24)
                            .rebuild_glium(&display).unwrap();
                    } else {
                        //glutin::WindowBuilder::new()
                        //    .rebuild_glium(&display).unwrap();
                    }
                },
                ev => camera.process_input(&ev),
            }
        }

        support::Action::Continue
    });

}
