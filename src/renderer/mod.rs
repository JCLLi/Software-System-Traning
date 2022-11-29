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
pub struct Renderer<'a> {
    generator: &'a dyn Generator,
    raytracer: &'a dyn RayTracer,
    shader: &'a dyn Shader,
    datastructure: Arc<dyn DataStructure>,
}

impl<'a> Renderer<'a> {
    pub(self) fn new(
        generator: &'a dyn Generator,
        raytracer: &'a dyn RayTracer,
        shader: &'a dyn Shader,
        datastructure: Arc<dyn DataStructure>,
    ) -> Self {
        Self {
            generator,
            raytracer,
            shader,
            datastructure,
        }
    }

    //All these clones seem redundant, find a way to eliminate them
    pub fn render(&self, camera: &Camera) -> OutputBuffer {
        self.generator.generate_internal(
            self.raytracer,
            self.datastructure.clone(),
            self.shader,
            camera,
        )
    }
}
