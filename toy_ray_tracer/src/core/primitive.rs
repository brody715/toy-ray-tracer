use std::sync::Arc;

use crate::core::Ray;
use crate::core::AABB;
use crate::core::{Point3f, Vec3f};
use crate::math::SamplerType;

use super::SurfaceInteraction;

pub trait Primitive: Sync + Send {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<SurfaceInteraction>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;

    // TODO: find a better way to change sample type
    fn set_sampler(&mut self, _sampler_type: SamplerType) {
        unimplemented!()
    }

    fn pdf_value(&self, _origin: &Point3f, _v: &Vec3f) -> f32 {
        1.0
    }

    fn random(&self, _origin: &Vec3f) -> Vec3f {
        Vec3f::new(1.0, 0.0, 0.0)
    }
}

pub type PrimitivePtr = Arc<dyn Primitive + Sync + Send>;
pub type PrimitiveRef<'a> = &'a (dyn Primitive);
