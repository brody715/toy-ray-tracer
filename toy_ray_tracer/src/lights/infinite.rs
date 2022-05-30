use crate::core::{Light, LightFlags, Ray, Spectrum};

pub struct EnvironmentLight {
    pub background: Spectrum,
}

impl EnvironmentLight {
    pub fn new(background: Spectrum) -> Self { Self { background } }
}

impl Light for EnvironmentLight {
    fn color(&self, _r: &Ray) -> Spectrum {
        self.background
    }

    fn get_flags(&self) -> u8 {
        LightFlags::Infinite as u8
    }
}
