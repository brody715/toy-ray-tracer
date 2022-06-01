use std::sync::Arc;

use crate::core::Ray;
use crate::core::AABB;
use crate::core::{Point3f, Vec3f};

use super::SurfaceInteraction;

pub trait Primitive: Sync + Send {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<SurfaceInteraction>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;

    fn sample_pdf(&self, _point: &Point3f, _wi: &Vec3f) -> f32;

    fn sample_wi(&self, _point: &Vec3f) -> Vec3f;
}

pub type PrimitivePtr = Arc<dyn Primitive + Sync + Send>;
pub type PrimitiveRef<'a> = &'a (dyn Primitive);

pub trait PrimitiveContainer: Sync + Send {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<SurfaceInteraction>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

pub type PrimitiveContainerPtr = Arc<dyn PrimitiveContainer + Sync + Send>;
