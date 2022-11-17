use crate::datastructure::DataStructure;
use crate::generator::Generator;
use crate::raytracer::RayTracer;
use crate::renderer::Renderer;
use crate::shader::Shader;
use std::sync::{Arc, Mutex};

pub struct RendererBuilder {
    pub(self) generator: Arc<dyn Generator>,
}

pub struct RendererBuilderRaytracer {
    pub(self) generator: Arc<dyn Generator>,
    pub(self) raytracer: Arc<dyn RayTracer>,
}

pub struct RendererBuilderShader {
    pub(self) generator: Arc<dyn Generator>,
    pub(self) raytracer: Arc<dyn RayTracer>,
    pub(self) shader: Arc<dyn Shader>,
}

pub struct RendererBuilderDatastructure {
    pub(self) generator: Arc<dyn Generator>,
    pub(self) raytracer: Arc<dyn RayTracer>,
    pub(self) shader: Arc<dyn Shader>,
    pub(self) datastructure: Arc<Mutex<Box<dyn DataStructure>>>,
}

impl RendererBuilder {
    pub fn new(generator: Arc<dyn Generator>) -> Self {
        Self { generator }
    }

    pub fn with_raytracer(self, raytracer: Arc<dyn RayTracer>) -> RendererBuilderRaytracer {
        RendererBuilderRaytracer {
            generator: self.generator,
            raytracer,
        }
    }
}

impl RendererBuilderRaytracer {
    pub fn with_shader(self, shader: Arc<dyn Shader>) -> RendererBuilderShader {
        RendererBuilderShader {
            generator: self.generator,
            raytracer: self.raytracer,
            shader,
        }
    }
}

impl RendererBuilderShader {
    pub fn with_datastructure(
        self,
        datastructure: Arc<Mutex<Box<dyn DataStructure>>>,
    ) -> RendererBuilderDatastructure {
        RendererBuilderDatastructure {
            generator: self.generator,
            raytracer: self.raytracer,
            shader: self.shader,
            datastructure,
        }
    }
}

impl RendererBuilderDatastructure {
    pub fn build(self) -> Renderer {
        Renderer::new(
            self.generator,
            self.raytracer,
            self.shader,
            self.datastructure,
        )
    }
}
