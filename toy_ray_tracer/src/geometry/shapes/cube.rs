use super::rect::AARect;
use super::Plane;
use crate::aabb::AABB;
use crate::geometry::EnterContext;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::MaterialPtr;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Cube {
    p_min: Vec3,
    p_max: Vec3,
    sides: HittableList,
}

impl Cube {
    pub fn new(p_min: Vec3, p_max: Vec3, material: MaterialPtr) -> Self {
        let mut sides = HittableList::new();
        sides.add(AARect::new(
            Plane::XY,
            p_min.x,
            p_max.x,
            p_min.y,
            p_max.y,
            p_max.z,
            material.clone(),
        ));
        sides.add(AARect::new(
            Plane::XY,
            p_min.x,
            p_max.x,
            p_min.y,
            p_max.y,
            p_min.z,
            material.clone(),
        ));
        sides.add(AARect::new(
            Plane::ZX,
            p_min.z,
            p_max.z,
            p_min.x,
            p_max.x,
            p_max.y,
            material.clone(),
        ));
        sides.add(AARect::new(
            Plane::ZX,
            p_min.z,
            p_max.z,
            p_min.x,
            p_max.x,
            p_min.y,
            material.clone(),
        ));
        sides.add(AARect::new(
            Plane::YZ,
            p_min.y,
            p_max.y,
            p_min.z,
            p_max.z,
            p_max.x,
            material.clone(),
        ));
        sides.add(AARect::new(
            Plane::YZ,
            p_min.y,
            p_max.y,
            p_min.z,
            p_max.z,
            p_min.x,
            material,
        ));
        Cube {
            p_min,
            p_max,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(&ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.p_min,
            max: self.p_max,
        })
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_cube(self)
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_cube(EnterContext::new(self));
        self.sides.walk(walker);
    }
}