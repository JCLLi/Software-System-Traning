use crate::scene::triangle::Triangle;
use crate::util::ray::Ray;
use crate::util::vector::Vector;
use std::f64::EPSILON;
use std::sync::Arc;

#[derive(Debug)]
/// Represents the intersection point between a ray and a triangle.
pub struct Intersection {
    /// The original ray that was used to get this intersection.
    pub ray: Box<Ray>,
    /// the uv (barycentric) coordinates of the hitpoint on the triangle.
    pub uv: (f64, f64),
    /// The distance from the ray origin to the hitpoint on the triangle.
    pub t: f64,
    /// The triangle that was hit by the ray.
    pub triangle: Arc<Triangle>,
}

impl Intersection {
    /// Returns a point in 3d space where the hit occurred.
    pub fn hit_pos(&self) -> Vector {
        self.ray.origin + self.ray.direction * (self.t - EPSILON)
    }
}
