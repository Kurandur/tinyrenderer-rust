use tinyrenderer_rust::tga::{Format, TGAColor, TGAImage};

const IMAGE_WIDTH: i32 = 100;
const IMAGE_HEIGHT: i32 = 100;

fn main() {
    let mut image = TGAImage::new(IMAGE_WIDTH, IMAGE_HEIGHT, Format::RGB);
    let magenta = TGAColor::from_rgba(255, 0, 255, 255);

    line(13, 20, 80, 40, &mut image, magenta);

    image.write_tga_file("output.tga", true, true).unwrap();
}

fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut TGAImage, color: TGAColor) {
    for i in 0..=100 {
        let t = i as f32 / 100.0;
        let x = x0 + ((x1 - x0) as f32 * t) as i32;
        let y = y0 + ((y1 - y0) as f32 * t) as i32;
        image.set(x as usize, y as usize, color).unwrap();
    }
}
