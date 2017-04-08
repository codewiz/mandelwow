use cube::Cube;
use glium;
use glium::{Display, Program, Surface};
use glium::index::{IndexBuffer, PrimitiveType};

pub fn solid_fill_program(display: &Display) -> Program {
    let vertex_shader_src = include_str!("solid.vert");
    let fragment_shader_src = include_str!("solid.frag");
    return Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
}

pub fn draw<U>(display: &Display,
               frame: &mut glium::Frame,
               program: &Program,
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
