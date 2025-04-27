use std::{env, mem::swap};

use tinyrenderer_rust::{
    model::Model,
    tga::{Format, TGAColor, TGAImage},
};

const IMAGE_WIDTH: i32 = 800;
const IMAGE_HEIGHT: i32 = 800;
fn main() {
    let args: Vec<String> = env::args().collect();

    let model_path = if args.len() == 2 {
        &args[1]
    } else {
        "dude.obj"
    };

    let mut image = TGAImage::new(IMAGE_WIDTH, IMAGE_HEIGHT, Format::RGB);
    let white = TGAColor::from_rgba(0, 0, 255, 255);

    let model = Model::new(model_path).expect("Failed to load model");
    for i in 0..model.nfaces() {
        let face = model.face(i);
        for j in 0..3 {
            let v0 = model.vert(face[j] as usize);
            let v1 = model.vert(face[(j + 1) % 3] as usize);

            let x0 = ((v0.x + 1.0) * (IMAGE_WIDTH as f32) / 2.0) as i32;
            let y0 = ((v0.y + 1.0) * (IMAGE_HEIGHT as f32) / 2.0) as i32;
            let x1 = ((v1.x + 1.0) * (IMAGE_WIDTH as f32) / 2.0) as i32;
            let y1 = ((v1.y + 1.0) * (IMAGE_HEIGHT as f32) / 2.0) as i32;

            line(x0, y0, x1, y1, &mut image, white);
        }
    }

    image.write_tga_file("output.tga", true, true).unwrap();
}

fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut TGAImage, color: TGAColor) {
    let mut x0 = x0;
    let mut y0 = y0;
    let mut x1 = x1;
    let mut y1 = y1;
    let mut steep = false;

    if (x0 - x1).abs() < (y0 - y1).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror = dy.abs() * 2;
    let mut error = 0;
    let mut y = y0;
    if steep {
        for x in x0..=x1 {
            let _ = image.set(y as usize, x as usize, color);
            error += derror;
            if error > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error -= dx * 2;
            }
        }
    } else {
        for x in x0..=x1 {
            let _ = image.set(x as usize, y as usize, color);
            error += derror;
            if error > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error -= dx * 2;
            }
        }
    }
}
