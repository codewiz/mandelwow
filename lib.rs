extern crate cgmath;
#[macro_use(uniform,program,implement_vertex)]
extern crate glium;
extern crate glutin;
#[cfg(feature = "image")]
extern crate image;
extern crate libxm;
#[cfg(feature = "rust_rocket")]
extern crate rust_rocket;
extern crate sdl2;

pub mod bounding_box;
pub mod cube;
pub mod mandelwow;
pub mod shaded_cube;
pub mod sound;
pub mod support;
pub mod text;

pub use bounding_box::BoundingBox;
pub use cube::Cube;
pub use shaded_cube::ShadedCube;

#[cfg(feature = "image")]
pub fn screenshot(display : &glium::Display) {
    let image: glium::texture::RawImage2d<u8> = display.read_front_buffer();
    let image = image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned()).unwrap();
    let image = image::DynamicImage::ImageRgba8(image).flipv();
    let mut output = std::fs::File::create(&std::path::Path::new("screenshot.png")).unwrap();
    image.save(&mut output, image::ImageFormat::PNG).unwrap();
}

#[cfg(not(feature = "image"))]
pub fn screenshot(_ : &glium::Display) {
}
