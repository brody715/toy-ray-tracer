use crate::aabb;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::utils::random;
use std::cmp::Ordering;

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB,
}

fn box_compare(
    time0: f32,
    time1: f32,
    axis: usize,
) -> impl Fn(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
    move |a, b| {
        let a_bbox = a.bounding_box(time0, time1);
        let b_bbox = b.bounding_box(time0, time1);
        if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
            return a.min[axis].partial_cmp(&b.min[axis]).unwrap();
        } else {
            panic!["no bounding box in bvh node"]
        }
    }
}

impl BVH {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>, time0: f32, time1: f32) -> Self {
        let axis = random::usize(0..3);

        objects.sort_unstable_by(box_compare(time0, time1, axis));

        let len = objects.len();
        match len {
            0 => panic!["no objects"],
            1 => {
                let leaf = objects.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(time0, time1) {
                    BVH {
                        tree: BVHNode::Leaf(leaf),
                        bbox,
                    }
                } else {
                    panic!["no bounding box in BVH"]
                }
            }
            _ => {
                let right_objs = objects.drain((len / 2)..).collect();
                let left_objs = objects;

                let left = BVH::new(left_objs, time0, time1);
                let right = BVH::new(right_objs, time0, time1);

                let bbox = aabb::create_surrounding_box(&left.bbox, &right.bbox);
                BVH {
                    tree: BVHNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                }
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        if !self.bbox.hit(&ray, t_min, t_max) {
            return None;
        }
        match &self.tree {
            BVHNode::Leaf(leaf) => leaf.hit(&ray, t_min, t_max),
            BVHNode::Branch { left, right } => {
                let left = left.hit(&ray, t_min, t_max);
                if let Some(l) = &left {
                    t_max = l.t
                };
                let right = right.hit(&ray, t_min, t_max);
                if right.is_some() {
                    right
                } else {
                    left
                }
            }
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}