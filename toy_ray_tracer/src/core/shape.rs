use std::sync::Arc;

use super::{Point3f, Ray, SurfaceInteraction, Vec3f, AABB};

pub trait Shape: Sync + Send {
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB>;

    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<SurfaceInteraction>;

    fn intersect_p(&self, ray: &Ray) -> bool {
        // naive implementation
        match self.intersect(ray, 0.0, f32::INFINITY) {
            Some(_) => true,
            None => false,
        }
    }

    fn sample_pdf(&self, _point: &Point3f, _wi: &Vec3f) -> f32 {
        1.0
    }

    fn sample_wi(&self, _point: &Point3f) -> Vec3f {
        Vec3f::new(0.0, 0.0, 0.0)
    }
}

pub type ShapePtr = Arc<dyn Shape + Sync + Send>;
