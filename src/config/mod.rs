use crate::config::corecount::ThreadCount;
use crate::config::error::ConfigError;
use crate::util::vector::Vector;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub mod corecount;
pub mod defaults;
pub mod error;
pub mod run;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    general: GeneralConfig,
    camera: CameraConfig,
    generator: GeneratorConfig,
    raytracer: RaytracerConfig,
}

#[derive(Serialize, Deserialize)]
pub struct RaytracerConfig {
    samples_per_pixel: usize,
}

#[derive(Serialize, Deserialize)]
pub enum GeneratorConfig {
    /// Don't use any multithreading
    #[serde(rename = "basic")]
    Basic,

    #[serde(rename = "threaded")]
    Threaded {
        /// The number of cores to use during the raytracing.
        threads: ThreadCount,
    },
}

#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Very small float value.
    /// Two floats that are closer together than this value will be equal.
    epsilon: f64,

    /// Filename of the scene that will render
    scenename: String,

    /// Filename of the generated bitmap
    outputname: String,

    /// Path to search for texture files
    texturepath: String,
}

#[derive(Serialize, Deserialize)]
pub struct CameraConfig {
    /// The position of the camera in 3d space
    position: Vector,

    /// The rotation of the camera in 3d space
    direction: Vector,

    /// The width of the image to be generated
    width: usize,
    /// The height of the image to be generated
    height: usize,

    /// The field of view of the camera
    fov: f64,
}

impl Config {
    pub fn dump(&self, filename: impl AsRef<Path>) -> Result<(), ConfigError> {
        let yamlstring = serde_yaml::to_string(self)?;

        fs::write(filename, yamlstring)?;

        Ok(())
    }

    pub fn load(filename: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let contents = fs::read(filename)?;

        Ok(serde_yaml::from_slice(&contents)?)
    }
}
