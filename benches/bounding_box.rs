#![feature(test)]
extern crate test;

use glium::uniform;
use mandelwow_lib::Cube;
use mandelwow_lib::bounding_box::*;
use std::rc::Rc;

#[bench]
fn bench_bounding_box(b: &mut test::Bencher) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let program = Rc::new(solid_fill_program(&display));
    let bounds = Cube { xmin: -2., xmax: 0.7, ymin: -1., ymax:  1., zmin: -1.1, zmax:  1.1 };
    let bbox = BoundingBox::new(&display, &bounds, program);
    let mut frame = display.draw();
    b.iter(|| {
        let mat = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.0f32]];
        let uniforms = uniform! {
            model: mat,
            view: mat,
            perspective: mat,
        };
        bbox.draw(&mut frame, &uniforms);
    });
    frame.finish().unwrap();
}
