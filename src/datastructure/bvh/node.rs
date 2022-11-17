use crate::datastructure::bvh::boundingbox::BoundingBox;
use crate::scene::triangle::Triangle;
use crate::util::vector::Vector;
use core::fmt;
use log::debug;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub enum BVHNode {
    Leaf {
        bounding_box: BoundingBox,
        triangles: Vec<Arc<Triangle>>,
    },
    Node {
        bounding_box: BoundingBox,

        left: Box<BVHNode>,
        right: Box<BVHNode>,
    },
}

impl Display for BVHNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.print(f, 0)
    }
}

impl BVHNode {
    fn print(&self, f: &mut Formatter<'_>, depth: usize) -> fmt::Result {
        match self {
            BVHNode::Leaf {
                bounding_box: _,
                triangles,
            } => {
                write!(f, "{}", "\t".repeat(depth))?;
                writeln!(f, "leaf node with {} triangles:", triangles.len(),)?;
            }
            BVHNode::Node {
                left,
                right,
                bounding_box: _,
            } => {
                write!(f, "{}", "\t".repeat(depth))?;
                writeln!(f, ">>")?;
                left.print(f, depth + 1)?;
                right.print(f, depth + 1)?;
            }
        }

        Ok(())
    }

    pub fn new(triangles: Vec<Arc<Triangle>>) -> Self {
        debug!("Creating new KD Tree with {} triangles", triangles.len());

        let bb = BoundingBox::from_triangles(triangles.iter().cloned());

        Self::new_internal(triangles, bb, 0)
    }

    fn divide_triangles_over_boundingboxes(
        (leftbox, rightbox): (&BoundingBox, &BoundingBox),
        triangles: &Vec<Arc<Triangle>>,
    ) -> (Vec<Arc<Triangle>>, Vec<Arc<Triangle>>) {
        let mut leftset = Vec::with_capacity(65536);
        let mut rightset = Vec::with_capacity(65536);

        for i in triangles {
            if leftbox.contains(i) {
                leftset.push(i.clone());
            }
            if rightbox.contains(i) {
                rightset.push(i.clone());
            }
        }

        (leftset, rightset)
    }

    fn new_internal(
        triangles: Vec<Arc<Triangle>>,
        bounding_box: BoundingBox,
        depth: usize,
    ) -> Self {
        if triangles.len() == 0 {
            return BVHNode::Leaf {
                bounding_box: BoundingBox::EMPTY,
                triangles,
            };
        }
        if triangles.len() < 30 {
            return BVHNode::Leaf {
                bounding_box,
                triangles,
            };
        }

        let longest_axis = bounding_box.longest_axis();

        struct State {
            leftbox: BoundingBox,
            rightbox: BoundingBox,
            leftset: Vec<Arc<Triangle>>,
            rightset: Vec<Arc<Triangle>>,

            totalcost: f64,
        }

        let mut smallest: Option<State> = None;

        for (leftbox, rightbox) in longest_axis.divide(&bounding_box, 16) {
            let (leftset, rightset) =
                Self::divide_triangles_over_boundingboxes((&leftbox, &rightbox), &triangles);

            let leftcost = leftbox.cost(leftset.len());
            let rightcost = rightbox.cost(rightset.len());
            let totalcost = leftcost + rightcost;

            if let Some(s) = smallest.as_ref() {
                if totalcost < s.totalcost {
                    smallest = Some(State {
                        leftbox,
                        rightbox,
                        leftset,
                        rightset,
                        totalcost,
                    })
                }
            } else {
                smallest = Some(State {
                    leftbox,
                    rightbox,
                    leftset,
                    rightset,
                    totalcost,
                });
            }
        }

        // Can't fail because smallest is set in the first iteration of the loop.
        let smallest = smallest.unwrap();
        let current_cost = bounding_box.cost(triangles.len());

        debug!("Smallest possible split cost: {}", smallest.totalcost);
        debug!("Parent split cost: {}", current_cost);

        if smallest.totalcost >= current_cost {
            BVHNode::Leaf {
                bounding_box,
                triangles,
            }
        } else {
            BVHNode::Node {
                bounding_box,
                left: Box::new(Self::new_internal(
                    smallest.leftset,
                    smallest.leftbox,
                    depth + 1,
                )),
                right: Box::new(Self::new_internal(
                    smallest.rightset,
                    smallest.rightbox,
                    depth + 1,
                )),
            }
        }
    }

    pub fn includes_point(&self, point: &Vector) -> bool {
        match self {
            BVHNode::Leaf { bounding_box, .. } => bounding_box.includes_point(point),
            BVHNode::Node { bounding_box, .. } => bounding_box.includes_point(point),
        }
    }
}
