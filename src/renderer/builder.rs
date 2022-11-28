use crate::datastructure::DataStructure;
use crate::generator::Generator;
use crate::raytracer::RayTracer;
use crate::renderer::Renderer;
use crate::shader::Shader;
use std::sync::{Arc, Mutex};

pub struct RendererBuilder<'a> {
    pub(self) generator: &'a dyn Generator,
}

pub struct RendererBuilderRaytracer<'a> {
    pub(self) generator: &'a dyn Generator,
    pub(self) raytracer: &'a dyn RayTracer,
}

pub struct RendererBuilderShader<'a> {
    pub(self) generator: &'a dyn Generator,
    pub(self) raytracer: &'a dyn RayTracer,
    pub(self) shader: &'a dyn Shader
}

pub struct RendererBuilderDatastructure<'a> {
    pub(self) generator: &'a dyn Generator,
    pub(self) raytracer: &'a dyn RayTracer,
    pub(self) shader: &'a dyn Shader,
    pub(self) datastructure: Arc<dyn DataStructure>,
}

impl<'a> RendererBuilder<'a> {
    pub fn new(generator: &'a dyn Generator) -> Self {
        Self { generator }
    }

    pub fn with_raytracer(self, raytracer: &'a dyn RayTracer) -> RendererBuilderRaytracer<'a> {
        RendererBuilderRaytracer {
            generator: self.generator,
            raytracer,
        }
    }
}

impl<'a> RendererBuilderRaytracer<'a> {
    pub fn with_shader(self, shader: &'a dyn Shader) -> RendererBuilderShader<'a> {
        RendererBuilderShader {
            generator: self.generator,
            raytracer: self.raytracer,
            shader,
        }
    }
}

impl<'a> RendererBuilderShader<'a> {
    pub fn with_datastructure(
        self,
        datastructure: Arc<dyn DataStructure>,
    ) -> RendererBuilderDatastructure<'a> {
        RendererBuilderDatastructure {
            generator: self.generator,
            raytracer: self.raytracer,
            shader: self.shader,
            datastructure,
        }
    }
}

impl<'a> RendererBuilderDatastructure<'a> {
    //changes here
    pub fn build(self) -> Renderer<'a> {
        Renderer::new(
            self.generator,
            self.raytracer,
            self.shader,
            self.datastructure,
        )
    }
}
