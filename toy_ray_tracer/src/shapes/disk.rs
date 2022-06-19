use crate::{
    core::{vec3, Point2f, Shape, Transform, Vec3f},
    math::{Sampler, SamplerPtr},
    utils::random,
};
use std::f32::consts::PI;

use crate::{core::Ray, core::SurfaceInteraction, core::AABB};

use super::Plane;

#[derive(Debug, Clone, Copy)]
struct DiskData {
    center: Vec3f,
    radius: f32,
    #[allow(dead_code)]
    normal: Vec3f,
    plane: Plane,
}

pub struct Disk {
    center: Vec3f,
    radius: f32,
    normal: Vec3f,
    sampler: SamplerPtr,
    object_to_world: Transform,
    world_to_object: Transform,
}

impl Disk {
    pub fn new(center: Vec3f, radius: f32, normal: Vec3f, object_to_world: Transform) -> Self {
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
            sampler: Box::new(DiskRandomSampler::new(DiskData {
                center,
                radius,
                normal,
                plane,
            })),
            object_to_world: object_to_world.clone(),
            world_to_object: object_to_world.inverse(),
        }
    }
}

impl Shape for Disk {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<SurfaceInteraction> {
        let ray = self.world_to_object.transform_ray(ray);

        let o: Vec3f = ray.origin() - self.center;
        let t = -self.normal.dot(&o) / ray.direction().dot(&self.normal);
        let q: Vec3f = o + ray.direction() * t;

        // trace!("q={}, o={}, t={}", q, o, t);
        // in disk
        if q.dot(&q) < self.radius * self.radius {
            if t < t_min || t > t_max {
                return None;
            }

            // TODO: u, v, polar coordinates like sphere ?
            let p = ray.origin() + t * ray.direction();

            let mut si = SurfaceInteraction::new(
                t,
                p,
                Point2f::new(0.0, 0.0),
                -ray.direction(),
                self.normal,
            );

            self.object_to_world.transform_surface_iteraction(&mut si);

            return Some(si);
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t11: f32) -> Option<crate::core::AABB> {
        // (P - center) \cdot normal = 0

        // e = radius * (1.0 - normal * normal).sqrt()
        // let e: Vec3 = -vec3::elementwise_mult(&self.normal, &self.normal);
        // let e = e.add_scalar(1.0);
        // let e = self.radius * vec3::sqrt(e);

        // return Some(AABB::new(self.center - e, self.center + e));
        // TODO: more precise bbox
        let bbox = AABB::new(
            self.center.add_scalar(-self.radius),
            self.center.add_scalar(self.radius),
        );

        let bbox = self.object_to_world.transform_bounding_box(bbox);
        Some(bbox)
    }

    fn sample_pdf(&self, origin: &crate::core::Point3f, v: &Vec3f) -> f32 {
        // TODO: Support transform

        let rec = self.intersect(&Ray::new(origin.clone(), v.clone(), 0.0), 0.001, f32::MAX);

        // TODO: Consider not axis-aligned
        if let Some(si) = rec {
            // TODO: Disk Importance Sampling
            let area = self.radius * self.radius * PI;

            let distance_squared = si.t_hit * si.t_hit * v.norm_squared();
            let cosine = (v.dot(&si.normal) / v.norm()).abs();

            let pdf = distance_squared / (cosine * area);
            return pdf;
        }
        0.0
    }

    fn sample_wi(&self, origin: &Vec3f) -> Vec3f {
        // TODO: Support transform
        self.sampler.sample_direction(origin).normalize()
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

fn point_on_disk(theta: f32, r: f32, center: Vec3f, plane: Plane) -> Vec3f {
    let c = center;
    let a = r * theta.sin();
    let b = r * theta.cos();

    // currently only support axis-aligned
    return match plane {
        Plane::YZ => Vec3f::new(c.x, c.y + a, c.z + b),
        Plane::ZX => Vec3f::new(c.x + b, c.y, c.z + a),
        Plane::XY => Vec3f::new(c.x + a, c.y + b, c.z),
    };
}

impl Sampler for DiskRandomSampler {
    fn sample_direction(&self, origin: &Vec3f) -> Vec3f {
        let disk = &self.disk;
        // @see https://stackoverflow.com/questions/5837572/generate-a-random-point-within-a-circle-uniformly
        let theta = random::f32_r(0.0, 2.0 * PI);
        let r = disk.radius * random::f32().sqrt();

        let random_point = point_on_disk(theta, r, disk.center, disk.plane);
        return random_point - origin;
    }
}

struct DiskRandomFixedSampler {
    sampled_points: Vec<Vec3f>,
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
    fn sample_direction(&self, origin: &Vec3f) -> Vec3f {
        let index = random::usize(0..self.sampled_points.len());
        return self.sampled_points[index] - origin;
    }
}

struct DiskUniformSampler {
    sampled_points: Vec<Vec3f>,
}

impl DiskUniformSampler {
    pub fn new(disk: DiskData, block_size: [i32; 2]) -> Self {
        let mut sampled_points: Vec<Vec3f> = Vec::new();

        for i in 0..block_size[0] {
            for j in 0..block_size[1] {
                let x = i as f32 / block_size[0] as f32;
                let y = j as f32 / block_size[1] as f32;
                let theta = 2.0 * PI * x;
                let r = disk.radius * y.sqrt();

                let random_point = point_on_disk(theta, r, disk.center, disk.plane);
                sampled_points.push(random_point);
            }
        }

        Self { sampled_points }
    }
}

impl Sampler for DiskUniformSampler {
    fn sample_direction(&self, origin: &Vec3f) -> Vec3f {
        let idx = random::usize(0..self.sampled_points.len());
        let random_point = self.sampled_points[idx];
        return random_point - origin;
    }
}

struct DiskBlueNoiseSampler {
    sampled_points: Vec<Vec3f>,
}

impl DiskBlueNoiseSampler {
    pub fn new(disk: DiskData, block_size: [i32; 2]) -> Self {
        let mut sampled_points: Vec<Vec3f> = Vec::new();

        // BlueNoise alg
        let n_points = block_size[0] * block_size[1];
        for idx in 0..n_points {
            let (x, y) = Self::halton_sequense_2d(idx);
            let theta = 2.0 * PI * x;
            let r = disk.radius * y.sqrt();

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
    fn sample_direction(&self, origin: &Vec3f) -> Vec3f {
        let idx = random::usize(0..self.sampled_points.len());
        let random_point = self.sampled_points[idx];
        return random_point - origin;
    }
}
