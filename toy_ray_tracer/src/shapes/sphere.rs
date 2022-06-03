use crate::core::Point3f;
use crate::core::Ray;
use crate::core::Shape;
use crate::core::SurfaceInteraction;
use crate::core::Transform;
use crate::core::AABB;
use crate::core::{vec3, Point2f, Vec3f};
use crate::math::ONB;
use nalgebra::Vector3;
use std::f32;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3f,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32, object_to_world: Transform) -> Self {
        // TODO: fix transform radius

        let center = object_to_world.transform_point3(&center);

        Sphere { center, radius }
    }
}

fn get_sphere_uv(p: &Vec3f) -> (f32, f32) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI);
    let v = (theta + f32::consts::FRAC_PI_2) / f32::consts::PI;
    (u, v)
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<SurfaceInteraction> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
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
            let normal = (p - self.center) / self.radius;
            let (u, v) = get_sphere_uv(&normal);
            let si = SurfaceInteraction::new(t, p, Point2f::new(u, v), -ray.direction(), normal);
            return Some(si);
        }

        return None;
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return false;
        }
        return true;
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let radius = Vector3::new(self.radius, self.radius, self.radius);
        let min = self.center - radius;
        let max = self.center + radius;
        Some(AABB { min, max })
    }

    fn sample_pdf(&self, point: &Point3f, wi: &Vec3f) -> f32 {
        let ray = Ray::new(point.clone(), wi.clone(), 0.0);
        if self.intersect_p(&ray) {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center - point).norm_squared()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            return 1.0 / solid_angle;
        }

        return 0.0;
    }

    fn sample_wi(&self, origin: &Vec3f) -> Vec3f {
        let direction = self.center - origin;
        let distance_squared = direction.norm_squared();
        let uvw = ONB::build_form_w(&direction);
        return uvw.local(vec3::random_to_sphere(self.radius, distance_squared));
    }
}
