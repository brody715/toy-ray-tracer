use crate::{
    geometry::{EnterContext, GeometryVisitor, GeometryWalker},
    core::{Hittable, HittablePtr},
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
        ray: &crate::core::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::core::HitRecord> {
        let rec = self.hittable.hit(&ray, t_min, t_max);
        match rec {
            Some(mut rec) => {
                rec.flip_normal();
                Some(rec)
            }
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<crate::core::AABB> {
        self.hittable.bounding_box(t0, t1)
    }

    fn pdf_value(&self, origin: &crate::core::Point3, v: &crate::core::Vec3) -> f32 {
        self.hittable.pdf_value(origin, v)
    }

    fn random(&self, origin: &crate::core::Vec3) -> crate::core::Vec3 {
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
