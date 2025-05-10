use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom, Write},
    ops::Index,
};

#[derive(Default)]
#[repr(packed)]
pub struct TGAHeader {
    id_length: u8,
    color_map_type: u8,
    data_type_code: u8,
    color_map_origin: u16,
    color_map_length: u16,
    color_map_depth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    image_descriptor: u8,
}

impl TGAHeader {
    pub fn new() -> Self {
        TGAHeader {
            id_length: 0,
            color_map_type: 0,
            data_type_code: 0,
            color_map_origin: 0,
            color_map_length: 0,
            color_map_depth: 0,
            x_origin: 0,
            y_origin: 0,
            width: 0,
            height: 0,
            bits_per_pixel: 0,
            image_descriptor: 0,
        }
    }
    pub fn from_file(mut file: &File) -> Self {
        let mut id_length = [0u8; 1];
        let mut color_map_type = [0u8; 1];
        let mut data_type_code = [0u8; 1];
        let mut color_map_origin = [0u8; 2];
        let mut color_map_length = [0u8; 2];
        let mut color_map_depth = [0u8; 1];
        let mut x_origin = [0u8; 2];
        let mut y_origin = [0u8; 2];
        let mut width = [0u8; 2];
        let mut height = [0u8; 2];
        let mut bits_per_pixel = [0u8; 1];
        let mut image_descriptor = [0u8; 1];

        file.read_exact(&mut id_length)
            .expect("Error while reading id_length");
        file.read_exact(&mut color_map_type)
            .expect("Error while reading color_map_type");
        file.read_exact(&mut data_type_code)
            .expect("Error while reading id_length");
        file.read_exact(&mut color_map_origin)
            .expect("Error while reading color_map_origin");
        file.read_exact(&mut color_map_length)
            .expect("Error while reading color_map_length");
        file.read_exact(&mut color_map_depth)
            .expect("Error while reading color_map_depth");
        file.read_exact(&mut x_origin)
            .expect("Error while reading x_origin");
        file.read_exact(&mut y_origin)
            .expect("Error while reading y_origin");
        file.read_exact(&mut width)
            .expect("Error while reading width");
        file.read_exact(&mut height)
            .expect("Error while reading height");
        file.read_exact(&mut bits_per_pixel)
            .expect("Error while reading bits_per_pixel");
        file.read_exact(&mut image_descriptor)
            .expect("Error while reading image_descriptor");
        TGAHeader {
            id_length: id_length[0],
            color_map_type: color_map_type[0],
            data_type_code: data_type_code[0],
            color_map_origin: u16::from_le_bytes(color_map_origin),
            color_map_length: u16::from_le_bytes(color_map_length),
            color_map_depth: color_map_depth[0],
            x_origin: u16::from_le_bytes(x_origin),
            y_origin: u16::from_le_bytes(y_origin),
            width: u16::from_le_bytes(width),
            height: u16::from_le_bytes(height),
            bits_per_pixel: bits_per_pixel[0],
            image_descriptor: image_descriptor[0],
        }
    }
}

#[derive(Clone, Copy)]
pub struct TGAColor {
    bgra: [u8; 4],
    bytespp: u8,
}

impl TGAColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            bgra: [b, g, r, a],
            bytespp: 4,
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            bgra: [b, g, r, 255],
            bytespp: 4,
        }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            bgra: [b, g, r, a],
            bytespp: 4,
        }
    }
    pub fn from_hex(hex: u32) -> Self {
        TGAColor::from_rgba(
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
            255,
        )
    }
    pub fn from_bpp(bpp: u8) -> Self {
        TGAColor {
            bgra: [0; 4],
            bytespp: bpp,
        }
    }
}

impl Index<usize> for TGAColor {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bgra[index]
    }
}

pub enum Format {
    Grayscale = 1,
    RGB = 3,
    RGBA = 4,
}

