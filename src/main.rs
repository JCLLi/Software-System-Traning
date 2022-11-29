#![allow(unused)]

use crate::config::Config;
use log::LevelFilter;

mod config;
mod datastructure;
mod generator;
mod raytracer;
mod renderer;
mod scene;
mod shader;
mod util;

fn main() {
    simple_logging::log_to_stderr(LevelFilter::Info);

    Config::load("configurations/reference.yml")
        .unwrap()
        .run()
        .unwrap();
}