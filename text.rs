use cgmath::conv::array4x4;
use cgmath::Matrix4;
use glium;
use glium::{Display, Program, Surface, implement_vertex, texture, uniform};
use std;

fn gamma<T>(x: T) -> f32
where
    f32: From<T>,
    T: Copy,
{
    ((f32::from(x)) / 255.).powf(2.2)
}

fn srgb<T>(c: [T; 3]) -> [f32; 4]
where
    f32: From<T>,
    T: Copy,
{
    [gamma(c[0]), gamma(c[1]), gamma(c[2]), 0.0]
}

#[cfg(feature = "image")]
fn c64_font() -> (u32, u32, Vec<u8>) {
    let image = image::load_from_memory_with_format(
        &include_bytes!("textures/c64-font.png")[..],
        image::PNG,
    ).unwrap()
        .to_luma();
    let (w, h) = image.dimensions();
    (w, h, image.into_raw())
}

#[cfg(not(feature = "image"))]
fn c64_font() -> (u32, u32, Vec<u8>) {
    let pixels = &include_bytes!("textures/c64-font.gray")[..];
    (128, 128, Vec::from(pixels))
}

pub fn text_program(display: &Display) -> Program {
    //load_program(display, "shaders/text.vert", "shaders/text.frag");
    let vertex_shader_src = include_str!("shaders/text.vert");
    let fragment_shader_src = include_str!("shaders/text.frag");
    Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub struct Text {
    tex: texture::Texture2d,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program,
    params: glium::DrawParameters<'static>,
}

impl Text {
    pub fn new(display: &Display) -> Text {
        let (w, h, pixels) = c64_font();
        let image = glium::texture::RawImage2d {
            data: std::borrow::Cow::from(pixels),
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
                        position: [-0.5, -0.5],
                        tex_coords: [0.0, 1.0],
                    },
                    Vertex {
                        position: [-0.5, 0.5],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex {
                        position: [0.5, 0.5],
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex {
                        position: [0.5, -0.5],
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
            tex: tex,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: text_program(display),
            params: params,
        }
    }

    pub fn draw(&self, frame: &mut glium::Frame, c: char, model: &Matrix4<f32>, perspview: &[[f32; 4]; 4]) {
        let uniforms =
            uniform! {
            model: array4x4(*model),
            perspview: *perspview,
            tex: self.tex.sampled()
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            index: c as i32,
            // RGB values from http://unusedino.de/ec64/technical/misc/vic656x/colors/
            bgcolor: srgb([ 64,  50, 133u8]),  //  6 - blue
            fgcolor: srgb([120, 106, 189u8]),  // 14 - light blue
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
