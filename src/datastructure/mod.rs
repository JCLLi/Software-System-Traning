use crate::datastructure::intersection::Intersection;
use crate::util::ray::Ray;
use std::fmt::Debug;

pub mod bvh;
pub mod intersection;

/// A destructure is a struct that recieves a ray and returns whether or not the ray intersected,
/// and if so, where in the scene that intersection was by returning an `Intersection` struct.
pub trait DataStructure: Send + Sync + Debug {
    /// If a ray intersects multiple points in the scene, the intersects function must always
    /// return the intersection closest to the origin of the ray.
    fn intersects(&mut self, ray: Ray) -> Option<Intersection>;
}
