use std::{env, mem::swap};

use rand::Rng;
use tinyrenderer_rust::{
    geometry::{Vec2f, Vec2i},
    model::Model,
    tga::{Format, TGAColor, TGAImage},
};

const IMAGE_WIDTH: i32 = 2000;
const IMAGE_HEIGHT: i32 = 2000;

fn main() {
    let args: Vec<String> = env::args().collect();

    let model_path = if args.len() == 2 {
        &args[1]
    } else {
        "dude.obj"
    };

    let white = TGAColor::from_rgba(255, 255, 255, 255);
    let red = TGAColor::from_rgba(255, 0, 0, 255);
    let green = TGAColor::from_rgba(0, 255, 0, 255);
    let magenta = TGAColor::from_rgba(255, 0, 255, 255);
    let mut image = TGAImage::new(IMAGE_WIDTH, IMAGE_HEIGHT, Format::RGB);
    let mut rng = rand::rng();
    let model = Model::new(model_path).expect("Failed to load model");
    for i in 0..model.nfaces() {
        let face = model.face(i);
        let screen_coords: Vec<Vec2i> = face
            .iter()
            .map(|&idx| {
                let v = model.vert(idx as usize);
                Vec2i::new(
                    ((v.x + 1.0) * IMAGE_WIDTH as f32 / 2.0) as i32,
                    ((v.y + 1.0) * IMAGE_HEIGHT as f32 / 2.0) as i32,
                )
            })
            .collect();

        triangle(
            screen_coords[0],
            screen_coords[1],
            screen_coords[2],
            &mut image,
            TGAColor::from_rgba(
                rng.random_range(0..255),
                rng.random_range(0..255),
                rng.random_range(0..255),
                255,
            ),
        );
    }

    image.write_tga_file("output.tga", true, true).unwrap();
}

fn triangle(t0: Vec2i, t1: Vec2i, t2: Vec2i, image: &mut TGAImage, color: TGAColor) {
    let mut t0 = t0;
    let mut t1 = t1;
    let mut t2 = t2;
    if t0.v > t1.v {
        std::mem::swap(&mut t0, &mut t1);
    }
    if t0.v > t2.v {
        std::mem::swap(&mut t0, &mut t2);
    }
    if t1.v > t2.v {
        std::mem::swap(&mut t1, &mut t2);
    }
    let total_height = (t2.v - t0.v) as usize;
    for i in 0..total_height {
        let second_half = i as i32 > t1.v - t0.v || t1.v == t0.v;
        let segment_height = if second_half {
            t2.v - t1.v
        } else {
            t1.v - t0.v
        };
        let alpha = i as f32 / total_height as f32;
        let beta = if second_half {
            (i as i32 - (t1.v - t0.v)) as f32 / segment_height as f32
        } else {
            (i as f32) / segment_height as f32
        };
        let mut a = t0 + (t2 - t0) * alpha;
        let mut b = if second_half {
            t1 + (t2 - t1) * beta
        } else {
            t0 + (t1 - t0) * beta
        };
        if a.u > b.u {
            std::mem::swap(&mut a, &mut b);
        }
        for j in a.u..=b.u {
            let _ = image.set(j as usize, t0.v as usize + i, color);
        }
    }
    for y in t0.v..=t1.v {
        let segment_height = t1.v - t0.v;
        let alpha: f32 = (y - t0.v) as f32 / total_height as f32;
        let beta: f32 = (y - t0.v) as f32 / segment_height as f32;

        let mut a = t0 + (t2 - t0) * alpha;
        let mut b = t0 + (t1 - t0) * beta;
        if a.u > b.u {
            std::mem::swap(&mut a, &mut b);
        }
        for j in a.u..=b.u {
            let _ = image.set(j as usize, y as usize, color);
        }
    }
    for y in t1.v..=t2.v {
        let segment_height = t2.v - t1.v;
        let alpha: f32 = (y - t0.v) as f32 / total_height as f32;
        let beta: f32 = (y - t1.v) as f32 / segment_height as f32;

        let mut a = t0 + (t2 - t0) * alpha;
        let mut b = t1 + (t2 - t1) * beta;
        if a.u > b.u {
            std::mem::swap(&mut a, &mut b);
        }
        for j in a.u..=b.u {
            let _ = image.set(j as usize, y as usize, color);
        }
    }
}

fn line(p0: Vec2i, p1: Vec2i, image: &mut TGAImage, color: TGAColor) {
    let mut x0 = p0.u;
    let mut y0 = p0.v;
    let mut x1 = p1.u;
    let mut y1 = p1.v;
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
