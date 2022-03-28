use std::collections::HashSet;

use crate::{
    geometry::EnterContext,
    hittable::{Hittable, HittablePtr},
};

pub struct TagsHittable {
    pub(crate) tags: HashSet<String>,
    pub(crate) child: HittablePtr,
}

impl TagsHittable {
    pub fn new(tags: HashSet<String>, child: HittablePtr) -> Self {
        Self { tags, child }
    }
}

impl Hittable for TagsHittable {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        self.child.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<crate::aabb::AABB> {
        self.child.bounding_box(t0, t1)
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_tags_hittable(self);
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_tags_hittable(EnterContext::new(self));
        self.child.walk(walker);
    }

    fn pdf_value(&self, origin: &crate::vec::Point3, v: &crate::vec::Vec3) -> f32 {
        self.child.pdf_value(origin, v)
    }

    fn random(&self, origin: &crate::vec::Vec3) -> crate::vec::Vec3 {
        self.child.random(origin)
    }
}
