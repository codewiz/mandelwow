#![feature(test)]

extern crate mandelwow_lib;
#[macro_use(uniform)]
extern crate glium;
extern crate glutin;
extern crate test;

use glium::DisplayBuild;
use mandelwow_lib::Cube;
use mandelwow_lib::shaded_cube::*;

#[bench]
fn bench_shaded_cube(b: &mut test::Bencher) {
    let display = glutin::WindowBuilder::new().build_glium().unwrap();
    let program = shaded_program(&display);
    let bounds = Cube { xmin: -2., xmax: 0.7, ymin: -1., ymax:  1., zmin: -1.1, zmax:  1.1 };
    let cube = ShadedCube::new(&display, &bounds, &program);
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
