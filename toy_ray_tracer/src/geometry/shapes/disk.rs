use crate::{utils::random, vec::vec3};
use std::f32::consts::PI;

use visitor::EnterContext;

use crate::{
    aabb::AABB,
    geometry::visitor,
    hittable::{HitRecord, Hittable},
    material::MaterialPtr,
    ray::Ray,
    vec::Vec3,
};

use super::Plane;

pub struct Disk {
    center: Vec3,
    radius: f32,
    normal: Vec3,
    material: MaterialPtr,
    plane: Plane,
}

impl Disk {
    pub fn new(center: Vec3, radius: f32, normal: Vec3, material: MaterialPtr) -> Self {
        let plane = if normal == vec3::XUP {
            Plane::YZ
        } else if normal == vec3::YUP {
            Plane::ZX
        } else if normal == vec3::ZUP {
            Plane::XY
        } else {
            panic!("only support axis-aligned disk, got normal: {:?}", normal)
        };

        Self {
            center,
            radius,
            normal,
            material,
            plane,
        }
    }
}

impl Hittable for Disk {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let o: Vec3 = ray.origin() - self.center;
        let t = -self.normal.dot(&o) / ray.direction().dot(&self.normal);
        let q: Vec3 = o + ray.direction() * t;

        // trace!("q={}, o={}, t={}", q, o, t);
        // in disk
        if q.dot(&q) < self.radius * self.radius {
            if t < t_min || t > t_max {
                return None;
            }

            // TODO: u, v, polar coordinates like sphere ?
            let p = ray.origin() + t * ray.direction();
            let mut rec = HitRecord::new(t, 0.0, 0.0, p, self.material.as_ref());
            rec.set_face_normal(ray, &self.normal);
            return Some(rec);
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t11: f32) -> Option<crate::aabb::AABB> {
        // (P - center) \cdot normal = 0

        // e = radius * (1.0 - normal * normal).sqrt()
        // let e: Vec3 = -vec3::elementwise_mult(&self.normal, &self.normal);
        // let e = e.add_scalar(1.0);
        // let e = self.radius * vec3::sqrt(e);

        // return Some(AABB::new(self.center - e, self.center + e));
        // TODO: more precise bbox
        return Some(AABB::new(
            self.center.add_scalar(-self.radius),
            self.center.add_scalar(self.radius),
        ));
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_disk(self)
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_disk(EnterContext::new(self))
    }

    fn pdf_value(&self, origin: &crate::vec::Point3, v: &Vec3) -> f32 {
        let rec = self.hit(&Ray::new(origin.clone(), v.clone(), 0.0), 0.001, f32::MAX);

        // TODO: Consider not axis-aligned
        if let Some(rec) = rec {
            let area = self.radius * self.radius * PI;
            let distance_squared = rec.t * rec.t * v.norm_squared();
            let cosine = (v.dot(&rec.normal) / v.norm()).abs();

            return distance_squared / (cosine * area);
        }
        0.0
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        // @see https://stackoverflow.com/questions/5837572/generate-a-random-point-within-a-circle-uniformly
        let theta = random::f32_r(0.0, 2.0 * PI);
        let r = self.radius * random::f32().sqrt();

        let c = &self.center;

        let a = r * theta.sin();
        let b = r * theta.cos();

        // currently only support axis-aligned
        let random_point = match self.plane {
            Plane::YZ => Vec3::new(c.x, c.y + a, c.z + b),
            Plane::ZX => Vec3::new(c.x + b, c.y, c.z + a),
            Plane::XY => Vec3::new(c.x + a, c.y + b, c.z),
        };
        return random_point - origin;
    }
}
