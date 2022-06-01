use std::sync::Arc;

use crate::core::Ray;
use crate::core::SurfaceInteraction;
use crate::math::PDF;

use super::Color3;
use super::Vec3f;

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub attenuation: Vec3f,
    pub pdf: Option<Box<dyn PDF>>,
}



pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, si: &SurfaceInteraction) -> Option<ScatterRecord>;

    fn scattering_pdf(&self, ray: &Ray, si: &SurfaceInteraction, scattered: &Ray) -> f32;

    fn emitted(&self, _ray: &Ray, _si: &SurfaceInteraction) -> Color3 {
        Color3::zeros()
    }
}

pub type MaterialPtr = Arc<dyn Material + Sync + Send>;
