use image::{GenericImageView, ImageError, RgbImage};
use std::path::Path;

mod textureatlas;

use crate::scene::texturecoordinate::TextureCoordinate;
use crate::util::vector::Vector;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::{fmt, io};
pub use textureatlas::{TextureAtlas, TextureAtlasBuilder};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TextureError {
    #[error(transparent)]
    ImageError(#[from] ImageError),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("invalid filename")]
    FileName,
}

#[derive(Clone)]
pub struct Texture {
    image: RgbImage,
    size: (usize, usize),
}

impl Debug for Texture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Texture")
    }
}

impl Texture {
    pub fn new(filename: impl AsRef<Path>) -> Result<Self, TextureError> {
        let mut f = File::open(filename)?;
        let mut reader = BufReader::new(f);
        let mut buf = [0; 20];
        let mut image_buffer = Vec::with_capacity(0);
        while reader.read_exact(&mut buf).is_ok() {
            for i in buf {
                image_buffer.push(i);
            }
        }

        let image = image::load_from_memory(&image_buffer).map_err(TextureError::ImageError)?;
        let dimensions = image.dimensions();

        Ok(Self {
            image: image.into_rgb8(),
            size: (dimensions.0 as usize, dimensions.1 as usize),
        })
    }

    pub fn at(&self, coord: TextureCoordinate) -> Vector {
        let x = (coord.u * self.size.0 as f64) as u32;
        let y = (self.size.1 - (coord.v * self.size.1 as f64) as usize) as u32;

        let rgb = self.image.get_pixel(x, y);

        Vector::new(
            rgb.0[0] as f64 / 255.,
            rgb.0[1] as f64 / 255.,
            rgb.0[2] as f64 / 255.,
        )
    }
}
