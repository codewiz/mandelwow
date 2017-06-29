extern crate mandelwow_lib;

extern crate cgmath;
#[macro_use(uniform)]
extern crate glium;
extern crate glutin;

use cgmath::{Euler, Matrix4, Rad, SquareMatrix, Vector3, Vector4, Zero};
use cgmath::conv::array4x4;
use glium::{DisplayBuild, Surface};
use glutin::ElementState::Pressed;
use glutin::Event::KeyboardInput;
use glutin::VirtualKeyCode;
use mandelwow_lib::*;
use std::f32::consts::PI;
use std::time::{Duration, Instant};

#[cfg(target_os = "emscripten")]
use std::os::raw::{c_int, c_void};

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
    let mut soundplayer = sound::start();

    let display = glutin::WindowBuilder::new()
        .with_dimensions(1280, 720)
        .with_gl_profile(glutin::GlProfile::Core)
        //.with_fullscreen(glutin::get_primary_monitor())
        .with_depth_buffer(24)
        .with_vsync()
        .with_srgb(Some(true))
        .with_title("MandelWow")
        .build_glium()
        //.build_glium_debug(glium::debug::DebugCallbackBehavior::PrintAll)
        .unwrap();

    gl_info(&display);

    let text = text::Text::new(&display);
    let mandelwow_program = mandelwow::program(&display);
    let bounding_box_program = bounding_box::solid_fill_program(&display);
    let shaded_program = shaded_cube::shaded_program(&display);

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
    let shaded_cube = ShadedCube::new(&display, &shaded_program);

    const SEA_XSIZE: usize = 24;
    const SEA_ZSIZE: usize = 20;
    let sea_xmin = -14.0f32;
    let sea_xmax =  14.0f32;
    let sea_y = -2.5;
    let sea_zmin =  -2.0f32;
    let sea_zmax = -26.0f32;
    let sea_xstep = (sea_xmax - sea_xmin) / (SEA_XSIZE as f32);
    let sea_zstep = (sea_zmax - sea_zmin) / (SEA_ZSIZE as f32);

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

    let mut frame_cnt = 0;
    let mut last_report_time = Instant::now();
    let mut last_report_frame_cnt = 0;
    let mut accum_draw_time = Duration::new(0, 0);
    let mut accum_idle_time = Duration::new(0, 0);

    let mut last_hit = 0.0f32;
    let mut hit_time = 0.0f32;
    set_main_loop_callback(|| {
        let new_hit = sound::hit_event(&mut soundplayer);
        if new_hit > last_hit {
            hit_time = t;
        }
        last_hit = new_hit;
        let hit_delta = t - hit_time;
        let hit_scale = 1. / (1. + hit_delta * hit_delta * 15.0) + 1.;

        camera.update();
        let perspview = camera.get_perspview();

        // Vary the wow factor to slice the Mandelwow along its 4th dimension.
        let wmin = -0.8;
        let wmax =  0.8;
        let wsize = wmax - wmin;
        let wow = (((t * 0.7).sin() + 1.0) / 2.0) * wsize + wmin;

        //println!("t={} w={:?} camera={:?}", t, w, camera.get_pos());

        let time_before_swap = Instant::now();
        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        let time_before_draw = Instant::now();

        let rotation = Matrix4::from(
            Euler { x: Rad(t.sin() / 3.), y: Rad(t.sin() / 2.), z: Rad(t / 1.5)});
        let z_trans = -3.0;  // Send the model back a little bit so it fits the screen.
        let scale =
            Matrix4::from_diagonal(Vector4::new(hit_scale, hit_scale, hit_scale, 1.0));
        let model2 =
            Matrix4::from_translation(Vector3::unit_z() * z_trans) * rotation * scale;
        let model = array4x4(model2);

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

        for x in 0..SEA_XSIZE {
            for z in 0..SEA_ZSIZE {
                let wave = ((x as f32 / SEA_XSIZE as f32 * PI * 5.0 + t * 2.0).sin() +
                            (z as f32 / SEA_ZSIZE as f32 * PI * 3.0 + t * 3.0).sin()) * 0.3;
                let model = Matrix4::from_translation(sea[x][z] + Vector3 {x: 0., y: wave, z: 0.});
                let uniforms = uniform! {
                    model: array4x4(model),
                    perspview: perspview,
                    col: [0., (1. - wave).abs() * 0.5,  wave.abs()],
                };
                shaded_cube.draw(&mut frame, &uniforms);
            }
        }

        mandelwow::draw(&display, &mut frame, &mandelwow_program, model, &camera, &bounds, wow);

        text.draw(&mut frame, &perspview);

        frame.finish().unwrap();
        let time_after_draw = Instant::now();

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
                        glutin::WindowBuilder::new()
                            .rebuild_glium(&display).unwrap();
                    }
                },
                ev => camera.process_input(&ev),
            }
        }

        let now = Instant::now();
        frame_cnt += 1;
        accum_idle_time += time_before_draw - time_before_swap;
        accum_draw_time += time_after_draw - time_before_draw;
        if now - last_report_time > Duration::from_secs(5) {
            fn millis(d : Duration) -> f32 {
                d.as_secs() as f32 * 1e3 + d.subsec_nanos() as f32 / 1e6
            }
            let frames_done = frame_cnt - last_report_frame_cnt;
            let fps = frames_done as f32 / (now - last_report_time).as_secs() as f32;
            let avg_draw_time = millis(accum_draw_time / frames_done);
            let avg_idle_time = millis(accum_idle_time / frames_done);
            println!("fps={:.1} draw={:.1}ms idle={:.1}ms", fps, avg_draw_time, avg_idle_time);

            last_report_time = now;
            last_report_frame_cnt = frame_cnt;
            accum_draw_time = Duration::new(0, 0);
            accum_idle_time = Duration::new(0, 0);
        }

        if !pause {
            // Increment time
            t += 0.01;
        }

        support::Action::Continue
    });
}
