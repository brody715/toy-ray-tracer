use std::sync::Arc;

use crate::core::Color3;
use crate::core::Ray;
use crate::core::SurfaceInteraction;
use crate::math::PDF;

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub attenuation: Color3,
    pub pdf: Option<Box<dyn PDF>>,
}

pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, rec: &SurfaceInteraction) -> Option<ScatterRecord>;

    fn scattering_pdf(&self, ray: &Ray, rec: &SurfaceInteraction, scattered: &Ray) -> f32;

    fn emitted(&self, _ray: &Ray, _rec: &SurfaceInteraction) -> Color3 {
        Color3::zeros()
    }
}

pub type MaterialPtr = Arc<dyn Material + Sync + Send>;
