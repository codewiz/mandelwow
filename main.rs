use cgmath::conv::array4x4;
use cgmath::{Euler, Matrix4, Rad, SquareMatrix, Vector3, Vector4, Zero};
use glium::glutin::event::{ self, Event, VirtualKeyCode, WindowEvent };
use glium::glutin::event_loop::{ ControlFlow };
use glium::{Display, Program, Surface, uniform};
use mandelwow_lib::*;
use std::f32::consts::PI;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[cfg(target_os = "emscripten")]
use std::os::raw::{c_int, c_void};

fn gl_info(display: &glium::Display) {
    if cfg!(feature = "logging") {
        let version = *display.get_opengl_version();
        let api = match version {
            glium::Version(glium::Api::Gl, _, _) => "OpenGL",
            glium::Version(glium::Api::GlEs, _, _) => "OpenGL ES",
        };
        println!(
            "{} context verson: {}",
            api,
            display.get_opengl_version_string()
        );
    }
}

const SEA_XSIZE: usize = 40;
const SEA_ZSIZE: usize = 25;

struct World<'a> {
    display: Display,

    mandelwow_program: Rc<Program>,
    mandelwow_bounds: Cube,
    mandelwow_bbox: BoundingBox,
    bounding_box_enabled: bool,

    shaded_cube: ShadedCube,
    text: Text<'a>,

    sea: [[Vector3<f32>; SEA_ZSIZE]; SEA_XSIZE],

    // For the zoom animation synchronized to the drum-hits
    hit_time: f32,
    last_hit: f32,
}

impl<'a> World<'a> {
    pub fn new(display: glium::Display) -> World<'a> {
        let mandelwow_program = Rc::new(mandelwow::program(&display));
        let bounding_box_program = Rc::new(bounding_box::solid_fill_program(&display));
        let shaded_program = Rc::new(shaded_cube::shaded_program(&display));

        // These are the bounds for the 3D slice of the 4D Mandelwow
        let mandelwow_bounds = Cube {
            xmin: -2.0,
            xmax: 0.7,
            ymin: -1.0,
            ymax: 1.0,
            zmin: -1.1,
            zmax: 1.1,
        };

        // Generate a wavy sea made of cubes
        let sea_xmin = -20.0f32;
        let sea_xmax = 20.0f32;
        let sea_y = -2.5;
        let sea_zmin = -2.0f32;
        let sea_zmax = -27.0f32;
        let sea_xstep = (sea_xmax - sea_xmin) / (SEA_XSIZE as f32);
        let sea_zstep = (sea_zmax - sea_zmin) / (SEA_ZSIZE as f32);
        println!("xstep={} ystep={:?}", sea_xstep, sea_zstep);

        let mut sea = [[Vector3::zero(); SEA_ZSIZE]; SEA_XSIZE];
        for x in 0..SEA_XSIZE {
            for z in 0..SEA_ZSIZE {
                sea[x][z] = Vector3 {
                    x: sea_xmin + (x as f32) * sea_xstep,
                    y: sea_y,
                    z: sea_zmin + (z as f32) * sea_zstep,
                };
            }
        }

        World {
            mandelwow_program,
            mandelwow_bbox: BoundingBox::new(
                &display, &mandelwow_bounds, bounding_box_program.clone()),
            mandelwow_bounds,
            bounding_box_enabled: true,

            shaded_cube: ShadedCube::new(&display, shaded_program.clone()),
            text: text::Text::new(&display),
            sea: sea,

            hit_time: 0.0,
            last_hit: 0.0,

            display,
        }
    }

    fn draw_frame(
        &self,
        camera: &support::camera::CameraState,
        t: f32,
    ) {
        let perspview = camera.get_perspview();

        let hit_delta = t - self.hit_time;
        let hit_scale = 1. / (1. + hit_delta * hit_delta * 15.0) + 1.;

        // Vary the wow factor to slice the Mandelwow along its 4th dimension.
        let wmin = -0.8;
        let wmax = 0.8;
        let wsize = wmax - wmin;
        let wow = (((t * 0.7).sin() + 1.0) / 2.0) * wsize + wmin;

        //println!("t={} w={:?} camera={:?}", t, w, camera.get_pos());

        let mut frame = self.display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let rotation = Matrix4::from(Euler {
            x: Rad(t.sin() / 3.),
            y: Rad(t.sin() / 2.),
            z: Rad(t / 1.5),
        });
        let z_trans = -3.0; // Send the model back a little bit so it fits the screen.
        let scale = Matrix4::from_diagonal(Vector4::new(hit_scale, hit_scale, hit_scale, 1.0));
        let model2 = Matrix4::from_translation(Vector3::unit_z() * z_trans) * rotation * scale;
        let model = array4x4(model2);

        // Draw the bounding box before the fractal, when the Z-buffer is still clear,
        // so the lines behind the semi-translucent areas will be drawn.
        if self.bounding_box_enabled {
            let uniforms = uniform! {
                model: model,
                view:  camera.get_view(),
                perspective: camera.get_perspective(),
            };
            self.mandelwow_bbox.draw(&mut frame, &uniforms);
        }

        let text_rot = Matrix4::from_angle_x(cgmath::Deg(-90.0f32));
        let text_pos = Matrix4::from_translation(Vector3 {
            x: 0.0,
            y: 0.501,
            z: 0.0f32,
        }) * text_rot;
        for x in 0..SEA_XSIZE {
            for z in 0..SEA_ZSIZE {
                let wave = ((x as f32 / SEA_XSIZE as f32 * PI * 5.0 + t * 2.0).sin()
                    + (z as f32 / SEA_ZSIZE as f32 * PI * 3.0 + t * 3.0).sin())
                    * 0.3;
                let model = Matrix4::from_translation(
                    self.sea[x][z]
                        + Vector3 {
                            x: 0.,
                            y: wave,
                            z: 0.,
                        },
                );
                let uniforms = uniform! {
                    model: array4x4(model),
                    perspview: perspview,
                    col: [0., (1. - wave).abs() * 0.5,  wave.abs()],
                };
                self.shaded_cube.draw(&mut frame, &uniforms);
                let model = model * text_pos;
                let c = (x + z * SEA_XSIZE) as u8 as char;
                self.text.draw(&mut frame, c, &model, &perspview);
            }
        }

        mandelwow::draw(
            &self.display,
            &mut frame,
            &self.mandelwow_program,
            model,
            &camera,
            &self.mandelwow_bounds,
            wow,
        );

        frame.finish().unwrap();
    }
}


