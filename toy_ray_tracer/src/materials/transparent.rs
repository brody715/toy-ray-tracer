use crate::{
    bxdfs::{DeltaTransparentTransmission, TransparentTransmission},
    core::{Bsdf, Material, Spectrum, TexturePtr},
};

pub struct Transparent {
    pub eta: f32,
    pub roughness: TexturePtr<f32>,
    pub albedo: TexturePtr<Spectrum>,
}

impl Transparent {
    pub fn new(eta: f32, roughness: TexturePtr<f32>, albedo: TexturePtr<Spectrum>) -> Self {
        Self {
            eta,
            roughness,
            albedo,
        }
    }
}

impl Material for Transparent {
    fn compute_bsdf(&self, si: &crate::core::SurfaceInteraction) -> Option<crate::core::Bsdf> {
        let mut bsdf = Bsdf::new(si.normal);

        let albedo = self.albedo.evaluate(si);
        let roughness = self.roughness.evaluate(si);

        if roughness < f32::EPSILON {
            bsdf.set_raw(DeltaTransparentTransmission::new(self.eta, albedo));
        } else {
            bsdf.set_raw(TransparentTransmission::new(self.eta, roughness, albedo));
        }

        Some(bsdf)
    }
}
