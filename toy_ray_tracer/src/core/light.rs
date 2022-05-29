use std::sync::Arc;

use super::{Ray, Spectrum, Primitive};

#[repr(u8)]
pub enum LightFlags {
    Area = 4,
    Infinite = 8,
}

pub trait Light {
    fn color(&self, r: &Ray) -> Spectrum;

    fn get_flags(&self) -> u8;

    fn get_light_primitive(&self) -> Option<& dyn Primitive> {
        None
    }
}

pub fn is_area_light(light: &dyn Light) -> bool {
    light.get_flags() & LightFlags::Area as u8 != 0
}

pub fn is_infinite_light(light: &dyn Light) -> bool {
    light.get_flags() & LightFlags::Infinite as u8 != 0
}

pub type LightPtr = Arc<dyn Light + Sync + Send>;
