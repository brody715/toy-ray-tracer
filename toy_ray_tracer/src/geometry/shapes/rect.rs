use std::sync::Arc;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittablePtr};
use crate::material::MaterialPtr;
use crate::ray::Ray;
use crate::utils::random;
use crate::vec::Vec3;

pub struct Rect {
    // left-bottom vertex
    // p0: Vec3,
    // right-top vertex
    // p1: Vec3,
    // material: MaterialPtr,
    _impl: HittablePtr,
}

impl Rect {
    #[must_use]
    pub fn new(p0: Vec3, p1: Vec3, material: MaterialPtr) -> Self {
        let axiso = p0.iter().zip(p1.iter()).position(|(l, r)| l == r);

        let k_axis = axiso.unwrap_or(3);

        let (a, b, plane) = match k_axis {
            0 => (1, 2, Plane::YZ),
            1 => (2, 0, Plane::ZX),
            2 => (0, 1, Plane::XY),
            _ => panic!("unsupported rect: {}, {}", p0, p1),
        };

        let a0 = p0[a].min(p1[a]);
        let a1 = p0[a].max(p1[a]);
        let b0 = p0[b].min(p1[b]);
        let b1 = p0[b].max(p1[b]);
        let k = p0[k_axis];

        // info!(
        //     "p0={}, p1={}, k_axis={} a0={}, a1={}, b0={}, b1={}, k={}",
        //     &p0, &p1, k_axis, a0, a1, b0, b1, k
        // );

        Self {
            // p0,
            // p1,
            // material: material.clone(),
            _impl: Arc::new(AARect::new(plane, a0, a1, b0, b1, k, material.clone())),
        }
    }
}

impl Hittable for Rect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self._impl.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self._impl.bounding_box(t0, t1)
    }

    fn pdf_value(&self, origin: &crate::vec::Point3, v: &Vec3) -> f32 {
        self._impl.pdf_value(origin, v)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        self._impl.random(origin)
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub enum Plane {
    YZ,
    ZX,
    XY,
}

pub struct AARect {
    plane: Plane,
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    material: MaterialPtr,
}

impl AARect {
    pub fn new(
        plane: Plane,
        a0: f32,
        a1: f32,
        b0: f32,
        b1: f32,
        k: f32,
        material: MaterialPtr,
    ) -> Self {
        AARect {
            plane,
            a0,
            a1,
            b0,
            b1,
            k,
            material,
        }
    }
}

impl Hittable for AARect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (k_axis, a_axis, b_axis) = match &self.plane {
            Plane::YZ => (0, 1, 2),
            Plane::ZX => (1, 2, 0),
            Plane::XY => (2, 0, 1),
        };
        let t = (self.k - ray.origin()[k_axis]) / ray.direction()[k_axis];
        if t < t_min || t > t_max {
            None
        } else {
            let a = ray.origin()[a_axis] + t * ray.direction()[a_axis];
            let b = ray.origin()[b_axis] + t * ray.direction()[b_axis];
            if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                None
            } else {
                let u = (a - self.a0) / (self.a1 - self.a0);
                let v = (b - self.b0) / (self.b1 - self.b0);
                let p = ray.point_at_parameter(t);
                let mut normal = Vec3::zeros();
                normal[k_axis] = 1.0;
                let mut rec = HitRecord::new(t, u, v, p, self.material.as_ref());
                rec.set_face_normal(ray, &normal);
                return Some(rec);
            }
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let (min, max) = match &self.plane {
            Plane::YZ => (
                Vec3::new(self.k - 0.0001, self.a0, self.b0),
                Vec3::new(self.k + 0.0001, self.a1, self.b1),
            ),
            Plane::ZX => (
                Vec3::new(self.b0, self.k - 0.0001, self.a0),
                Vec3::new(self.b1, self.k + 0.0001, self.a1),
            ),
            Plane::XY => (
                Vec3::new(self.a0, self.b0, self.k - 0.0001),
                Vec3::new(self.a1, self.b1, self.k + 0.0001),
            ),
        };
        Some(AABB { min, max })
    }

    fn pdf_value(&self, origin: &crate::vec::Point3, v: &Vec3) -> f32 {
        let rec = self.hit(&Ray::new(origin.clone(), v.clone(), 0.0), 0.001, f32::MAX);

        if let Some(rec) = rec {
            let area = (self.a1 - self.a0) * (self.b1 - self.b0);
            let distance_squared = rec.t * rec.t * v.norm_squared();
            let cosine = (v.dot(&rec.normal) / v.norm()).abs();

            return distance_squared / (cosine * area);
        }

        return 0.0;
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let rand_a = random::f32_r(self.a0, self.a1);
        let rand_b = random::f32_r(self.b0, self.b1);

        let random_point = match self.plane {
            Plane::YZ => Vec3::new(self.k, rand_a, rand_b),
            Plane::ZX => Vec3::new(rand_b, self.k, rand_a),
            Plane::XY => Vec3::new(rand_a, rand_b, self.k),
        };
        return random_point - origin;
    }
}
