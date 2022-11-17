use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};

use crate::util::vector::Vector;

#[derive(Debug)]
pub struct MSTracer {
    samples_per_pixel: usize,
}

impl MSTracer {
    pub fn new(samples_per_pixel: usize) -> Self {
        Self { samples_per_pixel }
    }
}

impl RayTracer for MSTracer {
    fn raytrace<'r>(
        &self,
        x: usize,
        y: usize,
        datastructure: Arc<Mutex<Box<dyn DataStructure>>>,
        shader: Arc<dyn Shader>,
        camera: &Camera,
    ) -> Vector {
        let mut out = Vector::repeated(0f64);
        for _ in 0..self.samples_per_pixel {
            let ray = camera.generate_ray(x as f64, y as f64);
            out += shader.shade(ray, datastructure.clone()) / self.samples_per_pixel as f64;

            print!("\r{x}, {y} ");
            stdout().flush().unwrap();
        }

        out
    }
}
