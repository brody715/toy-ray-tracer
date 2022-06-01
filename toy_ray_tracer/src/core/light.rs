use std::sync::Arc;

use super::{Point3f, Ray, Spectrum, Vec3f};
use enumflags2::{bitflags, BitFlags};

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LightType {
    Area,
    Infinite,
    List,
}

pub type LightTypeFlags = BitFlags<LightType>;

pub trait Light {
    // background_l for envrionment light
    fn background_l(&self, r: &Ray) -> Spectrum;

    fn get_flags(&self) -> LightTypeFlags;

    fn sample_wi(&self, point: &Point3f) -> Vec3f;

    fn sample_pdf(&self, point: &Point3f, wi: &Vec3f) -> f32;
}

pub type LightPtr = Arc<dyn Light + Sync + Send>;
