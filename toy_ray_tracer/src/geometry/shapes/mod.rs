mod cube;
mod cylinder;
mod disk;
mod mesh;
mod plane;
mod pyramid;
mod rect;
mod sphere;
mod triangle;

pub use cube::Cube;
pub use cylinder::Cylinder;
pub use disk::Disk;
pub use mesh::{Mesh, MeshLoadOptions};
pub use plane::Plane;
pub use pyramid::Pyramid;
pub use rect::{AARect, Rect};
pub use sphere::{MovingSphere, Sphere};
pub use triangle::Triangle;

use crate::{hittable::Hittable, vec::vec3};

// Use as light object, if no light provided by scene
pub struct SkyLight {}

impl Hittable for SkyLight {
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
        1.0
    }

    fn random(&self, _origin: &crate::vec::Vec3) -> crate::vec::Vec3 {
        // WARNING: not distributed in sphere, try random_in_unit_sphere
        vec3::random().normalize()
    }

    fn accept(&self, visitor: &mut dyn super::GeometryVisitor) {
        visitor.visit_sky_light(self)
    }

    fn walk(&self, walker: &mut dyn super::GeometryWalker) {
        walker.enter_sky_light(super::EnterContext::new(self));
    }
}
