use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::geometry::Vec3f;

#[derive(Debug)]
pub struct Model {
    verts: Vec<Vec3f>,
    faces: Vec<Vec<i32>>,
}

impl Model {
    pub fn new(filename: &str) -> io::Result<Self> {
        let mut verts = Vec::new();
        let mut faces = Vec::new();

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
                            let mut indices = part.split('/');
                            if let Some(vertex_idx) = indices.next() {
                                if let Ok(mut idx) = vertex_idx.parse::<i32>() {
                                    idx -= 1;
                                    face.push(idx);
                                }
                            }
                        }
                        faces.push(face);
                    }
                    _ => {}
                }
            }
        }

        eprintln!("# v# {} f# {}", verts.len(), faces.len());
        Ok(Model { verts, faces })
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