impl Format {
    pub fn from_bpp(bpp: u8) -> Option<Self> {
        match bpp {
            1 => Some(Format::Grayscale),
            3 => Some(Format::RGB),
            4 => Some(Format::RGBA),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct TGAImage {
    w: i32,
    h: i32,
    pub bpp: u8,
    data: Vec<u8>,
}

impl TGAImage {
    pub fn new(width: i32, height: i32, format: Format) -> Self {
        let bpp = match format {
            Format::Grayscale => 1,
            Format::RGB => 3,
            Format::RGBA => 4,
        };
        let data_len = (width * height * bpp as i32) as usize;
        TGAImage {
            w: width,
            h: height,
            bpp: bpp,
            data: vec![0; data_len],
        }
    }

    pub fn from_tga_file(filename: &str) -> Option<TGAImage> {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("can't open file {}", filename);
                return None;
            }
        };

        let header = TGAHeader::from_file(&file);
        let width = header.width;
        let height = header.height;
        let bytespp = header.bits_per_pixel >> 3;
        let nbytes = (width as usize) * (height as usize) * (bytespp as usize);

        if header.id_length > 0 {
            file.seek(SeekFrom::Current(header.id_length as i64)).ok()?;
        }

        let data = match header.data_type_code {
            2 => {
                let mut buf = vec![0u8; nbytes];
                file.read_exact(&mut buf).ok()?;
                buf
            }
            10 => TGAImage::load_rle_data(&mut file, width as usize, height as usize, bytespp)?,
            other => {
                eprintln!("unsupported data type code: {}", other);
                return None;
            }
        };

        Some(TGAImage {
            h: height as i32,
            w: width as i32,
            bpp: bytespp,
            data: data,
        })
    }

    fn load_rle_data(file: &mut File, width: usize, height: usize, bpp: u8) -> Option<Vec<u8>> {
        let pixel_size = bpp as usize;
        let mut data = Vec::with_capacity(width * height * pixel_size);
        let mut pixels_read = 0;
        let total_pixels = width * height;

        while pixels_read < total_pixels {
            let mut header = [0u8; 1];
            file.read_exact(&mut header).ok()?;
            let chunk_header = header[0];

            let count = (chunk_header & 0x7F) + 1;
            if chunk_header & 0x80 != 0 {
                let mut pixel = vec![0u8; pixel_size];
                file.read_exact(&mut pixel).ok()?;
                for _ in 0..count {
                    data.extend(&pixel);
                    pixels_read += 1;
                }
            } else {
                for _ in 0..count {
                    let mut pixel = vec![0u8; pixel_size];
                    file.read_exact(&mut pixel).ok()?;
                    data.extend(&pixel);
                    pixels_read += 1;
                }
            }
        }

        Some(data)
    }

    fn flip_vertically(&mut self) {}

    fn flip_horizontally(&mut self) {}

    pub fn width(&self) -> i32 {
        self.w
    }

    pub fn height(&self) -> i32 {
        self.h
    }

    pub fn set(&mut self, x: usize, y: usize, color: TGAColor) -> Result<(), String> {
        if x >= self.w as usize || y >= self.h as usize {
            return Err("Coordinates out of bounds".to_string());
        }

        let bpp = self.bpp as usize;
        let index = (x + y * self.w as usize) * bpp;

        if index + bpp > self.data.len() {
            return Err("Index exceeds data buffer length".to_string());
        }

        self.data[index..index + bpp].copy_from_slice(&color.bgra[..bpp]);
        Ok(())
    }

    pub fn get(&self, x: i32, y: i32) -> Option<TGAColor> {
        if x < 0 || y < 0 || x >= self.w || y >= self.h || self.data.is_empty() {
            return None;
        }

        let bytespp = self.bpp as usize;
        let idx = (x + y * self.w) as usize * bytespp;

        if idx + bytespp > self.data.len() {
            return None;
        }

        let mut color = TGAColor {
            bgra: [0, 0, 0, 0],
            bytespp: self.bpp,
        };
        for i in 0..bytespp {
            color.bgra[i] = self.data[idx + i];
        }

        Some(color)
    }

    pub fn unload_rle_data(&self, out: &mut dyn Write) -> io::Result<()> {
        const MAX_CHUNK_LENGTH: u8 = 128;
        let npixels = (self.w * self.h) as usize;
        let mut curpix = 0;

        while curpix < npixels {
            let chunkstart = curpix * self.bpp as usize;
            let mut curbyte = curpix * self.bpp as usize;
            let mut run_length = 1;
            let mut raw = true;

            while curpix + run_length < npixels && run_length < MAX_CHUNK_LENGTH as usize {
                let mut succ_eq = true;
                for t in 0..self.bpp as usize {
                    if self.data[curbyte + t] != self.data[curbyte + t + self.bpp as usize] {
                        succ_eq = false;
                        break;
                    }
                }

                curbyte += self.bpp as usize;
                if run_length == 1 {
                    raw = !succ_eq;
                }
                if raw && succ_eq {
                    run_length -= 1;
                    break;
                }
                if !raw && !succ_eq {
                    break;
                }

                run_length += 1;
            }

            curpix += run_length;

            out.write_all(&[if raw {
                run_length as u8 - 1
            } else {
                run_length as u8 + 127
            }])?;

            if raw {
                out.write_all(&self.data[chunkstart..chunkstart + run_length * self.bpp as usize])?;
            } else {
                out.write_all(&self.data[chunkstart..chunkstart + self.bpp as usize])?;
            }
        }

        Ok(())
    }

    pub fn write_tga_file(&self, filename: &str, vflip: bool, rle: bool) -> io::Result<()> {
        let developer_area_ref: [u8; 4] = [0, 0, 0, 0];
        let extension_area_ref: [u8; 4] = [0, 0, 0, 0];
        let footer: &[u8; 18] = b"TRUEVISION-XFILE.\0";

        let mut out = File::create(filename).expect("Cant open file");
        let header = TGAHeader {
            bits_per_pixel: self.bpp << 3,
            width: self.width() as u16,
            height: self.height() as u16,
            data_type_code: match (self.bpp, rle) {
                (0, true) => 11,
                (0, false) => 3,
                (_, true) => 10,
                (_, false) => 2,
            },
            image_descriptor: if vflip { 0x00 } else { 0x20 },
            ..Default::default()
        };
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                &header as *const _ as *const u8,
                std::mem::size_of::<TGAHeader>(),
            )
        };
        out.write_all(header_bytes)?;

        if !rle {
            out.write_all(&self.data)?;
        } else {
            self.unload_rle_data(&mut out)?;
        }

        out.write_all(&developer_area_ref)?;
        out.write_all(&extension_area_ref)?;
        out.write_all(footer)?;

        Ok(())
    }
}
