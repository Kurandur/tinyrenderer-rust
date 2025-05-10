use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::geometry::{Vec2f, Vec3f};
use crate::tga::TGAImage;

#[derive(Debug)]
pub struct Model {
    verts: Vec<Vec3f>,
    faces: Vec<Vec<Vec3i>>,
    norms: Vec<Vec3f>,
    uv: Vec<Vec2f>,
    diffusemap: Option<TGAImage>,
}

impl Model {
    pub fn new(filename: &str) -> io::Result<Self> {
        let mut verts = Vec::new();
        let mut faces = Vec::new();
        let mut norms = Vec::new();
        let mut uv = Vec::new();

        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            if let Some(first) = parts.next() {
                match first {
                    "v" => {
                        let coords: Vec<f32> = parts
                            .take(3)
                            .filter_map(|x| x.parse::<f32>().ok())
                            .collect();
                        if coords.len() == 3 {
                            verts.push(Vec3f::new(coords[0], coords[1], coords[2]));
                        }
                    }
                    "f" => {
                        let mut face = Vec::new();
                        for part in parts {
                            let indices: Vec<_> = part.split('/').collect();
                            if indices.len() >= 3 {
                                let v_idx = indices[0].parse::<i32>().unwrap_or(0) - 1;
                                let vt_idx = indices[1].parse::<i32>().unwrap_or(0) - 1;
                                let vn_idx = indices[2].parse::<i32>().unwrap_or(0) - 1;
                                face.push(Vec3i::new(v_idx, vt_idx, vn_idx));
                            }
                        }
                        faces.push(face);
                    }
                    "vn" => {
                        let coords: Vec<f32> = parts
                            .take(3)
                            .filter_map(|x| x.parse::<f32>().ok())
                            .collect();
                        if coords.len() == 3 {
                            norms.push(Vec3f::new(coords[0], coords[1], coords[2]));
                        }
                    }
                    "vt" => {
                        let coords: Vec<f32> = parts
                            .take(2)
                            .filter_map(|x| x.parse::<f32>().ok())
                            .collect();
                        if coords.len() == 2 {
                            uv.push(Vec2f::new(coords[0], coords[1]));
                        }
                    }
                    _ => {}
                }
            }
        }
        eprintln!(
            "# v# {} f# {} n# {} uv# {}",
            verts.len(),
            faces.len(),
            norms.len(),
            uv.len()
        );
        Ok(Model {
            verts,
            faces,
            norms,
            uv,
            diffusemap: None,
        })
    }

    pub fn nverts(&self) -> usize {
        self.verts.len()
    }

    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }

    pub fn vert(&self, idx: usize) -> Vec3f {
        self.verts[idx]
    }

    pub fn face(&self, idx: usize) -> &Vec<i32> {
        &self.faces[idx]
    }
}
