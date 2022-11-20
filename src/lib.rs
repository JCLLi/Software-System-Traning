use crate::config::Config;

use log::LevelFilter;

pub mod config;
pub mod datastructure;
pub mod generator;
pub mod raytracer;
pub mod renderer;
pub mod scene;
pub mod shader;
pub mod util;

pub fn main() {
    simple_logging::log_to_stderr(LevelFilter::Info);

    Config::load("configurations/reference.yml")
        .unwrap()
        .run()
        .unwrap();
}
