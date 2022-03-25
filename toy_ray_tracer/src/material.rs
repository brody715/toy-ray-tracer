use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::{Color3, Vec3};

pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color3)>;

    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Color3;
}
