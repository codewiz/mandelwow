use crate::cube::Cube;
use glium;
use glium::{Display, Program, Surface, implement_vertex};
use glium::index::{IndexBuffer, PrimitiveType};
use std::rc::Rc;

pub fn solid_fill_program(display: &Display) -> Program {
    let vertex_shader_src = include_str!("shaders/solid.vert");
    let fragment_shader_src = include_str!("shaders/solid.frag");
    Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}

#[derive(Copy, Clone)]
struct Vertex { position: [f32; 3] }
implement_vertex!(Vertex, position);

pub struct BoundingBox {
    vertexes: glium::VertexBuffer<Vertex>,
    program: Rc<Program>,
    indices: IndexBuffer<u16>,
}

impl BoundingBox {
    pub fn new(display: &Display, c: &Cube, program: Rc<Program>) -> BoundingBox {
        let vertex_data = [
            Vertex { position: [c.xmin, c.ymin, c.zmin] },
            Vertex { position: [c.xmax, c.ymin, c.zmin] },
            Vertex { position: [c.xmax, c.ymax, c.zmin] },
            Vertex { position: [c.xmin, c.ymax, c.zmin] },
            Vertex { position: [c.xmin, c.ymin, c.zmax] },
            Vertex { position: [c.xmax, c.ymin, c.zmax] },
            Vertex { position: [c.xmax, c.ymax, c.zmax] },
            Vertex { position: [c.xmin, c.ymax, c.zmax] },
        ];

        const INDICES: &[u16] = &[0, 1, 1, 2, 2, 3, 3, 0,   // front
                                  4, 5, 5, 6, 6, 7, 7, 4,   // back
                                  0, 4, 1, 5, 2, 6, 3, 7];  // sides

        BoundingBox {
            vertexes: glium::VertexBuffer::new(display, &vertex_data).unwrap(),
            program: program,
            indices:  IndexBuffer::new(display, PrimitiveType::LinesList, INDICES).unwrap(),
        }
    }

    pub fn draw<U>(&self, frame: &mut glium::Frame,
                   uniforms: &U) where U: glium::uniforms::Uniforms {
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };
        frame.draw(&self.vertexes, &self.indices, &self.program, uniforms, &params).unwrap();
    }
}
