use std::{env, mem::swap};

use rand::Rng;
use tinyrenderer_rust::{
    geometry::{Matrix, Vec2f, Vec2i, Vec3f, Vec3i},
    model::Model,
    tga::{Format, TGAColor, TGAImage},
};

const IMAGE_WIDTH: i32 = 800;
const IMAGE_HEIGHT: i32 = 800;
const DEPTH: i32 = 255;

fn main() {
    let args: Vec<String> = env::args().collect();
    let model_path = if args.len() == 2 {
        &args[1]
    } else {
        "obj/dude.obj"
    };
    let mut model = Model::new(model_path).expect("Failed to load model");
    model.load_texture("obj/dude_diffuse.tga");

    // colors
    let red = TGAColor::from_rgb(255, 0, 0);
    let green = TGAColor::from_rgb(0, 255, 0);
    let white = TGAColor::from_rgb(255, 255, 255);
    let yellow = TGAColor::from_rgb(255, 255, 0);

    // image and camera setup
    let mut image = TGAImage::new(IMAGE_WIDTH, IMAGE_HEIGHT, Format::RGB);
    let mut zbuffer = vec![i32::MIN; IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize];
    let light_dir = Vec3f::new(0.0, 0.0, -1.0);
    let camera = Vec3f::new(0.0, 0.0, 3.0);
    let mut projection = Matrix::identity(4);
    let viewport = Matrix::new_from_viewport(
        (IMAGE_WIDTH / 8) as usize,
        (IMAGE_HEIGHT / 8) as usize,
        (IMAGE_WIDTH * 3 / 4) as usize,
        (IMAGE_HEIGHT * 3 / 4) as usize,
    );
    projection[3][2] = -1.0 / camera.z;

    // draw the model
    for i in 0..model.nfaces() {
        let face = model.face(i);
        let mut screen_coords: Vec<Vec3i> = Vec::with_capacity(3);
        let mut world_coords: Vec<Vec3f> = Vec::with_capacity(3);

        for j in 0..3 {
            let idx = face[j].x as usize;
            let v = model.vert(idx);
            screen_coords.push(
                ((&viewport * &projection) * Matrix::new_from_vector(v))
                    .to_vector()
                    .into(),
            );
            world_coords.push(v);
        }
        let mut n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);
        n.normalize();
        let intensity = n * light_dir;
        if intensity > 0.0 {
            let mut uv: Vec<Vec2i> = Vec::with_capacity(3);
            for k in 0..3 {
                uv.push(model.uv(i, k));
            }
            triangle_scanline(
                screen_coords[0],
                screen_coords[1],
                screen_coords[2],
                uv[0],
                uv[1],
                uv[2],
                &mut zbuffer,
                &mut image,
                &model,
                intensity,
            );
        }
    }

    image.write_tga_file("output.tga", true, true).unwrap();
}

fn world_to_screen(v: Vec3f, width: usize, height: usize) -> Vec3f {
    let x = ((v.x + 1.0) * (width as f32) / 2.0 + 0.5).floor() as f32;
    let y = ((v.y + 1.0) * (height as f32) / 2.0 + 0.5).floor() as f32;
    Vec3f::new(x, y, v.z)
}

fn barycentric(a: Vec3f, b: Vec3f, c: Vec3f, p: Vec3f) -> Vec3f {
    let mut s = [Vec3f::new(0.0, 0.0, 0.0); 2];
    for i in (0..2).rev() {
        s[i] = Vec3f::new(
            c.get(i) - a.get(i),
            b.get(i) - a.get(i),
            a.get(i) - p.get(i),
        );
    }

    let u = Vec3f::cross(s[0], s[1]);
    if u.z.abs() > 1e-2 {
        return Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
    }

    Vec3f::new(-1.0, 1.0, 1.0)
}

