mod cube;
mod rect;
mod sphere;

pub use cube::Cube;
pub use rect::{AARect, Plane, Rect};
pub use sphere::{MovingSphere, Sphere};

use crate::hittable::Hittable;

// Use as light object, if no light provided by scene
pub struct NopLight {}

impl Hittable for NopLight {
    fn hit(
        &self,
        _ray: &crate::ray::Ray,
        _t_min: f32,
        _t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<crate::aabb::AABB> {
        None
    }

    fn pdf_value(&self, _origin: &crate::vec::Point3, _v: &crate::vec::Vec3) -> f32 {
        0.0
    }

    fn random(&self, _origin: &crate::vec::Vec3) -> crate::vec::Vec3 {
        crate::vec::Vec3::new(1.0, 0.0, 0.0)
    }

    fn accept(&self, visitor: &mut dyn super::GeometryVisitor) {
        visitor.visit_nop_light(self)
    }

    fn walk(&self, walker: &mut dyn super::GeometryWalker) {
        walker.enter_nop_light(super::EnterContext::new(self));
    }
}
