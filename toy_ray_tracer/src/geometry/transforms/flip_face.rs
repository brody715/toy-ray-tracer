use crate::{
    core::{Primitive, PrimitivePtr},
};

pub struct FlipFace {
    hittable: PrimitivePtr,
}

impl FlipFace {
    #[must_use]
    pub fn new(hittable: PrimitivePtr) -> Self {
        Self { hittable }
    }
}

impl Primitive for FlipFace {
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

    fn pdf_value(&self, origin: &crate::core::Point3f, v: &crate::core::Vec3f) -> f32 {
        self.hittable.pdf_value(origin, v)
    }

    fn random(&self, origin: &crate::core::Vec3f) -> crate::core::Vec3f {
        self.hittable.random(origin)
    }
}
