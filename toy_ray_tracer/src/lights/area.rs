use crate::core::{Light, LightFlags, PrimitivePtr, Spectrum};

pub struct AreaLight {
    primitive: PrimitivePtr,
}

impl AreaLight {
    pub fn new(primitive: PrimitivePtr) -> Self {
        Self { primitive }
    }
}

impl Light for AreaLight {
    fn color(&self, _r: &crate::core::Ray) -> Spectrum {
        todo!()
    }

    fn get_flags(&self) -> u8 {
        return LightFlags::Area as u8;
    }

    fn get_light_primitive(&self) -> Option<&dyn crate::core::Primitive> {
        Some(self.primitive.as_ref())
    }
}
