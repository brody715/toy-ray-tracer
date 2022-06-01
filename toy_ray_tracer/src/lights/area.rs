use crate::core::{
    light::LightTypeFlags, Light, LightType, Point3f, PrimitivePtr, Spectrum, Vec3f,
};

pub struct AreaLight {
    primitive: PrimitivePtr,
}

impl AreaLight {
    pub fn new(primitive: PrimitivePtr) -> Self {
        Self { primitive }
    }
}

impl Light for AreaLight {
    fn background_l(&self, _r: &crate::core::Ray) -> Spectrum {
        todo!()
    }

    fn get_flags(&self) -> LightTypeFlags {
        LightType::Area.into()
    }

    fn sample_wi(&self, point: &Point3f) -> Vec3f {
        let prim = &self.primitive;
        prim.sample_wi(point)
    }

    fn sample_pdf(&self, point: &Point3f, wi: &Vec3f) -> f32 {
        let prim = self.primitive.as_ref();
        prim.sample_pdf(point, &wi)
    }
}
