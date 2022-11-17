use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;
use crate::util::vector::Vector;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

pub mod basic;
pub mod threaded;

type Callback<'a> = (dyn Fn(usize, usize) -> Vector + Sync + 'a);

/// A generator is a struct that simply iterates over all x-y coordinates in the output image,
/// and calls generate(x, y) on it. After all pixels are iterated it collects all data
/// into an `Outputbuffer`.
///
/// This is important to be it's own subsystem because this iteration can be done in many ways
/// such as multithreaded, singlethreaded, or even spread over multiple machines.
pub trait Generator: Debug {
    fn generate_internal(
        &self,
        raytracer: Arc<dyn RayTracer>,
        datastructure: Arc<Mutex<Box<dyn DataStructure>>>,
        shader: Arc<dyn Shader>,
        camera: &Camera,
    ) -> OutputBuffer {
        self.generate(camera, &|x, y| {
            raytracer.raytrace(x, y, datastructure.clone(), shader.clone(), camera)
        })
    }

    fn generate(&self, camera: &Camera, callback: &Callback) -> OutputBuffer;
}
