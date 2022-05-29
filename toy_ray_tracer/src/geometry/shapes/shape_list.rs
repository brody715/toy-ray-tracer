use std::sync::Arc;

use crate::core::{Shape, ShapePtr, AABB};
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
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn push(&mut self, shape: ShapePtr) {
        self.shapes.push(shape);
    }

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
    ) -> Option<crate::core::HitRecord> {
        todo!()
    }

    fn pdf_value(&self, origin: &crate::core::Point3f, v: &crate::core::Vec3f) -> f32 {
        let weight = 1.0 / self.shapes.len() as f32;

        let sum = self
            .shapes
            .iter()
            .map(|h| h.pdf_value(origin, v) * weight)
            .fold(0.0 as f32, |acc, v| acc + v);

        return sum;
    }

    fn random(&self, origin: &crate::core::Vec3f) -> crate::core::Vec3f {
        let idx = random::usize(0..self.shapes.len());
        return self.shapes[idx].random(origin);
    }
}
