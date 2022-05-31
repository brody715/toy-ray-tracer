use crate::core::{Primitive, PrimitivePtr};

pub struct FlipFacePrimitive {
    primitive: PrimitivePtr,
}

impl FlipFacePrimitive {
    pub fn new(primitive: PrimitivePtr) -> Self {
        Self { primitive }
    }
}

impl Primitive for FlipFacePrimitive {
    fn intersect(
        &self,
        ray: &crate::core::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::core::SurfaceInteraction> {
        let rec = self.primitive.intersect(ray, t_min, t_max);
        let rec = rec.map(|mut rec| {
            rec.flip_normal();
            rec
        });

        rec
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<crate::core::AABB> {
        self.primitive.bounding_box(t0, t1)
    }

    fn set_sampler(&mut self, _sampler_type: crate::math::SamplerType) {
        unimplemented!()
    }

    fn pdf_value(&self, origin: &crate::core::Point3f, v: &crate::core::Vec3f) -> f32 {
        self.primitive.pdf_value(origin, v)
    }

    fn random(&self, origin: &crate::core::Vec3f) -> crate::core::Vec3f {
        self.primitive.random(origin)
    }
}
