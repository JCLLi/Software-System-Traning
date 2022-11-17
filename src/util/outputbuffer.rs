use crate::util::color::Color;
use crate::util::vector::Vector;
use bmp::{px, Image, Pixel};
use std::fs::File;
use std::io::Write;

use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct OutputBuffer {
    buffer: Vec<Vec<Vector>>,
    backup_location: PathBuf,
}

impl OutputBuffer {
    pub fn new(backup_location: impl AsRef<Path>) -> Self {
        Self {
            buffer: Vec::new(),
            backup_location: backup_location.as_ref().to_path_buf(),
        }
    }

    pub fn presize(&mut self, width: usize, height: usize) {
        self.buffer.clear();
        self.buffer.reserve(height);
        for i in 0..height {
            self.buffer.push(Vec::new());
            self.buffer[i].resize_with(width, Default::default)
        }
    }

    pub fn with_size(width: usize, height: usize, backup_location: impl AsRef<Path>) -> Self {
        let mut res = Self::new(backup_location);
        res.presize(width, height);
        res
    }

    pub fn from_buffer(buffer: Vec<Vec<Vector>>, backup_location: impl AsRef<Path>) -> Self {
        Self {
            buffer,
            backup_location: backup_location.as_ref().to_path_buf(),
        }
    }

    pub fn to_bmp(&self) -> Image {
        let height = self.buffer.len();
        let width = if height > 0 { self.buffer[0].len() } else { 0 };

        let mut img = Image::new(width as u32, height as u32);

        for (x, y) in img.coordinates() {
            let color: &Color = &self.buffer[y as usize][x as usize].into();
            img.set_pixel(x, y, px!(color.r, color.g, color.b));
        }

        img
    }

    pub fn set_at(&mut self, x: usize, y: usize, color: Vector) {
        self.buffer[y][x] = color;

        let mut f = File::create(self.backup_location.clone()).unwrap();
        for row in &self.buffer {
            for column in row {
                write!(f, "{}, {}, {};", column.x, column.y, column.z).unwrap();
                f.flush().unwrap();
            }
            writeln!(f).unwrap();
        }
    }
}
