use crate::datastructure::bvh::boundingbox::BoundingBox;
use crate::datastructure::bvh::boxintersection::BoxIntersection;
use crate::datastructure::bvh::node::BVHNode;
use crate::datastructure::intersection::Intersection;
use crate::datastructure::DataStructure;
use crate::scene::scene::Scene;
use crate::scene::triangle::Triangle;
use crate::util::ray::Ray;

use log::debug;
//use core::num::dec2flt::rawfp::RawFloat;
use crate::util::consts::INTERSECTION_EPSILON;
use core::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

mod boundingbox;
mod boxintersection;
mod node;

pub struct KDTreeDataStructure {
    root: BVHNode,
}

impl Debug for KDTreeDataStructure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<KDTreeDataStructure...>")
    }
}


fn intersects_triangle(ray: &Ray, triangle: &Arc<Triangle>) -> Option<Intersection> {
    // let (Ta, Tb, Tc) = triangle.get_all();
    let aa = triangle.a();
    let edge1 = triangle.b() - aa;
    let edge2 = triangle.c() - aa;

    let h = ray.direction.cross(&edge2);
    let a = edge1.dot(&h);

    if -INTERSECTION_EPSILON < a && a < INTERSECTION_EPSILON {
        return None;
    }

    let f = 1f64 / a;

    let s = ray.origin - aa;
    let u = f * s.dot(&h);

    let q = s.cross(&edge1);
    let v = f * ray.direction.dot(&q);

    if u < 0f64 || u > 1f64 {
        return None;
    }

    if v < 0f64 || u + v > 1f64 {
        return None;
    }

    let t = f * edge2.dot(&q);
    if t < INTERSECTION_EPSILON {
        return None;
    }

    Some(Intersection {
        uv: (u, v),
        t,
        ray: Box::new(ray.clone()),
        triangle: triangle.clone(),
    })
}

impl KDTreeDataStructure {
    pub fn new(scene: &Scene) -> Self {
        debug!("Started building KD-Tree");
        let triangles: Vec<Arc<Triangle>> = scene.triangles().collect();
        debug!("Cached triangles locally");

        let root = BVHNode::new(triangles);
        println!("{}", root);

        Self { root }
    }

    fn intersect_internal(ray: &Ray, node: &BVHNode) -> Option<Intersection> {
        debug!("intersection {:?} {}", ray, node);
        match node {
            BVHNode::Leaf {
                bounding_box,
                triangles,
            } => {
                if intersects_boundingbox(bounding_box, &ray).is_some() {
                    let mut min = None;

                    for triangle in triangles {
                        if let Some(intersection) =
                            intersects_triangle(&ray, &triangle)
                        {
                            min = match min {
                                None => Some(intersection),
                                Some(i) if intersection.t < i.t => Some(intersection),
                                _ => min,
                            };
                        }
                    }

                    return min;
                }
                None
            }
            BVHNode::Node {
                bounding_box: _,
                left,
                right,
            } => {
                let dist_l = intersects_bhv(left, &ray);
                let dist_r = intersects_bhv(right, &ray);

                match (dist_l, dist_r) {
                    (None, None) => None,
                    (Some(_), None) => Self::intersect_internal(&ray, left),
                    (None, Some(_)) => Self::intersect_internal(&ray, right),
                    (Some(left_intersection), Some(right_intersection)) => {
                        if left_intersection.t < right_intersection.t {
                            let hit = Self::intersect_internal(&ray, left);
                            if let Some(intersection) = hit {
                                if left.includes_point(&intersection.hit_pos()) {
                                    return Some(intersection);
                                }
                            }
                            Self::intersect_internal(&ray, right)
                        } else {
                            let hit = Self::intersect_internal(&ray, right);
                            if let Some(intersection) = hit {
                                if right.includes_point(&intersection.hit_pos()) {
                                    return Some(intersection);
                                }
                            }
                            Self::intersect_internal(ray, left)
                        }
                    }
                }
            }
        }
    }
}

impl DataStructure for KDTreeDataStructure {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        Self::intersect_internal(&ray, &self.root)
    }
}

pub fn intersects_bhv(node: &BVHNode, ray: &Ray) -> Option<BoxIntersection> {
    match node {
        BVHNode::Leaf {
            bounding_box,
            triangles: _,
        } => intersects_boundingbox(bounding_box, &ray),
        BVHNode::Node {
            bounding_box,
            left: _,
            right: _,
        } => intersects_boundingbox(bounding_box, &ray),
    }
}

pub fn intersects_boundingbox(boundingbox: &BoundingBox, ray: &Ray) -> Option<BoxIntersection> {
    let tmin = (boundingbox.min.x - ray.origin.x) / ray.direction.x;
    let tmax = (boundingbox.max.x - ray.origin.x) / ray.direction.x;

    let (tmin, tmax) = if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    };

    let tymin = (boundingbox.min.y - ray.origin.y) / ray.direction.y;
    let tymax = (boundingbox.max.y - ray.origin.y) / ray.direction.y;

    let (tymin, tymax) = if tymin > tymax {
        (tymax, tymin)
    } else {
        (tymin, tymax)
    };

    if (tmin > tymax) || (tymin > tmax) {
        return None;
    }

    let tmin = if tymin > tmin { tymin } else { tmin };

    let tmax = if tymax < tmax { tymax } else { tmax };

    let tzmin = (boundingbox.min.z - ray.origin.z) / ray.direction.z;
    let tzmax = (boundingbox.max.z - ray.origin.z) / ray.direction.z;

    let (tzmin, tzmax) = if tzmin > tzmax {
        (tzmax, tzmin)
    } else {
        (tzmin, tzmax)
    };

    if (tmin > tzmax) || (tzmin > tmax) {
        return None;
    }

    let tmin = if tzmin > tmin { tzmin } else { tmin };

    let tmax = if tzmax < tmax { tzmax } else { tmax };

    let t = tmin.min(tmax);

    Some(BoxIntersection {
        t,
        ray: ray.clone(),
        boundingbox: boundingbox.clone(),
    })
}
