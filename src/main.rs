use std::f32::consts::PI;

use tinyrenderer_rust::tga::{Format, TGAColor, TGAImage};

const IMAGE_WIDTH: i32 = 100;
const IMAGE_HEIGHT: i32 = 100;

fn main() {
    let mut image = TGAImage::new(IMAGE_WIDTH, IMAGE_HEIGHT, Format::RGB);
    let magenta = TGAColor::from_rgba(255, 0, 255, 255);

    image.set(52, 41, magenta).unwrap();

    image.write_tga_file("output.tga", true, true).unwrap();
}
