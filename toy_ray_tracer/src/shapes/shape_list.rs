use std::sync::Arc;

use crate::core::{Shape, ShapePtr, SurfaceInteraction, AABB};
use crate::utils::random;

pub struct ShapeList {
    shapes: Vec<ShapePtr>,
}

impl From<Vec<ShapePtr>> for ShapeList {
    fn from(shapes: Vec<ShapePtr>) -> Self {
        return Self { shapes };
    }
}

impl ShapeList {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, shape: ShapePtr) {
        self.shapes.push(shape);
    }

    #[allow(dead_code)]
    pub fn emplace_back(&mut self, shape: impl Shape + 'static) {
        self.shapes.push(Arc::new(shape));
    }
}

impl Shape for ShapeList {
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let bbox = self.shapes.iter().fold(None, |acc, item| {
            AABB::union_optional_bbox(&acc, &item.bounding_box(t0, t1))
        });
        bbox
    }

    fn intersect(
        &self,
        ray: &crate::core::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<SurfaceInteraction> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<SurfaceInteraction> = None;
        for h in self.shapes.iter() {
            if let Some(hit) = h.intersect(ray, t_min, closest_so_far) {
                closest_so_far = hit.t_hit;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }

    fn sample_pdf(&self, origin: &crate::core::Point3f, v: &crate::core::Vec3f) -> f32 {
        let weight = 1.0 / self.shapes.len() as f32;

        let sum = self
            .shapes
            .iter()
            .map(|h| h.sample_pdf(origin, v) * weight)
            .fold(0.0 as f32, |acc, v| acc + v);

        return sum;
    }

    fn sample_wi(&self, origin: &crate::core::Vec3f) -> crate::core::Vec3f {
        let idx = random::usize(0..self.shapes.len());
        return self.shapes[idx].sample_wi(origin);
    }
}
