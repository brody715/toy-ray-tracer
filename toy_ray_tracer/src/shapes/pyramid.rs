use std::sync::Arc;

use crate::{
    core::Shape,
    core::Vec3f,
    core::{ShapePtr, AABB},
};

use super::{shape_list::ShapeList, Triangle};

pub struct Pyramid {
    // actual, 4 triangles
    items: ShapeList,
}

impl Pyramid {
    pub fn new(vertices: [Vec3f; 4]) -> Self {
        let mut triangles: Vec<Arc<Triangle>> = Vec::new();

        // C_4^3
        for i in 0..4 {
            let av = vertices[(i + 1) % 4];
            let bv = vertices[(i + 2) % 4];
            let cv = vertices[(i + 3) % 4];
            triangles.push(Arc::new(Triangle::new([av, bv, cv], None)))
        }

        let triangles: Vec<ShapePtr> = triangles.into_iter().map(|v| v as ShapePtr).collect();

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
    ) -> Option<crate::core::HitRecord> {
        self.items.intersect(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.items.bounding_box(t0, t1)
    }

    fn pdf_value(&self, origin: &crate::core::Point3f, v: &Vec3f) -> f32 {
        self.items.pdf_value(origin, v)
    }

    fn random(&self, origin: &Vec3f) -> Vec3f {
        self.items.random(origin)
    }
}
