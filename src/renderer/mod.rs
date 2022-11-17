use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;
use std::sync::{Arc, Mutex};

mod builder;

use crate::generator::Generator;
pub use builder::RendererBuilder;

#[derive(Debug)]
pub struct Renderer {
    generator: Arc<dyn Generator>,
    raytracer: Arc<dyn RayTracer>,
    shader: Arc<dyn Shader>,
    datastructure: Arc<Mutex<Box<dyn DataStructure>>>,
}

impl Renderer {
    pub(self) fn new(
        generator: Arc<dyn Generator>,
        raytracer: Arc<dyn RayTracer>,
        shader: Arc<dyn Shader>,
        datastructure: Arc<Mutex<Box<dyn DataStructure>>>,
    ) -> Self {
        Self {
            generator,
            raytracer,
            shader,
            datastructure,
        }
    }

    pub fn render(&self, camera: &Camera) -> OutputBuffer {
        self.generator.generate_internal(
            self.raytracer.clone(),
            self.datastructure.clone(),
            self.shader.clone(),
            camera,
        )
    }
}
