use std::f32::consts::PI;

use crate::core::{light::LightTypeFlags, vec3, Light, LightType, Point3f, Ray, Spectrum, Vec3f};

pub struct EnvironmentLight {
    pub background: Spectrum,
}

impl EnvironmentLight {
    pub fn new(background: Spectrum) -> Self {
        Self { background }
    }
}

impl Light for EnvironmentLight {
    fn background_l(&self, _r: &Ray) -> Spectrum {
        self.background
    }

    fn get_flags(&self) -> LightTypeFlags {
        LightType::Infinite.into()
    }

    fn sample_wi(&self, _point: &Point3f) -> Vec3f {
        vec3::random_env_sphere()
    }

    fn sample_pdf(&self, _point: &Point3f, _wi: &crate::core::Vec3f) -> f32 {
        1.0 / (4.0 * PI)
    }
}
