use glium;
use glium::{Display, Program, Surface, implement_vertex};
use glium::index::{IndexBuffer, PrimitiveType};
use std::rc::Rc;

pub fn shaded_program(display: &Display) -> Program {
    let vertex_shader_src = include_str!("shaders/shaded.vert");
    let fragment_shader_src = include_str!("shaders/shaded.frag");
    Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);

pub struct ShadedCube {
    vertexes: glium::VertexBuffer<Vertex>,
    program: Rc<Program>,
    indices: IndexBuffer<u16>,
}

impl<'a> ShadedCube {
    pub fn new(display: &Display, program: Rc<Program>) -> ShadedCube {
        //      x--->
        //      4 ──────┐ 5
        //      ╱┆     ╱│
        //   0 ┌─────┐1 │
        // y   │ 7+┄┄│┄┄+ 6    z
        // |   │╱    │ ╱    ╱
        // v   └─────┘
        //    3       2
        let vertex_data = [
            // Front
            Vertex { position: [-0.5, -0.5,  0.5], normal: [  0.,  0., -1.] },  // 0
            Vertex { position: [ 0.5, -0.5,  0.5], normal: [ -1.,  0.,  0.] },  // 1
            Vertex { position: [ 0.5,  0.5,  0.5], normal: [  0., -1.,  0.] },  // 2
            Vertex { position: [-0.5,  0.5,  0.5], normal: [  1.,  0.,  0.] },  // 3

            // Back
            Vertex { position: [-0.5, -0.5, -0.5], normal: [  0.,  1.,  0.] },  // 4
            Vertex { position: [ 0.5, -0.5, -0.5], normal: [  0.,  0.,  1.] },  // 5
            Vertex { position: [ 0.5,  0.5, -0.5], normal: [  0.,  0.,  0.] },  // 6
            Vertex { position: [-0.5,  0.5, -0.5], normal: [  0.,  0.,  0.] },  // 7
        ];
        const INDICES: &[u16] = &[
             1,  2,  0,  2,  3,  0,    // Front
             5,  6,  1,  6,  2,  1,    // Right
             6,  7,  2,  7,  3,  2,    // Top
             4,  0,  3,  7,  4,  3,    // Left
             5,  1,  4,  1,  0,  4,    // Top
             7,  6,  5,  4,  7,  5u16  // Back
        ];

        ShadedCube {
            vertexes: glium::VertexBuffer::new(display, &vertex_data).unwrap(),
            program: program,
            indices:  IndexBuffer::new(display, PrimitiveType::TrianglesList, INDICES).unwrap(),
        }
    }

    pub fn draw<U>(&self, frame: &mut glium::Frame, uniforms: &U)
            where U: glium::uniforms::Uniforms {
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };
        frame.draw(&self.vertexes, &self.indices, &self.program, uniforms, &params).unwrap();
    }
}
