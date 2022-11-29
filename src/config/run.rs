use std::borrow::Borrow;
use crate::config::error::ConfigError;
use crate::config::{Config, GeneratorConfig};
use crate::datastructure::bvh::KDTreeDataStructure;
use crate::datastructure::DataStructure;
use crate::generator::basic::BasicGenerator;
use crate::generator::threaded::ThreadedGenerator;
use crate::generator::Generator;
use crate::raytracer::mstracer::MSTracer;

use crate::renderer::RendererBuilder;
use crate::scene::scene::SceneBuilder;
use crate::shader::mcshader::McShader;

use crate::util::camera::Camera;
use std::sync::{Arc, Mutex};

use std::path::PathBuf;

impl Config {
    pub fn run(self) -> Result<(), ConfigError> {
        //load_obj is fine, library
        let (model, mtls) = tobj::load_obj(self.general.scenename.as_ref())?;

        //texture path is fine, very simple function
        let scene = SceneBuilder::default()
            .texturepath(PathBuf::from(&self.general.texturepath))
            .build_from_tobj((model, mtls))?;

        //Arc seems redundant, Box might be fine
        let generator: Box<dyn Generator> = match self.generator {
            GeneratorConfig::Basic => Box::new(BasicGenerator),
            GeneratorConfig::Threaded { threads } => {
                Box::new(ThreadedGenerator::new(threads.get_cores()))
            }
        };

        let raytracer = Box::new(MSTracer::new(self.raytracer.samples_per_pixel));

        let shader = Box::new(McShader);

        //Mutex seems redundant as well
        let datastructure =
            Arc::new(KDTreeDataStructure::new(&scene));
        //All these Arc seems redundant
        let renderer = RendererBuilder::new(generator.as_ref())
            .with_raytracer(raytracer.as_ref())
            .with_shader(shader.as_ref())
            .with_datastructure(datastructure.clone())
            .build();

        let camera = Camera::new(
            self.camera.position,
            self.camera.direction,
            self.camera.width,
            self.camera.height,
            self.camera.fov,
        );

        dbg!(&renderer);

        renderer
            .render(&camera)
            .to_bmp()
            .save(self.general.outputname)?;

        Ok(())
    }
}
