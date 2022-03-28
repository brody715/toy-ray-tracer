use crate::aabb::AABB;
use crate::geometry::EnterContext;
use crate::hittable::{HitRecord, Hittable, HittablePtr};
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Translate {
    hittable: HittablePtr,
    offset: Vec3,
}

impl Translate {
    pub fn new(hittable: HittablePtr, offset: Vec3) -> Self {
        Translate { hittable, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        self.hittable.hit(&moved_ray, t_min, t_max).map(|mut hit| {
            hit.p += self.offset;
            hit
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hittable.bounding_box(t0, t1).map(|mut b| {
            b.min += self.offset;
            b.max += self.offset;
            b
        })
    }

    fn pdf_value(&self, origin: &crate::vec::Point3, v: &Vec3) -> f32 {
        self.hittable.pdf_value(origin, v)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        self.hittable.random(origin)
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_translate(self)
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_translate(EnterContext::new(self));
        self.hittable.walk(walker);
    }
}
