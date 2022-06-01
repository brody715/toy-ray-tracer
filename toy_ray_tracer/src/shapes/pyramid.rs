use std::sync::Arc;

use crate::{
    core::Shape,
    core::Vec3f,
    core::{ShapePtr, AABB},
};

use super::{shape_list::ShapeList, triangle::TriangleMeshStorage, Triangle};

pub struct Pyramid {
    // actual, 4 triangles
    items: ShapeList,
}

impl Pyramid {
    pub fn new(vertices: [Vec3f; 4]) -> Self {
        let vertex_indices = vec![0, 1, 2, 0, 2, 3, 0, 3, 1, 1, 3, 2];
        let positions = vertices.to_vec();

        let mesh = Arc::new(
            TriangleMeshStorage::try_new(4, vertex_indices, positions, vec![], vec![]).unwrap(),
        );

        let mut triangles: Vec<ShapePtr> = Vec::new();

        // C_4^3
        for i in 0..4 {
            triangles.push(Arc::new(Triangle::new(i, mesh.clone())));
        }

        let items = ShapeList::from(triangles);

        Self { items }
    }
}

impl Shape for Pyramid {
    fn intersect(
        &self,
        ray: &crate::core::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::core::SurfaceInteraction> {
        self.items.intersect(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.items.bounding_box(t0, t1)
    }

    fn sample_pdf(&self, origin: &crate::core::Point3f, v: &Vec3f) -> f32 {
        self.items.sample_pdf(origin, v)
    }

    fn sample_wi(&self, origin: &Vec3f) -> Vec3f {
        self.items.sample_wi(origin)
    }
}
