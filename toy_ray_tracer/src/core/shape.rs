use std::sync::Arc;

use super::{HitRecord, Point3f, Ray, Vec3f, AABB};

pub trait Shape: Sync + Send {
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB>;

    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn pdf_value(&self, _origin: &Point3f, _v: &Vec3f) -> f32 {
        1.0
    }

    fn random(&self, _origin: &Point3f) -> Vec3f {
        Vec3f::new(0.0, 0.0, 0.0)
    }
}

pub type ShapePtr = Arc<dyn Shape + Sync + Send>;
