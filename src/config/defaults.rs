use crate::config::{CameraConfig, GeneralConfig, GeneratorConfig, RaytracerConfig};
use crate::util::vector::Vector;

impl Default for RaytracerConfig {
    fn default() -> Self {
        Self {
            samples_per_pixel: 200,
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            epsilon: 0.00001,
            scenename: "test".to_string(),
            outputname: "render.bmp".to_string(),
            texturepath: "scenes".to_string(),
        }
    }
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            position: Vector::default(),
            direction: Vector::new(0.0, 0.0, -1.0),
            width: 1000,
            height: 1000,
            fov: 60.,
        }
    }
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        GeneratorConfig::Basic
    }
}
