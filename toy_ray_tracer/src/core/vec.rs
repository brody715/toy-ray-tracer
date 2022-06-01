use std::ops::{Index, IndexMut};

use nalgebra::{Vector2, Vector3, Vector4};

#[allow(dead_code)]
pub type Vec3f = Vector3<f32>;

pub type Vec4f = Vector4<f32>;

#[allow(dead_code)]
pub type Vec2f = Vector2<f32>;

#[allow(dead_code)]
pub type Color3 = Vec3f;

#[allow(dead_code)]
pub type Point3f = Vec3f;
pub type Point2f = Vec2f;

#[derive(Debug)]
pub struct Vec3List(Vec<Vec3f>);

impl Vec3List {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn new_with_size(new_len: usize) -> Self {
        let mut v = Self::new();
        v.resize(new_len);
        v
    }

    pub(crate) fn resize(&mut self, new_len: usize) {
        self.0.resize(new_len, Vec3f::zeros())
    }

    pub(crate) fn add(&self, rhs: &Vec3List) -> Self {
        let v: Vec<Vec3f> = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(v1, v2)| v1 + v2)
            .collect();
        Self(v)
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<Vec3f> {
        self.0.iter()
    }
}

impl From<Vec<Vec3f>> for Vec3List {
    fn from(v: Vec<Vec3f>) -> Self {
        return Self(v);
    }
}

impl Index<usize> for Vec3List {
    type Output = Vec3f;
    fn index<'a>(&'a self, i: usize) -> &'a Vec3f {
        &self.0[i]
    }
}

impl IndexMut<usize> for Vec3List {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Vec3f {
        &mut self.0[i]
    }
}

// utils for Vec3
pub mod vec3 {
    use std::f32::{consts::PI, EPSILON};

    use crate::utils::random;

    use super::{Vec3f, Vec4f};

    #[allow(dead_code)]
    pub const YUP: Vec3f = Vec3f::new(0.0, 1.0, 0.0);

    pub const ZUP: Vec3f = Vec3f::new(0.0, 0.0, 1.0);

    pub const XUP: Vec3f = Vec3f::new(1.0, 0.0, 0.0);

    pub const INF: Vec3f = Vec3f::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);

    #[inline]
    pub fn lerp(a: &Vec3f, b: &Vec3f, t: f32) -> Vec3f {
        a + (b - a) * t
    }

    #[inline]
    pub fn scalar(value: f32) -> Vec3f {
        Vec3f::new(value, value, value)
    }

    #[inline]
    pub fn is_near_zero(v: &Vec3f) -> bool {
        let eps = EPSILON;
        return v[0] < eps && v[1] < eps && v[2] < eps;
    }

    #[inline]
    pub fn is_black(v: &Vec3f) -> bool {
        self::is_near_zero(v)
    }

    pub fn is_same_hemisphere(wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> bool {
        return wi.dot(normal) * wo.dot(normal) > 0.0;
    }

    #[allow(dead_code)]
    pub fn random() -> Vec3f {
        return Vec3f::new(random::f32(), random::f32(), random::f32());
    }

    #[allow(dead_code)]
    pub fn random_in_unit_sphere() -> Vec3f {
        let unit = Vec3f::new(1.0, 1.0, 1.0);
        loop {
            let p = 2.0 * Vec3f::new(random::f32(), random::f32(), random::f32()) - unit;
            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
    }

    #[allow(dead_code)]
    pub fn random_in_unit_disk() -> Vec3f {
        let unit = Vec3f::new(1.0, 1.0, 0.0);
        loop {
            let p = 2.0 * Vec3f::new(random::f32(), random::f32(), 0.0) - unit;
            if p.dot(&p) < 1.0 {
                return p;
            }
        }
    }

    #[allow(dead_code)]
    pub fn random_in_hemisphere(normal: &Vec3f) -> Vec3f {
        let in_unit_sphere = self::random_in_unit_sphere();
        if in_unit_sphere.dot(&normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    #[allow(dead_code)]
    pub fn random_unit_vector() -> Vec3f {
        return self::random_in_unit_sphere().normalize();
    }

    pub fn random_cosine_direction() -> Vec3f {
        let r1 = random::f32();
        let r2 = random::f32();
        let z = (1.0 - r2).sqrt();

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();

        return Vec3f::new(x, y, z);
    }

    pub fn random_hemisphere_cosine(normal: &Vec3f) -> Vec3f {
        let dir = self::random_cosine_direction();
        if dir.dot(normal) > 0.0 {
            dir
        } else {
            -dir
        }
    }

    pub fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3f {
        let r1 = random::f32();
        let r2 = random::f32();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        return Vec3f::new(x, y, z);
    }

    #[inline]
    pub fn reflect(v: &Vec3f, n: &Vec3f) -> Vec3f {
        -v + 2.0 * v.dot(&n) * n
    }

    #[inline]
    pub fn refract(uv: &Vec3f, n: &Vec3f, ni_over_nt: f32) -> Vec3f {
        let cos_theta = self::dot(&-uv, n).min(1.0);
        let r_out_perp = ni_over_nt * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * n;
        return r_out_perp + r_out_parallel;
    }

    #[inline]
    pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    #[inline]
    pub fn elementwise_mult(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
        v1.zip_map(v2, |e1, e2| e1 * e2)
    }

    #[inline]
    pub fn elementwise_div(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
        v1.zip_map(v2, |e1, e2| e1 / e2)
    }

    #[inline]
    pub fn sqrt(v: Vec3f) -> Vec3f {
        v.map(|e| e.sqrt())
    }

    #[inline]
    pub fn dot(v1: &Vec3f, v2: &Vec3f) -> f32 {
        return v1.dot(v2);
    }

    #[allow(dead_code)]
    #[inline]
    pub fn min(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
        return Vec3f::new(v1.x.min(v2.x), v1.y.min(v2.y), v1.z.min(v2.z));
    }

    #[allow(dead_code)]
    #[inline]
    pub fn max(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
        return Vec3f::new(v1.x.max(v2.x), v1.y.max(v2.y), v1.z.max(v2.z));
    }

    pub fn point3_to_homo(p: &Vec3f) -> Vec4f {
        return Vec4f::new(p[0], p[1], p[2], 1.0);
    }

    pub fn vec3_to_homo(v: &Vec3f) -> Vec4f {
        return Vec4f::new(v[0], v[1], v[2], 0.0);
    }
}
