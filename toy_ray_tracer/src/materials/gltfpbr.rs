use crate::{
    bxdfs::GltfPbrBxdf,
    core::{Bsdf, Color3, Material, Spectrum, SurfaceInteraction, TexturePtr},
};

use super::clamp_roughness;

pub struct GltfPbrMaterial {
    eta: f32,
    base_color: TexturePtr<Spectrum>,
    metallic: TexturePtr<f32>,
    roughness: TexturePtr<f32>,
    emission: TexturePtr<Spectrum>,
}

impl GltfPbrMaterial {
    pub fn new(
        eta: f32,
        base_color: TexturePtr<Spectrum>,
        metallic: TexturePtr<f32>,
        roughness: TexturePtr<f32>,
        emission: TexturePtr<Spectrum>,
    ) -> Self {
        Self {
            eta,
            base_color,
            metallic,
            roughness,
            emission,
        }
    }
}

impl Material for GltfPbrMaterial {
    fn emission(&self, si: &crate::core::SurfaceInteraction) -> Color3 {
        self.emission.evaluate(si)
    }

    fn compute_bsdf(&self, si: &SurfaceInteraction) -> Option<Bsdf> {
        let mut bsdf = Bsdf::new(si.normal);

        let base_color = self.base_color.evaluate(si);
        let roughness = self.roughness.evaluate(si);
        let metallic = self.metallic.evaluate(si);

        let roughness = clamp_roughness(roughness);

        bsdf.set_raw(GltfPbrBxdf::new(self.eta, base_color, roughness, metallic));

        Some(bsdf)
    }
}
