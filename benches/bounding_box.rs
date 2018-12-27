#![feature(test)]

extern crate mandelwow_lib;
extern crate glutin;
#[macro_use(uniform)]
extern crate glium;
extern crate test;

use mandelwow_lib::Cube;
use mandelwow_lib::bounding_box::*;

#[bench]
fn bench_bounding_box(b: &mut test::Bencher) {
    let events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let program = solid_fill_program(&display);
    let bounds = Cube { xmin: -2., xmax: 0.7, ymin: -1., ymax:  1., zmin: -1.1, zmax:  1.1 };
    let bbox = BoundingBox::new(&display, &bounds, &program);
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
