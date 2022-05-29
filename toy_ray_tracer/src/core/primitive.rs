use std::sync::Arc;

use crate::core::Ray;
use crate::core::AABB;
use crate::core::{Point3f, Vec3f};
use crate::math::SamplerType;

use super::Spectrum;
use super::reflection::Bsdf;
use super::vec::Point2f;
use super::Color3;

pub struct HitRecord {
    pub t: f32,
    pub uv: Point2f,
    pub point: Point3f,

    pub normal: Vec3f,
    pub front_face: bool,

    pub bsdf: Option<Bsdf>,
    pub emitted: Spectrum,
}

impl HitRecord {
    pub fn new(t: f32, uv: Point2f, p: Point3f) -> Self {
        Self {
            t,
            uv,
            point: p,
            normal: Vec3f::zeros(),
            front_face: false,
            bsdf: None,
            emitted: Color3::zeros(),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3f) -> &mut Self {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        return self;
    }

    pub fn flip_normal(&mut self) -> &mut Self {
        self.front_face = !self.front_face;
        self.normal = -self.normal;
        return self;
    }
}

pub trait Primitive: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
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

pub struct GeometricPrimitive {}
