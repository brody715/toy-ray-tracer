use crate::aabb::AABB;
use crate::aabb::{self, create_sphere_box};
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Material, MaterialPtr};
use crate::ray::Ray;
use crate::vec::Vec3;
use nalgebra::Vector3;
use std::f32;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: MaterialPtr,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: MaterialPtr) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

fn get_sphere_uv(p: &Vec3) -> (f32, f32) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI);
    let v = (theta + f32::consts::FRAC_PI_2) / f32::consts::PI;
    (u, v)
}

#[inline]
fn sphere_hit<'a>(
    ray: &Ray,
    center: Vec3,
    radius: f32,
    material: &'a dyn Material,
    t_min: f32,
    t_max: f32,
) -> Option<HitRecord<'a>> {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_discriminant = discriminant.sqrt();

    let root1 = (-b - sqrt_discriminant) / (2.0 * a);
    let root = if root1 > t_min && root1 < t_max {
        Some(root1)
    } else {
        let root2 = (-b + sqrt_discriminant) / (2.0 * a);
        if root2 > t_min && root2 < t_max {
            Some(root2)
        } else {
            None
        }
    };

    if let Some(t) = root {
        let p = ray.point_at_parameter(t);
        let normal = (p - center) / radius;
        let (u, v) = get_sphere_uv(&normal);
        let mut rec = HitRecord::new(t, u, v, p, material);
        rec.set_face_normal(ray, &normal);
        return Some(rec);
    }

    return None;
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        return sphere_hit(
            ray,
            self.center,
            self.radius,
            self.material.as_ref(),
            t_min,
            t_max,
        );
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let radius = Vector3::new(self.radius, self.radius, self.radius);
        let min = self.center - radius;
        let max = self.center + radius;
        Some(AABB { min, max })
    }
}

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: MaterialPtr,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: MaterialPtr,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        sphere_hit(
            ray,
            self.center(ray.time()),
            self.radius,
            self.material.as_ref(),
            t_min,
            t_max,
        )
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let box0 = create_sphere_box(&self.center0, self.radius);
        let box1 = create_sphere_box(&self.center1, self.radius);
        Some(aabb::create_surrounding_box(&box0, &box1))
    }
}
