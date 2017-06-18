use cgmath::conv::array4x4;
use cgmath::{Matrix4, One};
use glium;
use glium::{Surface, texture};
use image;
use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub struct Text<'a> {
    tex: texture::CompressedSrgbTexture2d,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program,
    params: glium::DrawParameters<'a>,
    model: Matrix4<f32>,
}

impl<'a> Text<'a> {
    pub fn new(display: &glium::Display) -> Text {
        let image = image::load(Cursor::new(&include_bytes!("c64-font.png")[..]), image::PNG)
            .unwrap()
            .to_rgba();
        let dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), dimensions);
        let tex = glium::texture::CompressedSrgbTexture2d::new(display, image).unwrap();

        // building the vertex buffer, which contains all the vertices that we will draw
        let vertex_buffer = {
            glium::VertexBuffer::new(
                display,
                &[
                    Vertex {
                        position: [-1.0, -1.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex {
                        position: [-1.0, 1.0],
                        tex_coords: [0.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, 1.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, -1.0],
                        tex_coords: [1.0, 0.0],
                    },
                ],
            ).unwrap()
        };

        let index_buffer = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TriangleStrip,
            &[1 as u16, 2, 0, 3],
        ).unwrap();

        // compiling shaders and linking them together
        let program = program!(display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 model;
                uniform mat4 perspview;

                in vec2 position;
                in vec2 tex_coords;

                out vec2 v_tex_coords;

                void main() {
                    gl_Position = perspview * model * vec4(position, 0.0, 1.0);
                    // Characters are arranged in a 16x16 square.
                    // Texture oordinates originate in the bottom-left corner.
                    v_tex_coords = (tex_coords) / 16.0 + vec2(0. / 16., 15. / 16.);
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 f_color;

                void main() {
                    f_color = texture(tex, v_tex_coords);
                }
            "
        }).unwrap();

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            multisampling: true,
            ..Default::default()
        };

        Text {
            model: Matrix4::one(),
            tex: tex,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: program,
            params: params,
        }
    }

    pub fn draw(&self, frame: &mut glium::Frame, perspview: &[[f32; 4]; 4]) {
        let uniforms = uniform! {
            model: array4x4(self.model),
            perspview: *perspview,
            tex: self.tex.sampled()
                //.minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };
        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &self.params,
            )
            .unwrap();
    }
}
