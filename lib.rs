pub mod bounding_box;
pub mod cube;
pub mod mandelwow;
pub mod shaded_cube;
pub mod sound;
pub mod support;
pub mod text;
pub mod timer;

pub use crate::bounding_box::BoundingBox;
pub use crate::cube::Cube;
pub use crate::shaded_cube::ShadedCube;
pub use crate::text::Text;
pub use crate::timer::Timer;

#[cfg(feature = "image")]
pub fn screenshot(display : &glium::Display) {
    let image: glium::texture::RawImage2d<'_, u8> = display.read_front_buffer().unwrap();
    let image = image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned()).unwrap();
    let image = image::DynamicImage::ImageRgba8(image).flipv().to_rgb();
    let image = image::DynamicImage::ImageRgb8(image);
    let mut output = std::fs::File::create(&std::path::Path::new("screenshot.png")).unwrap();
    image.write_to(&mut output, image::ImageFormat::PNG).unwrap();
}

#[cfg(not(feature = "image"))]
pub fn screenshot(_ : &glium::Display) {
}
