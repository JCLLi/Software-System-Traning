use crate::util::ray::Ray;
use crate::util::vector::Vector;
use std::f64;

#[derive(Debug)]
pub struct Camera {
    pub pos: Vector,
    pub direction: Vector,
    pub width: usize,
    pub height: usize,
    pub fov: f64,
    pub inf_width: f64,
    pub inf_height: f64,
    pub angle: f64,
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn new(pos: Vector, direction: Vector, width: usize, height: usize, fov: f64) -> Self {
        let inf_width = 1f64 / (width as f64);
        let inf_height = 1f64 / (height as f64);
        let angle = (f64::consts::PI * 0.5f64 * fov / 180f64).tan();
        let aspect_ratio = width as f64 / height as f64;
        let internal_direction = direction.rotated(Vector::new(0., 0., 1.)).unit();

        Self {
            pos,
            direction: internal_direction,
            width,
            height,
            fov,
            inf_width,
            inf_height,
            angle,
            aspect_ratio,
        }
    }

    pub fn generate_ray(&self, x: f64, y: f64) -> Box<Ray> {
        let xdir = (2f64 * x as f64 * self.inf_width - 1f64) * self.angle * self.aspect_ratio;
        let ydir = (1f64 - 2f64 * y as f64 * self.inf_height) * self.angle;

        let raydir = Vector::new(xdir, ydir, -1f64)
            .rotated(Vector::new(0., 0., 1.))
            .rotated(self.direction)
            .rotated(Vector::new(0., 0., -1.));

        Box::new(Ray::new(self.pos, raydir))
    }
}
