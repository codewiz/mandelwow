extern crate mandelwow_lib;

extern crate cgmath;
#[macro_use(uniform)]
extern crate glium;
extern crate glutin;
extern crate image;

use cgmath::{Euler, Matrix4, Rad, Vector3};
use glium::{DisplayBuild, Surface};
use glutin::ElementState::Pressed;
use glutin::Event::KeyboardInput;
use glutin::VirtualKeyCode;
use mandelwow_lib::*;

#[cfg(target_os = "emscripten")]
use std::os::raw::{c_int, c_void};

fn screenshot(display : &glium::Display) {
    let image: glium::texture::RawImage2d<u8> = display.read_front_buffer();
    let image = image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned()).unwrap();
    let image = image::DynamicImage::ImageRgba8(image).flipv();
    let mut output = std::fs::File::create(&std::path::Path::new("screenshot.png")).unwrap();
    image.save(&mut output, image::ImageFormat::PNG).unwrap();
}

fn gl_info(display : &glium::Display) {
    let version = *display.get_opengl_version();
    let api = match version {
        glium::Version(glium::Api::Gl, _, _) => "OpenGL",
        glium::Version(glium::Api::GlEs, _, _) => "OpenGL ES"
    };
    println!("{} context verson: {}", api, display.get_opengl_version_string());
}

#[cfg(target_os = "emscripten")]
#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern fn();
#[cfg(target_os = "emscripten")]
extern {
    fn emscripten_set_main_loop(func : em_callback_func, fps : c_int, simulate_infinite_loop : c_int);
}

#[cfg(target_os = "emscripten")]
thread_local!(static MAIN_LOOP_CALLBACK: std::cell::RefCell<*mut c_void> =
              std::cell::RefCell::new(std::ptr::null_mut()));

#[cfg(target_os = "emscripten")]
pub fn set_main_loop_callback<F>(callback : F) where F : FnMut() -> support::Action {
    MAIN_LOOP_CALLBACK.with(|log| {
            *log.borrow_mut() = &callback as *const _ as *mut c_void;
            });

    unsafe { emscripten_set_main_loop(wrapper::<F>, 0, 1); }

    unsafe extern "C" fn wrapper<F>() where F : FnMut() -> support::Action {
        MAIN_LOOP_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            (*closure)();
        });
    }
}

#[cfg(not(target_os = "emscripten"))]
pub fn set_main_loop_callback<F>(callback : F) where F : FnMut() -> support::Action {
    support::start_loop(callback);
}

fn main() {
    let _soundplayer = sound::start();

    let display = glutin::WindowBuilder::new()
        .with_dimensions(600, 600)
        //.with_fullscreen(glutin::get_primary_monitor())
        .with_depth_buffer(24)
        .with_vsync()
        .with_title(format!("MandelWow"))
        .build_glium()
        .unwrap();

    gl_info(&display);

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
    let mandelwow_bbox = bounding_box::BoundingBox::new(&display, &bounds, &bounding_box_program);

    set_main_loop_callback(|| {
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

        let rotation = Matrix4::from(
            Euler { x: Rad(t.sin() / 3.), y: Rad(t.sin() / 2.), z: Rad(t / 1.5)});
        let z_trans = -2.0;  // Send the model back a little bit so it fits the screen.
        let model2 =
            Matrix4::from_translation(Vector3::unit_z() * z_trans) * rotation;
        let model = cgmath::conv::array4x4(model2);

        // Draw the bounding box before the fractal, when the Z-buffer is still clear,
        // so the lines behind the semi-translucent areas will be drawn.
        if bounding_box_enabled {
            let uniforms = uniform! {
                model: model,
                view:  camera.get_view(),
                perspective: camera.get_perspective(),
            };
            mandelwow_bbox.draw(&mut frame, &uniforms);
        }

        mandelwow::draw(&display, &mut frame, &mandelwow_program, model, &camera, &bounds, wow);
        frame.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glutin::Event::Closed |
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
