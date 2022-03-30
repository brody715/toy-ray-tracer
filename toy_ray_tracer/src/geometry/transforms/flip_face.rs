use crate::{
    geometry::{EnterContext, GeometryVisitor, GeometryWalker},
    hittable::{Hittable, HittablePtr},
};

pub struct FlipFace {
    hittable: HittablePtr,
}

impl FlipFace {
    #[must_use]
    pub fn new(hittable: HittablePtr) -> Self {
        Self { hittable }
    }
}

impl Hittable for FlipFace {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        let rec = self.hittable.hit(&ray, t_min, t_max);
        match rec {
            Some(mut rec) => {
                rec.flip_normal();
                Some(rec)
            }
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<crate::aabb::AABB> {
        self.hittable.bounding_box(t0, t1)
    }

    fn pdf_value(&self, origin: &crate::vec::Point3, v: &crate::vec::Vec3) -> f32 {
        self.hittable.pdf_value(origin, v)
    }

    fn random(&self, origin: &crate::vec::Vec3) -> crate::vec::Vec3 {
        self.hittable.random(origin)
    }

    fn accept(&self, visitor: &mut dyn GeometryVisitor) {
        visitor.visit_flip_face(self)
    }

    fn walk(&self, walker: &mut dyn GeometryWalker) {
        walker.enter_flip_face(EnterContext::new(self));
        self.hittable.walk(walker);
    }
}
