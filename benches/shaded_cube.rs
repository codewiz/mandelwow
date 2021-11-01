#![feature(test)]
extern crate test;

use glium::uniform;
use mandelwow_lib::shaded_cube::*;
use std::rc::Rc;

#[bench]
fn bench_shaded_cube(b: &mut test::Bencher) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let program = Rc::new(shaded_program(&display));
    let cube = ShadedCube::new(&display, program);
    let mut frame = display.draw();
    b.iter(|| {
        let model =     [[0.7, 0.5, -0.5, 0.0], [0.0, 0.7, 0.7, 0.0], [0.7, -0.5,  0.5,  0.0], [0., 0., -3.0, 1.0f32]];
        let perspview = [[0.5, 0.0,  0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0,  0.0, -1.0, -1.0], [0., 0., -0.2, 0.0f32]];
        let uniforms = uniform! {
            model: model,
            perspview: perspview,
        };
        cube.draw(&mut frame, &uniforms);
    });
    frame.finish().unwrap();
}
