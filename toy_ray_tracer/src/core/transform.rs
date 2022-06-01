use std::ops::Mul;

use nalgebra::{Matrix4, Unit};

use super::{vec3, Ray, SurfaceInteraction, Vec3f, Vec4f, AABB};

#[derive(Clone)]
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

    pub fn translate(offset: Vec3f) -> Transform {
        let m = Matrix4::new_translation(&offset);
        Self::new(m)
    }

    pub fn scale(scale: Vec3f) -> Transform {
        let m = Matrix4::new_nonuniform_scaling(&scale);
        Self::new(m)
    }

    // angle in degree
    pub fn rotate(axis: Vec3f, angle: f32) -> Transform {
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
        let origin = self.transform_point3(&ray.origin());
        let dir = self.transform_vector3(&ray.direction());
        return Ray::new(origin, dir, ray.time());
    }

    pub fn transform_point3(&self, point: &Vec3f) -> Vec3f {
        let point = Vec4f::new(point[0], point[1], point[2], 1.0);
        let point = self.m * point;
        return point.xyz();
    }

    pub fn transform_vector3(&self, vec: &Vec3f) -> Vec3f {
        let vec = Vec4f::new(vec[0], vec[1], vec[2], 0.0);
        let vec = self.m * vec;
        return vec.xyz();
    }

    pub fn transform_unit_dir(&self, vec: &Vec3f) -> Vec3f {
        let vec = Vec4f::new(vec[0], vec[1], vec[2], 0.0);
        let vec = self.m * vec;
        return vec.xyz().normalize();
    }

    pub fn transform_normal(&self, normal: &Vec3f) -> Vec3f {
        return self.transform_vector3(normal).normalize();
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
            .fold(Vec3f::new(f32::MAX, f32::MAX, f32::MAX), |acc, p| {
                vec3::min(&acc, &p)
            });

        let max = points
            .iter()
            .fold(Vec3f::new(-f32::MAX, -f32::MAX, -f32::MAX), |acc, p| {
                vec3::max(&acc, &p)
            });

        AABB { min, max }
    }

    pub fn transform_surface_iteraction(&self, si: &mut SurfaceInteraction) {
        si.point = self.transform_point3(&si.point);
        si.normal = self.transform_normal(&si.normal);
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
