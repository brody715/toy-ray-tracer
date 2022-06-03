use crate::core::Ray;
use crate::core::SurfaceInteraction;
use crate::core::Transform;
use crate::core::Vec3f;
use crate::core::{Shape, AABB};

use super::create_triangles;
use super::shape_list::ShapeList;

pub struct Rect {
    // actual, 2 triangles
    triangles: ShapeList,
}

impl Rect {
    #[must_use]
    pub fn new(p0: Vec3f, p1: Vec3f, object_to_world: Transform) -> Self {
        let axiso = p0.iter().zip(p1.iter()).position(|(l, r)| l == r);

        let k_axis = axiso.unwrap_or(3);

        let (a_axis, b_axis) = match k_axis {
            0 => (1, 2), // yz
            1 => (2, 0), // zx
            2 => (0, 1), // xy
            _ => panic!("only support axis-aligned rect: {}, {}", p0, p1),
        };

        // 2d rect
        let a0 = p0[a_axis].min(p1[a_axis]);
        let a1 = p0[a_axis].max(p1[a_axis]);
        let b0 = p0[b_axis].min(p1[b_axis]);
        let b1 = p0[b_axis].max(p1[b_axis]);
        let k = p0[k_axis];

        let v0 = {
            let mut v = Vec3f::zeros();
            v[a_axis] = a0;
            v[b_axis] = b0;
            v[k_axis] = k;
            v
        };

        let v1 = {
            let mut v = Vec3f::zeros();
            v[a_axis] = a0;
            v[b_axis] = b1;
            v[k_axis] = k;
            v
        };

        let v2 = {
            let mut v = Vec3f::zeros();
            v[a_axis] = a1;
            v[b_axis] = b0;
            v[k_axis] = k;
            v
        };

        let v3 = {
            let mut v = Vec3f::zeros();
            v[a_axis] = a1;
            v[b_axis] = b1;
            v[k_axis] = k;
            v
        };

        let indices = vec![0, 1, 2, 1, 3, 2];
        let positions = vec![v0, v1, v2, v3];

        let triangles = create_triangles(indices, positions, object_to_world).unwrap();

        Self { triangles }
    }
}

impl Shape for Rect {
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.triangles.bounding_box(t0, t1)
    }

    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<SurfaceInteraction> {
        self.triangles.intersect(ray, t_min, t_max)
    }

    fn sample_pdf(&self, origin: &crate::core::Point3f, v: &Vec3f) -> f32 {
        self.triangles.sample_pdf(origin, v)
    }

    fn sample_wi(&self, origin: &Vec3f) -> Vec3f {
        self.triangles.sample_wi(origin)
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        self.triangles.intersect_p(ray)
    }
}