#[cfg(target_os = "emscripten")]
#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern "C" fn();
#[cfg(target_os = "emscripten")]
extern "C" {
    fn emscripten_set_main_loop(func: em_callback_func, fps: c_int, simulate_infinite_loop: c_int);
}

#[cfg(target_os = "emscripten")]
thread_local!(static MAIN_LOOP_CALLBACK: std::cell::RefCell<*mut c_void> =
              std::cell::RefCell::new(std::ptr::null_mut()));

#[cfg(target_os = "emscripten")]
pub fn set_main_loop_callback<F>(callback: F)
where
    F: FnMut() -> support::Action,
{
    MAIN_LOOP_CALLBACK.with(|log| {
        *log.borrow_mut() = &callback as *const _ as *mut c_void;
    });

    unsafe {
        emscripten_set_main_loop(wrapper::<F>, 0, 1);
    }

    unsafe extern "C" fn wrapper<F>()
    where
        F: FnMut() -> support::Action,
    {
        MAIN_LOOP_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            (*closure)();
        });
    }
}

#[cfg(not(target_os = "emscripten"))]
pub fn set_main_loop_callback<F>(callback: F)
where
    F: FnMut() -> support::Action,
{
    support::start_loop(callback);
}

//extern crate gleam;

/*
extern "C" {
    fn emscripten_GetProcAddress(
        name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_void;
}
*/

fn main() {
    /*
    let gl = gleam::gl::GlesFns::load_with(|addr| {
            let addr = std::ffi::CString::new(addr).unwrap();
            emscripten_GetProcAddress(addr.into_raw() as *const _) as *const _
        });
    gl.glGetInternalformativ(0, 0, 0, 0, 0);
    */

    let mut soundplayer = sound::start();

    let event_loop = glutin::event_loop::EventLoop::new();
    //let fullscreen = Some(glutin::window::Fullscreen::Borderless(event_loop.primary_monitor()));
    let window = glutin::window::WindowBuilder::new()
        //.with_dimensions(1280, 720)
        //.with_fullscreen(fullscreen);
        ;
    //.with_title("MandelWow");
    let context = glutin::ContextBuilder::new()
        //.with_gl_profile(glutin::GlProfile::Core)
        //.with_gl(glutin::GlRequest::Specific(glutin::Api::WebGl, (2, 0)))
        .with_gl(glutin::GlRequest::Specific(
            glutin::Api::OpenGlEs,
            (3, 0),
        ))
        //.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 0)))
        //.with_depth_buffer(24)
        .with_vsync(true);

    let display = glium::Display::new(window, context, &event_loop).unwrap();
    gl_info(&display);
    let mut world = World::new(display);

    let mut timer = Timer::new();
    let mut camera = support::camera::CameraState::new();
    let _fullscreen = true;

    event_loop.run(move |event, _, control_flow| {
        let t = timer.t;
        let new_hit = sound::hit_event(&mut soundplayer);
        if new_hit > world.last_hit {
            world.hit_time = t;
        }
        world.last_hit = new_hit;

        camera.update();

        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_nanos(16666667));
        match event {
            Event::NewEvents(cause) => {
                match cause {
                    event::StartCause::ResumeTimeReached { .. } | event::StartCause::Init => {
                        world.draw_frame(&camera, t);
                    },
                    _ => {}
                }
            } _ => (),
        }
        if let Event::WindowEvent { event, .. } = event {
            camera.process_input(&event);
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == event::ElementState::Pressed {
                        if let Some(key) = input.virtual_keycode {
                            match key {
                                VirtualKeyCode::Escape | VirtualKeyCode::Q => {
                                    *control_flow = ControlFlow::Exit;
                                }
                                _ => (),
                            }
                        }
                    }
                }
                /*
                KeyboardInput { input: glutin::KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::Escape), .. } } |
                KeyboardInput { input: glutin::KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::Q), .. } } => {
                    *control_flow = ControlFlow::Exit;
                },
                KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::B) } => {
                    bounding_box_enabled ^= true;
                },
                KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::P) } => {
                    timer.pause ^= true;
                },
                KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::PageUp) } => {
                    timer.t += 0.01;
                },
                KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::PageDown) } => {
                    timer.t -= 0.01;
                },
                KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::F10) } => {
                    screenshot(&display);
                },
                KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::F11) } => {
                    fullscreen ^= true;
                    if fullscreen {
                        // Not implemented on Linux
                        glutin::WindowBuilder::new()
                            .with_fullscreen(glutin::get_primary_monitor())
                            .with_depth_buffer(24)
                            .rebuild_glium(&display).unwrap();
                    } else {
                        glutin::WindowBuilder::new()
                            .rebuild_glium(&display).unwrap();
                    }
                },
                */
                _ => (),
            }
        }

        timer.update();
    });
}
