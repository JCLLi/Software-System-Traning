use crate::datastructure::DataStructure;
use crate::shader::{diffuse, emittance, Shader};
use crate::util::ray::Ray;
use crate::util::vector::Vector;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct McShader;

impl McShader {
    pub fn shade_internal(
        &self,
        ray: Box<Ray>,
        depth: usize,
        datastructure: Arc<Mutex<Box<dyn DataStructure>>>,
    ) -> Vector {
        let intersection =
            if let Some(intersection) = datastructure.lock().unwrap().intersects(*ray.clone()) {
                intersection
            } else {
                return Vector::repeated(0f64);
            };

        let hit_pos = intersection.hit_pos();

        let part_emi = emittance(&intersection);

        let indirect = if depth > 0 {
            let bounce_direction =
                Vector::point_on_hemisphere().rotated(intersection.triangle.normal());
            let bounce_ray = Ray::new(hit_pos, bounce_direction);
            let indirect_light =
                self.shade_internal(Box::new(bounce_ray), depth - 1, datastructure);
            indirect_light * diffuse(&intersection, hit_pos, hit_pos + bounce_direction)
        } else {
            Vector::repeated(0f64)
        };

        indirect * 2. + part_emi
    }
}

impl Shader for McShader {
    fn shade(&self, ray: Box<Ray>, datastructure: Arc<Mutex<Box<dyn DataStructure>>>) -> Vector {
        self.shade_internal(ray, 4, datastructure)
    }
}
