use crate::{
    math::{NopSampler, Sampler, SamplerPtr},
    utils::random,
    vec::vec3,
};
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

#[derive(Debug, Clone, Copy)]
struct DiskData {
    center: Vec3,
    radius: f32,
    #[allow(dead_code)]
    normal: Vec3,
    plane: Plane,
}

pub struct Disk {
    center: Vec3,
    radius: f32,
    normal: Vec3,
    material: MaterialPtr,
    plane: Plane,
    sampler: SamplerPtr,
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
            sampler: Box::new(NopSampler::new()),
        }
    }
}

impl<'a> Hittable for Disk {
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

    fn set_sampler(&mut self, sampler_type: crate::math::SamplerType) {
        let disk_data = DiskData {
            center: self.center,
            radius: self.radius,
            normal: self.normal,
            plane: self.plane,
        };
        let sampler: SamplerPtr = match sampler_type {
            crate::math::SamplerType::Uniform { block_size } => {
                Box::new(DiskUniformSampler::new(disk_data, block_size))
            }
            crate::math::SamplerType::Random => Box::new(DiskRandomSampler::new(disk_data)),
            crate::math::SamplerType::BlueNoise { block_size } => {
                Box::new(DiskBlueNoiseSampler::new(disk_data, block_size))
            }
            crate::math::SamplerType::RandomFixed { block_size } => {
                Box::new(DiskRandomFixedSampler::new(disk_data, block_size))
            }
        };
        self.sampler = sampler;
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
        self.sampler.sample_direction(origin)
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_disk(self)
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_disk(EnterContext::new(self))
    }
}

struct DiskRandomSampler {
    disk: DiskData,
}

impl DiskRandomSampler {
    pub fn new(disk: DiskData) -> Self {
        Self { disk }
    }
}

fn point_on_disk(theta: f32, r: f32, center: Vec3, plane: Plane) -> Vec3 {
    let c = center;
    let a = r * theta.sin();
    let b = r * theta.cos();

    // currently only support axis-aligned
    return match plane {
        Plane::YZ => Vec3::new(c.x, c.y + a, c.z + b),
        Plane::ZX => Vec3::new(c.x + b, c.y, c.z + a),
        Plane::XY => Vec3::new(c.x + a, c.y + b, c.z),
    };
}

impl Sampler for DiskRandomSampler {
    fn sample_direction(&self, origin: &Vec3) -> Vec3 {
        let disk = &self.disk;
        // @see https://stackoverflow.com/questions/5837572/generate-a-random-point-within-a-circle-uniformly
        let theta = random::f32_r(0.0, 2.0 * PI);
        let r = disk.radius * random::f32().sqrt();

        let random_point = point_on_disk(theta, r, disk.center, disk.plane);
        return random_point - origin;
    }
}

struct DiskRandomFixedSampler {
    sampled_points: Vec<Vec3>,
}

impl DiskRandomFixedSampler {
    pub fn new(disk: DiskData, block_size: [i32; 2]) -> Self {
        let n_points = block_size[0] * block_size[1];
        let sampled_points = (0..n_points)
            .map(|_| {
                let theta = random::f32_r(0.0, 2.0 * PI);
                let r = disk.radius * random::f32().sqrt();

                point_on_disk(theta, r, disk.center, disk.plane)
            })
            .collect();

        Self { sampled_points }
    }
}

impl Sampler for DiskRandomFixedSampler {
    fn sample_direction(&self, origin: &Vec3) -> Vec3 {
        let index = random::usize(0..self.sampled_points.len());
        return self.sampled_points[index] - origin;
    }
}

struct DiskUniformSampler {
    sampled_points: Vec<Vec3>,
}

impl DiskUniformSampler {
    pub fn new(disk: DiskData, block_size: [i32; 2]) -> Self {
        let mut sampled_points: Vec<Vec3> = Vec::new();

        for i in 0..block_size[0] {
            for j in 0..block_size[1] {
                let x = i as f32 / block_size[0] as f32;
                let y = j as f32 / block_size[1] as f32;
                let theta = 2.0 * PI * x;
                let r = disk.radius * y;

                let random_point = point_on_disk(theta, r, disk.center, disk.plane);
                sampled_points.push(random_point);
            }
        }

        Self { sampled_points }
    }
}

impl Sampler for DiskUniformSampler {
    fn sample_direction(&self, origin: &Vec3) -> Vec3 {
        let idx = random::usize(0..self.sampled_points.len());
        let random_point = self.sampled_points[idx];
        return random_point - origin;
    }
}

struct DiskBlueNoiseSampler {
    sampled_points: Vec<Vec3>,
}

impl DiskBlueNoiseSampler {
    pub fn new(disk: DiskData, block_size: [i32; 2]) -> Self {
        let mut sampled_points: Vec<Vec3> = Vec::new();

        // BlueNoise alg
        let n_points = block_size[0] * block_size[1];
        for idx in 0..n_points {
            let (x, y) = Self::halton_sequense_2d(idx);
            let theta = 2.0 * PI * x;
            let r = disk.radius * y;

            let random_point = point_on_disk(theta, r, disk.center, disk.plane);
            sampled_points.push(random_point);
        }

        Self { sampled_points }
    }

    fn halton_sequense_2d(idx: i32) -> (f32, f32) {
        return (Self::radical_inverse(idx, 2), Self::radical_inverse(idx, 3));
    }

    fn radical_inverse(n: i32, base: i32) -> f32 {
        let mut n = n;
        let mut val = 0.0;
        let inv_base = 1.0 / base as f32;
        let mut inv_bi = inv_base;
        while n > 0 {
            let r = n % base;
            let q = n / base;
            val += r as f32 * inv_bi;
            inv_bi *= inv_base;
            n = q;
        }
        return val;
    }
}

impl Sampler for DiskBlueNoiseSampler {
    fn sample_direction(&self, origin: &Vec3) -> Vec3 {
        let idx = random::usize(0..self.sampled_points.len());
        let random_point = self.sampled_points[idx];
        return random_point - origin;
    }
}
