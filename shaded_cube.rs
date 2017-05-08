use cube::Cube;
use glium;
use glium::{Display, Program, Surface};
use glium::index::{IndexBuffer, PrimitiveType};

pub fn shaded_program(display: &Display) -> Program {
    let vertex_shader_src = include_str!("shaded.vert");
    let fragment_shader_src = include_str!("shaded.frag");
    return Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);

pub struct ShadedCube<'a> {
    vertexes: glium::VertexBuffer<Vertex>,
    program: &'a Program,
    indices: IndexBuffer<u16>,
}

impl<'a> ShadedCube<'a> {
    pub fn new(display: &Display, c: &Cube, program: &'a Program) -> ShadedCube<'a> {
        //      x--->
        //      4 ──────┐ 5
        //      ╱┆     ╱│
        //   0 ┌─────┐1 │
        // y   │ 7+┄┄│┄┄+ 6
        // |   │╱    │ ╱   /
        // v   └─────┘    z
        //    3       2
        let vertex_data = [
            // Front face
            Vertex { position: [c.xmin, c.ymin, c.zmin], normal: [  0.,  0.,  1.] },  // 0
            Vertex { position: [c.xmax, c.ymin, c.zmin], normal: [  0.,  0.,  1.] },  // 1
            Vertex { position: [c.xmax, c.ymax, c.zmin], normal: [  0.,  0.,  1.] },  // 2
            Vertex { position: [c.xmin, c.ymax, c.zmin], normal: [  0.,  0.,  1.] },  // 3

            // Back face
            Vertex { position: [c.xmin, c.ymax, c.zmax], normal: [  0.,  0., -1.] },  // 7
            Vertex { position: [c.xmax, c.ymax, c.zmax], normal: [  0.,  0., -1.] },  // 6
            Vertex { position: [c.xmax, c.ymin, c.zmax], normal: [  0.,  0., -1.] },  // 5
            Vertex { position: [c.xmin, c.ymin, c.zmax], normal: [  0.,  0., -1.] },  // 4

            // Right face
            Vertex { position: [c.xmax, c.ymin, c.zmin], normal: [ -1.,  0.,  0.] },  // 1
            Vertex { position: [c.xmax, c.ymin, c.zmax], normal: [ -1.,  0.,  0.] },  // 5
            Vertex { position: [c.xmax, c.ymax, c.zmax], normal: [ -1.,  0.,  0.] },  // 6
            Vertex { position: [c.xmax, c.ymax, c.zmin], normal: [ -1.,  0.,  0.] },  // 2

            // Left face
            Vertex { position: [c.xmin, c.ymin, c.zmin], normal: [  1.,  0.,  0.] },  // 0
            Vertex { position: [c.xmin, c.ymax, c.zmin], normal: [  1.,  0.,  0.] },  // 3
            Vertex { position: [c.xmin, c.ymax, c.zmax], normal: [  1.,  0.,  0.] },  // 7
            Vertex { position: [c.xmin, c.ymin, c.zmax], normal: [  1.,  0.,  0.] },  // 4

            // Top face
            Vertex { position: [c.xmin, c.ymin, c.zmin], normal: [  0.,  1.,  0.] },  // 0
            Vertex { position: [c.xmin, c.ymin, c.zmax], normal: [  0.,  1.,  0.] },  // 4
            Vertex { position: [c.xmax, c.ymin, c.zmax], normal: [  0.,  1.,  0.] },  // 5
            Vertex { position: [c.xmax, c.ymin, c.zmin], normal: [  0.,  1.,  0.] },  // 1

            // Bottom face
            Vertex { position: [c.xmax, c.ymax, c.zmin], normal: [  0., -1.,  0.] },  // 2
            Vertex { position: [c.xmax, c.ymax, c.zmax], normal: [  0., -1.,  0.] },  // 6
            Vertex { position: [c.xmin, c.ymax, c.zmax], normal: [  0., -1.,  0.] },  // 7
            Vertex { position: [c.xmin, c.ymax, c.zmin], normal: [  0., -1.,  0.] },  // 3
        ];
        const INDICES: &[u16] = &[
             0,  1,  2,  0,  2,  3,  // Front
             4,  5,  6,  4,  6,  7,  // Back
             8,  9, 10,  8, 10, 11,  // Right
            12, 13, 14, 12, 14, 15,  // Left
            16, 17, 18, 16, 18, 19,  // Top
            20, 21, 22, 20, 22, 23u16  // Bottom
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
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };
        frame.draw(&self.vertexes, &self.indices, &self.program, uniforms, &params).unwrap();
    }
}
