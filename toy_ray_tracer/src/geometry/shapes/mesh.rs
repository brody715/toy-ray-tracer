use std::sync::Arc;

use crate::{
    geometry::{shapes::Triangle, EnterContext},
    hittable::{Hittable, HittablePtr},
    hittable_list::HittableList,
    vec::Vec3,
};

pub struct Mesh {
    _vertices: Vec<Vec3>,
    _faces: Vec<Arc<Triangle>>,
    items: HittablePtr,
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, faces: Vec<Arc<Triangle>>) -> Self {
        let items: Vec<HittablePtr> = faces
            .clone()
            .into_iter()
            .map(|v| v as HittablePtr)
            .collect();
        let items = Arc::new(HittableList::from(items));

        Self {
            _vertices: vertices,
            _faces: faces,
            items,
        }
    }
}

impl Hittable for Mesh {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        self.items.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<crate::aabb::AABB> {
        self.items.bounding_box(t0, t1)
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_mesh(self)
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_mesh(EnterContext::new(self));

        self.items.walk(walker);
    }
}
