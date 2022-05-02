use std::sync::Arc;

use crate::aabb::AABB;
use crate::geometry::{EnterContext, GeometryVisitor, GeometryWalker};
use crate::material::Material;
use crate::math::SamplerType;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

pub struct HitRecord<'a> {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub p: Vec3,
    pub material: &'a dyn Material,

    pub normal: Vec3,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, u: f32, v: f32, p: Vec3, material: &'a dyn Material) -> Self {
        Self {
            t,
            u,
            v,
            p,
            material,
            normal: Vec3::zeros(),
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) -> &mut Self {
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

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;

    // TODO: find a better way to change sample type
    fn set_sampler(&mut self, _sampler_type: SamplerType) {
        unimplemented!()
    }

    fn pdf_value(&self, _origin: &Point3, _v: &Vec3) -> f32 {
        1.0
    }

    fn random(&self, _origin: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }

    fn accept(&self, visitor: &mut dyn GeometryVisitor);

    fn walk(&self, walker: &mut dyn GeometryWalker);
}

pub type HittablePtr = Arc<dyn Hittable + Sync + Send>;
pub type HittableRef<'a> = &'a (dyn Hittable);

pub struct NopHittable {}

impl NopHittable {
    #[allow(dead_code)]
    pub fn new() -> Self {
        return NopHittable {};
    }
}

impl Hittable for NopHittable {
    fn hit(&self, _ray: &Ray, _t_minn: f32, _t_max: f32) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(Point3::zeros(), Point3::zeros()))
    }

    fn accept(&self, visitor: &mut dyn GeometryVisitor) {
        visitor.visit_nop_hittable(self)
    }

    fn walk(&self, walker: &mut dyn GeometryWalker) {
        walker.enter_nop_hittable(EnterContext::new(self))
    }
}
