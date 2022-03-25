use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Isotropic;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::utils::random;
use std::f32;

use crate::vec::Vec3;

pub struct ConstantMedium<H: Hittable, T: Texture> {
    boundary: H,
    density: f32,
    phase_function: Isotropic<T>,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, T> {
    pub fn new(boundary: H, density: f32, texture: T) -> Self {
        ConstantMedium {
            boundary,
            density,
            phase_function: Isotropic::new(texture),
        }
    }
}

impl<H: Hittable, T: Texture> Hittable for ConstantMedium<H, T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut hit1) = self.boundary.hit(&ray, -f32::MAX, f32::MAX) {
            if let Some(mut hit2) = self.boundary.hit(&ray, hit1.t + 0.0001, f32::MAX) {
                if hit1.t < t_min {
                    hit1.t = t_min
                }
                if hit2.t > t_max {
                    hit2.t = t_max
                }
                if hit1.t < hit2.t {
                    let distance_inside_boundary = (hit2.t - hit1.t) * ray.direction().norm();
                    let hit_distance = -(1.0 / self.density) * random::f32().ln();
                    if hit_distance < distance_inside_boundary {
                        let t = hit1.t + hit_distance / ray.direction().norm();
                        let mut rec = HitRecord::new(
                            t,
                            0.0,
                            0.0,
                            ray.point_at_parameter(t),
                            &self.phase_function,
                        );
                        rec.set_face_normal(ray, &Vec3::new(1.0, 0.0, 0.0));
                        return Some(rec);
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}