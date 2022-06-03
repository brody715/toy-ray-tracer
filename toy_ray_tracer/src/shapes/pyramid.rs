use crate::{
    core::Shape,
    core::AABB,
    core::{Transform, Vec3f},
};

use super::{create_triangles, shape_list::ShapeList};

pub struct Pyramid {
    // 4 triangles
    triangles: ShapeList,
}

impl Pyramid {
    pub fn new(vertices: [Vec3f; 4], object_to_world: Transform) -> Self {
        let vertex_indices = vec![0, 1, 2, 0, 2, 3, 0, 3, 1, 1, 3, 2];
        let positions = vertices.to_vec();
        let triangles = create_triangles(vertex_indices, positions, object_to_world).unwrap();

        Self { triangles }
    }
}

impl Shape for Pyramid {
    fn intersect(
        &self,
        ray: &crate::core::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::core::SurfaceInteraction> {
        self.triangles.intersect(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.triangles.bounding_box(t0, t1)
    }

    fn sample_pdf(&self, origin: &crate::core::Point3f, v: &Vec3f) -> f32 {
        self.triangles.sample_pdf(origin, v)
    }

    fn sample_wi(&self, origin: &Vec3f) -> Vec3f {
        self.triangles.sample_wi(origin)
    }
}
