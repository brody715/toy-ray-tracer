use std::sync::Arc;

use crate::{
    aabb::AABB,
    geometry::EnterContext,
    hittable::{Hittable, HittablePtr},
    hittable_list::HittableList,
    material::MaterialPtr,
    vec::{vec3, Vec3},
};

use super::Triangle;

pub struct Pyramid {
    vertices: [Vec3; 4],
    // actual
    items: HittablePtr,
}

impl Pyramid {
    pub fn new(vertices: [Vec3; 4], material: MaterialPtr) -> Self {
        let mut triangles: Vec<Arc<Triangle>> = Vec::new();

        // C_4^3
        for i in 0..4 {
            let av = vertices[(i + 1) % 4];
            let bv = vertices[(i + 2) % 4];
            let cv = vertices[(i + 3) % 4];
            triangles.push(Arc::new(Triangle::new(
                [av, bv, cv],
                None,
                material.clone(),
            )))
        }

        let triangles: Vec<HittablePtr> = triangles.into_iter().map(|v| v as HittablePtr).collect();

        let items = Arc::new(HittableList::from(triangles));

        Self { vertices, items }
    }
}

impl Hittable for Pyramid {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        self.items.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = self
            .vertices
            .iter()
            .fold(vec3::INF, |acc, v| vec3::min(&acc, &v));

        let max = self
            .vertices
            .iter()
            .fold(-vec3::INF, |acc, v| vec3::max(&acc, &v));

        Some(AABB::new(min, max))
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_pyramid(self);
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_pyramid(EnterContext::new(self));
        self.items.walk(walker);
    }

    fn pdf_value(&self, origin: &crate::vec::Point3, v: &Vec3) -> f32 {
        self.items.pdf_value(origin, v)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        self.items.random(origin)
    }
}
