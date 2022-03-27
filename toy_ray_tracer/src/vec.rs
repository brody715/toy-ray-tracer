use std::ops::{Index, IndexMut};

use nalgebra::Vector3;

#[allow(dead_code)]
pub type Vec3 = Vector3<f32>;

#[allow(dead_code)]
pub type Color3 = Vec3;

#[allow(dead_code)]
pub type Point3 = Vec3;

#[derive(Debug)]
pub struct Vec3List(Vec<Vec3>);

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
        self.0.resize(new_len, Vec3::zeros())
    }

    pub(crate) fn add(&self, rhs: &Vec3List) -> Self {
        let v: Vec<Vec3> = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(v1, v2)| v1 + v2)
            .collect();
        Self(v)
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<Vec3> {
        self.0.iter()
    }
}

impl From<Vec<Vec3>> for Vec3List {
    fn from(v: Vec<Vec3>) -> Self {
        return Self(v);
    }
}

impl Index<usize> for Vec3List {
    type Output = Vec3;
    fn index<'a>(&'a self, i: usize) -> &'a Vec3 {
        &self.0[i]
    }
}

impl IndexMut<usize> for Vec3List {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Vec3 {
        &mut self.0[i]
    }
}

// utils for Vec3
pub mod vec3 {
    use crate::utils::random;

    use super::Vec3;

    #[allow(dead_code)]
    pub const YUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    #[allow(dead_code)]
    pub fn random() -> Vec3 {
        todo!()
    }

    #[allow(dead_code)]
    pub fn random_in_unit_sphere() -> Vec3 {
        let unit = Vec3::new(1.0, 1.0, 1.0);
        loop {
            let p = 2.0 * Vec3::new(random::f32(), random::f32(), random::f32()) - unit;
            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
    }

    #[allow(dead_code)]
    pub fn random_in_unit_disk() -> Vec3 {
        let unit = Vec3::new(1.0, 1.0, 0.0);
        loop {
            let p = 2.0 * Vec3::new(random::f32(), random::f32(), 0.0) - unit;
            if p.dot(&p) < 1.0 {
                return p;
            }
        }
    }

    #[inline]
    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - 2.0 * v.dot(&n) * n
    }

    #[inline]
    pub fn refract(uv: &Vec3, n: &Vec3, ni_over_nt: f32) -> Vec3 {
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
    pub fn elementwise_mult(v1: &Vec3, v2: &Vec3) -> Vec3 {
        v1.zip_map(v2, |e1, e2| e1 * e2)
    }

    #[inline]
    pub fn sqrt(v: Vec3) -> Vec3 {
        v.map(|e| e.sqrt())
    }

    #[inline]
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        return v1.dot(v2);
    }

    #[allow(dead_code)]
    #[inline]
    pub fn min(v1: &Vec3, v2: &Vec3) -> Vec3 {
        return Vec3::new(v1.x.min(v2.x), v1.y.min(v2.y), v1.z.min(v2.z));
    }

    #[allow(dead_code)]
    #[inline]
    pub fn max(v1: &Vec3, v2: &Vec3) -> Vec3 {
        return Vec3::new(v1.x.max(v2.x), v1.y.max(v2.y), v1.z.max(v2.z));
    }
}
