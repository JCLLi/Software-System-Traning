use crate::datastructure::intersection::Intersection;
use crate::datastructure::DataStructure;
use crate::scene::texturecoordinate::TextureCoordinate;
use crate::util::ray::Ray;
use crate::util::vector::Vector;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

pub mod mcshader;

/// A shader in the rusttracer codebase means a piece of code that takes a ray,
/// and asks the `datastructure` where it lands. Based on the `Intersection` struct
/// it gets back, it can give a color to a pixel. A shader can query the `datastructure`
/// multiple times to achieve such things as reflection, refraction, and other effects.
pub trait Shader: Send + Sync + Debug {
    fn shade(&self, ray: Box<Ray>, datastructure: Arc<Mutex<Box<dyn DataStructure>>>) -> Vector;
}

pub fn ambient(intersection: &Intersection) -> Vector {
    let texture = if let Some(texture) = intersection.triangle.mesh.material.ambient_texture.clone()
    {
        let coord = map_uv(intersection);

        texture.at(coord)
    } else {
        Vector::new(1., 1., 1.)
    };

    intersection.triangle.material().ambient * texture
}

pub fn emittance(intersection: &Intersection) -> Vector {
    let texture = if let Some(texture) = intersection
        .triangle
        .mesh
        .material
        .emittance_texture
        .clone()
    {
        let coord = map_uv(intersection);

        texture.at(coord)
    } else {
        Vector::new(1., 1., 1.)
    };

    intersection.triangle.material().emittance * texture
}

pub fn map_uv(intersection: &Intersection) -> TextureCoordinate {
    let texa = intersection.triangle.texture_a();
    let texb = intersection.triangle.texture_b();
    let texc = intersection.triangle.texture_c();

    let e1 = texc - texa;
    let e2 = texb - texa;

    texa.to_owned() + (e1 * intersection.uv.1) + (e2 * intersection.uv.0)
}

pub fn diffuse(intersection: &Intersection, hit_pos: Vector, light_pos: Vector) -> Vector {
    let triangle = intersection.triangle.clone();

    let texture = if let Some(texture) = intersection.triangle.mesh.material.diffuse_texture.clone()
    {
        let coord = map_uv(intersection);

        texture.at(coord)
    } else {
        Vector::new(1., 1., 1.)
    };

    let light_dir = (light_pos - hit_pos).unit();
    light_dir.dot(triangle.normal()).max(0.) * triangle.material().diffuse * texture
}

pub fn specular(
    intersection: &Intersection,
    hit_pos: Vector,
    light_pos: Vector,
    cam_pos: Vector,
) -> Vector {
    let texture =
        if let Some(texture) = intersection.triangle.mesh.material.specular_texture.clone() {
            let coord = map_uv(intersection);

            texture.at(coord)
        } else {
            Vector::new(1., 1., 1.)
        };

    let triangle = intersection.triangle.clone();

    let light_dir = (light_pos - hit_pos).unit();
    let reflec = 2f64 * (triangle.normal().dot(light_dir)) * triangle.normal() - light_dir;
    let spec = 0f64.max((cam_pos - hit_pos).unit().dot(reflec));

    spec.powf(triangle.material().shininess) * triangle.material().specular * texture
}
