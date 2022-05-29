use super::rect::AARect;
use super::shape_list::ShapeList;
use super::Plane;
use crate::core::MaterialPtr;
use crate::core::Ray;
use crate::core::Shape;
use crate::core::Vec3f;
use crate::core::AABB;
use crate::core::{HitRecord, Primitive};

pub struct Cube {
    p_min: Vec3f,
    p_max: Vec3f,
    sides: ShapeList,
}

impl Cube {
    pub fn new(p_min: Vec3f, p_max: Vec3f, material: MaterialPtr) -> Self {
        let mut sides = ShapeList::new();
        sides.emplace_back(AARect::new(
            Plane::XY,
            p_min.x,
            p_max.x,
            p_min.y,
            p_max.y,
            p_max.z,
        ));
        sides.emplace_back(AARect::new(
            Plane::XY,
            p_min.x,
            p_max.x,
            p_min.y,
            p_max.y,
            p_min.z,
        ));
        sides.emplace_back(AARect::new(
            Plane::ZX,
            p_min.z,
            p_max.z,
            p_min.x,
            p_max.x,
            p_max.y,
        ));
        sides.emplace_back(AARect::new(
            Plane::ZX,
            p_min.z,
            p_max.z,
            p_min.x,
            p_max.x,
            p_min.y,
        ));
        sides.emplace_back(AARect::new(
            Plane::YZ,
            p_min.y,
            p_max.y,
            p_min.z,
            p_max.z,
            p_max.x,
        ));
        sides.emplace_back(AARect::new(
            Plane::YZ,
            p_min.y,
            p_max.y,
            p_min.z,
            p_max.z,
            p_min.x,
        ));
        Cube {
            p_min,
            p_max,
            sides,
        }
    }
}

impl Primitive for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.intersect(&ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.p_min,
            max: self.p_max,
        })
    }
}
