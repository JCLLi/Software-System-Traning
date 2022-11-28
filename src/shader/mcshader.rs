use crate::datastructure::DataStructure;
use crate::shader::{diffuse, emittance, Shader};
use crate::util::ray::Ray;
use crate::util::vector::Vector;
use std::sync::{Arc, Mutex};
use crate::datastructure::intersection::Intersection;

#[derive(Debug)]
pub struct McShader;

impl McShader {
    pub fn shade_internal<'a>(
        &self,
        ray: &Ray,
        depth: usize,
        datastructure: Arc<dyn DataStructure>,
        intersection: &Option<Intersection>,
    ) -> Vector {
        if let Some(intersection_ref) = intersection {
            let hit_pos = intersection_ref.hit_pos();
            let part_emi = emittance(intersection_ref);
            let indirect = if depth > 0 {
                let bounce_direction =
                    Vector::point_on_hemisphere().rotated(intersection_ref.triangle.normal());
                let bounce_ray = Ray::new(hit_pos, bounce_direction);
                let indirect_light =
                    //pass by reference without the Box<> wrapping around
                    self.shade_internal(&bounce_ray, depth - 1, datastructure.clone(), &datastructure.intersects(&bounce_ray));
                // println!("We are at depth {}, and the indirect light value is {} {} {}", depth, indirect_light.x, indirect_light.y, indirect_light.z);
                indirect_light * diffuse(intersection_ref, hit_pos, hit_pos + bounce_direction)
            } else {
                Vector::repeated(0f64)
            };

            return indirect * 2. + part_emi;
        }
        else {
            return Vector::repeated(0f64);
        }
    }
}

impl Shader for McShader {
    fn shade<'a> (&self, ray: &Ray, datastructure: Arc<dyn DataStructure>, intersection: &Option<Intersection>) -> Vector {
        self.shade_internal(ray, 4, datastructure.clone(), intersection)
    }
}