fn triangle_raster(pts: Vec<Vec3f>, zbuffer: &mut [f32], image: &mut TGAImage, color: TGAColor) {
    let mut bbox_min = Vec2f::new(f32::MAX, f32::MAX);
    let mut bbox_max = Vec2f::new(f32::MIN, f32::MIN);

    let clamp = Vec2f::new((IMAGE_WIDTH - 1) as f32, (IMAGE_HEIGHT - 1) as f32);

    for i in 0..3 {
        for j in 0..2 {
            let val = match j {
                0 => pts[i].x,
                1 => pts[i].y,
                _ => unreachable!(),
            };
            let min_val = bbox_min.get(j).min(val).max(0.0);
            let max_val = bbox_max.get(j).max(val).min(clamp.get(j));
            bbox_min.set(j, min_val);
            bbox_max.set(j, max_val);
        }
    }
    let mut p = Vec3f::new(0.0, 0.0, 0.0);

    let min_x = bbox_min.x.floor() as i32;
    let max_x = bbox_max.x.ceil() as i32;
    let min_y = bbox_min.y.floor() as i32;
    let max_y = bbox_max.y.ceil() as i32;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            p.x = x as f32;
            p.y = y as f32;

            let bc_screen = barycentric(pts[0], pts[1], pts[2], p);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            p.z = 0.0;
            for i in 0..3 {
                let weight = match i {
                    0 => bc_screen.x,
                    1 => bc_screen.y,
                    2 => bc_screen.z,
                    _ => unreachable!(),
                };
                p.z += pts[i].z * weight;
            }

            let idx = (x + y * IMAGE_WIDTH as i32) as usize;
            if idx < zbuffer.len() && p.z > zbuffer[idx] {
                zbuffer[idx] = p.z;
                let _ = image.set(x as usize, y as usize, color);
            }
        }
    }
}

fn triangle_scanline(
    mut t0: Vec3i,
    mut t1: Vec3i,
    mut t2: Vec3i,
    mut uv0: Vec2i,
    mut uv1: Vec2i,
    mut uv2: Vec2i,
    zbuffer: &mut [i32],
    image: &mut TGAImage,
    model: &Model,
    intensity: f32,
) {
    if t0.y == t1.y && t0.y == t2.y {
        return;
    }

    if t0.y > t1.y {
        std::mem::swap(&mut t0, &mut t1);
        std::mem::swap(&mut uv0, &mut uv1);
    }
    if t0.y > t2.y {
        std::mem::swap(&mut t0, &mut t2);
        std::mem::swap(&mut uv0, &mut uv2);
    }
    if t1.y > t2.y {
        std::mem::swap(&mut t1, &mut t2);
        std::mem::swap(&mut uv1, &mut uv2);
    }
    let total_height = (t2.y - t0.y) as i32;
    for i in 0..total_height {
        let second_half = i > (t1.y - t0.y) || t1.y == t0.y;
        let segment_height = if second_half {
            t2.y - t1.y
        } else {
            t1.y - t0.y
        };
        let alpha = i as f32 / total_height as f32;
        let beta = if second_half {
            (i - (t1.y - t0.y)) as f32 / segment_height as f32
        } else {
            i as f32 / segment_height as f32
        };

        let mut a = t0 + (t2 - t0) * alpha;
        let mut b = if second_half {
            t1 + (t2 - t1) * beta
        } else {
            t0 + (t1 - t0) * beta
        };
        let mut uv_a = uv0 + (uv2 - uv0) * alpha;
        let mut uv_b = if second_half {
            uv1 + (uv2 - uv1) * beta
        } else {
            uv0 + (uv1 - uv0) * beta
        };

        if a.x > b.x {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut uv_a, &mut uv_b);
        }

        for j in (a.x as usize)..=(b.x as usize) {
            let phi = if b.x == a.x {
                1.0
            } else {
                ((j as i32 - a.x) as f32 / (b.x - a.x) as f32) as f32
            };
            let mut p = a + (b - a) * phi;
            let uv_p = uv_a + (uv_b - uv_a) * phi;
            p.x = j as i32;
            p.y = t0.y + i;

            let idx = (p.x + p.y * IMAGE_WIDTH) as usize;
            if idx < zbuffer.len() && zbuffer[idx] < p.z as i32 {
                zbuffer[idx] = p.z as i32;
                let color = model.diffuse(uv_p);
                let _ = image.set(
                    p.x as usize,
                    p.y as usize,
                    TGAColor::from_rgb(
                        (color[2] as f32 * intensity) as u8,
                        (color[1] as f32 * intensity) as u8,
                        (color[0] as f32 * intensity) as u8,
                    ),
                );
            }
        }
    }
}

fn line(p0: Vec3i, p1: Vec3i, image: &mut TGAImage, color: TGAColor) {
    let mut x0 = p0.x;
    let mut y0 = p0.y;
    let mut x1 = p1.x;
    let mut y1 = p1.y;
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
