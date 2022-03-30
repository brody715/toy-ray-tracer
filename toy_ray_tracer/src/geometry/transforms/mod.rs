mod flip_face;
mod rotate;
mod translate;

use std::sync::Arc;

pub use rotate::{Axis, Rotate};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use translate::Translate;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittablePtr},
    ray::Ray,
    vec::Vec3,
};

use super::EnterContext;

pub use flip_face::FlipFace;

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TransformParam {
    Rotate { axis: Axis, angle: f32 },
    Translate { offset: [f32; 3] },
}

pub struct Transforms {
    transformed: HittablePtr,
}

impl Transforms {
    pub fn new(transform_params: &[TransformParam], hittable: HittablePtr) -> Self {
        let transformed: HittablePtr =
            transform_params
                .iter()
                .fold(hittable, |hittable, param| match param {
                    TransformParam::Rotate { axis, angle } => {
                        Arc::new(Rotate::new(*axis, hittable, *angle))
                    }
                    TransformParam::Translate { offset } => {
                        Arc::new(Translate::new(hittable, Vec3::from_column_slice(offset)))
                    }
                });

        Self { transformed }
    }
}
impl Hittable for Transforms {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.transformed.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.transformed.bounding_box(t0, t1)
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_transforms(self)
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_transforms(EnterContext::new(self));
        self.transformed.walk(walker);
    }

    fn pdf_value(&self, origin: &crate::vec::Point3, v: &Vec3) -> f32 {
        self.transformed.pdf_value(origin, v)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        self.transformed.random(origin)
    }
}
