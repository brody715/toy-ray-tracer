use std::sync::Arc;

use crate::core::Color3;
use crate::core::HitRecord;
use crate::core::Ray;
use crate::math::PDF;

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub attenuation: Color3,
    pub pdf: Option<Box<dyn PDF>>,
}

pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;

    fn scattering_pdf(&self, ray: &Ray, rec: &HitRecord, scattered: &Ray) -> f32;

    fn emitted(&self, _ray: &Ray, _rec: &HitRecord) -> Color3 {
        Color3::zeros()
    }
}

pub type MaterialPtr = Arc<dyn Material + Sync + Send>;
