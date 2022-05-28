use std::ops::Mul;

use nalgebra::{Matrix4, Unit};

use crate::{
    core::AABB,
    geometry::EnterContext,
    core::{HitRecord, Hittable, HittablePtr},
    core::Ray,
    core::{vec3, Vec3, Vec4f},
};

pub struct Transformed {
    pub hittable: HittablePtr,
    pub transform: Transform,
}

impl Transformed {
    pub fn new(hittable: HittablePtr, transform: Transform) -> Self {
        Self {
            hittable,
            transform,
        }
    }
}

impl Hittable for Transformed {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = self.transform.transform_ray(ray);
        let rec = self.hittable.hit(&moved_ray, t_min, t_max).map(|mut hit| {
            // transform from object space to world space
            hit.p = self.transform.transform_point3(hit.p);

            hit
        });
        rec
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hittable
            .bounding_box(t0, t1)
            .map(|b| self.transform.transform_bounding_box(b))
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_transformed(self);
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_transformed(EnterContext::new(self));
        self.hittable.walk(walker);
    }

    fn pdf_value(&self, origin: &crate::core::Point3, v: &crate::core::Vec3) -> f32 {
        self.hittable.pdf_value(origin, v)
    }

    fn random(&self, origin: &crate::core::Vec3) -> crate::core::Vec3 {
        self.hittable.random(origin)
    }
}

pub struct Transform {
    m: Matrix4<f32>,
    inv_m: Matrix4<f32>,
}

impl Transform {
    pub fn new(m: Matrix4<f32>) -> Self {
        Self {
            m,
            inv_m: m.try_inverse().unwrap(),
        }
    }

    pub fn identity() -> Self {
        Self::new(Matrix4::identity())
    }

    pub fn translate(offset: Vec3) -> Transform {
        let m = Matrix4::new_translation(&offset);
        Self::new(m)
    }

    pub fn scale(scale: Vec3) -> Transform {
        let m = Matrix4::new_nonuniform_scaling(&scale);
        Self::new(m)
    }

    // angle in degree
    pub fn rotate(axis: Vec3, angle: f32) -> Transform {
        let m = Matrix4::from_axis_angle(&Unit::new_normalize(axis), angle.to_radians());
        Self::new(m)
    }
}

impl Transform {
    pub fn inverse(&self) -> Self {
        Self {
            m: self.inv_m,
            inv_m: self.m,
        }
    }

    pub fn transform_ray(&self, ray: &Ray) -> Ray {
        // move from world space to object space
        let inverse_ = self.inverse();
        let origin = inverse_.transform_point3(ray.origin());
        let dir = inverse_.transform_vector3(ray.direction());
        return Ray::new(origin, dir, ray.time());
    }

    pub fn transform_point3(&self, point: Vec3) -> Vec3 {
        let point = Vec4f::new(point[0], point[1], point[2], 1.0);
        let point = self.m * point;
        return point.xyz();
    }

    pub fn transform_vector3(&self, vec: Vec3) -> Vec3 {
        let vec = Vec4f::new(vec[0], vec[1], vec[2], 0.0);
        let vec = self.m * vec;
        return vec.xyz();
    }

    pub fn transform_bounding_box(&self, bbox: AABB) -> AABB {
        let b_min = bbox.min;
        let b_max = bbox.max;
        // cube has 8 points
        let points: Vec<Vec4f> = vec![
            Vec4f::new(b_min[0], b_min[1], b_min[2], 1.0),
            Vec4f::new(b_max[0], b_min[1], b_min[2], 1.0),
            Vec4f::new(b_min[0], b_max[1], b_min[2], 1.0),
            Vec4f::new(b_max[0], b_max[1], b_min[2], 1.0),
            Vec4f::new(b_min[0], b_min[1], b_max[2], 1.0),
            Vec4f::new(b_max[0], b_min[1], b_max[2], 1.0),
            Vec4f::new(b_min[0], b_max[1], b_max[2], 1.0),
            Vec4f::new(b_max[0], b_max[1], b_max[2], 1.0),
        ];

        // transform points
        let points = points
            .iter()
            .map(|p| (self.m * p).xyz())
            .collect::<Vec<_>>();

        // get min and max
        let min = points
            .iter()
            .fold(Vec3::new(f32::MAX, f32::MAX, f32::MAX), |acc, p| {
                vec3::min(&acc, &p)
            });

        let max = points
            .iter()
            .fold(Vec3::new(-f32::MAX, -f32::MAX, -f32::MAX), |acc, p| {
                vec3::max(&acc, &p)
            });

        AABB { min, max }
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Self::Output {
        return Transform {
            m: self.m * rhs.m,
            inv_m: rhs.inv_m * self.inv_m,
        };
    }
}
