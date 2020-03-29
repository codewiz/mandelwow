#[cfg(feature = "image")]
pub fn take_screenshot(display : &glium::Display) {
    let image: glium::texture::RawImage2d<'_, u8> = display.read_front_buffer().unwrap();
    let image = image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned()).unwrap();
    let image = image::DynamicImage::ImageRgba8(image).flipv().to_rgb();
    let image = image::DynamicImage::ImageRgb8(image);
    let mut output = std::fs::File::create(&std::path::Path::new("screenshot.png")).unwrap();
    image.write_to(&mut output, image::ImageFormat::PNG).unwrap();
}

#[cfg(not(feature = "image"))]
pub fn take_screenshot(_ : &glium::Display) {
}
