use derive_new::new;

use crate::ray::Ray;
use crate::vec::{Point3, Vec3};
use crate::{aabb::AABB, material::Material};

#[derive(new)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub p: Vec3,
    pub material: &'a dyn Material,

    #[new(default)]
    pub normal: Vec3,
    #[new(default)]
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) -> &mut Self {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        return self;
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

pub struct EmptyHittable {}

impl EmptyHittable {
    pub fn new() -> Self {
        return EmptyHittable {};
    }
}

impl Hittable for EmptyHittable {
    fn hit(&self, _ray: &Ray, _t_minn: f32, _t_max: f32) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(Point3::zeros(), Point3::zeros()))
    }
}