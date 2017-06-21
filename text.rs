use cgmath::conv::array4x4;
use cgmath::{Matrix4, One};
use glium;
use glium::{Surface, texture};
use image;
use std;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub struct Text<'a> {
    tex: texture::Texture2d,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program,
    params: glium::DrawParameters<'a>,
    model: Matrix4<f32>,
}

impl<'a> Text<'a> {
    pub fn new(display: &glium::Display) -> Text {
        let image =
            image::load_from_memory_with_format(&include_bytes!("c64-font.png")[..], image::PNG)
                .unwrap()
                .to_luma();
        let (w, h) = image.dimensions();
        let image = glium::texture::RawImage2d {
            data: std::borrow::Cow::from(image.into_raw()),
            width: w,
            height: h,
            format: glium::texture::ClientFormat::U8,
        };
        let tex = glium::texture::Texture2d::with_format(
            display,
            image,
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap,
        ).unwrap();

        // building the vertex buffer, which contains all the vertices that we will draw
        let vertex_buffer = {
            glium::VertexBuffer::new(
                display,
                &[
                    Vertex {
                        position: [-1.0, -1.0],
                        tex_coords: [0.0, 1.0],
                    },
                    Vertex {
                        position: [-1.0, 1.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex {
                        position: [1.0, 1.0],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex {
                        position: [1.0, -1.0],
                        tex_coords: [1.0, 1.0],
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
                uniform int index;

                in vec2 position;
                in vec2 tex_coords;

                out vec2 v_tex_coords;
                out ivec4 v_fgcolor;

                void main() {
                    gl_Position = perspview * model * vec4(position, 0.0, 1.0);

                    // Characters are arranged in a 16x16 square.
                    int xpos = index % 16;
                    int ypos = index / 16;
                    v_tex_coords = (tex_coords + vec2(xpos, ypos)) / 16.;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                uniform vec4 bgcolor;
                uniform vec4 fgcolor;

                in vec2 v_tex_coords;
                out vec4 f_color;

                void main() {
                    f_color = texture(tex, v_tex_coords).x == 0U ? bgcolor : fgcolor;
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
        let uniforms =
            uniform! {
            model: array4x4(self.model),
            perspview: *perspview,
            tex: self.tex.sampled()
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            index: 'C' as i32,
            // RGB values from http://unusedino.de/ec64/technical/misc/vic656x/colors/
            bgcolor: [  53./255.,  40./255., 121./255.,   0.0/255. ] as [f32; 4],
            fgcolor: [ 120./255., 106./255., 255./255., 188.0/255. ] as [f32; 4],
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
