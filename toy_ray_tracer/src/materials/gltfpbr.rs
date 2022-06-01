use crate::{
    bxdfs::GltfPbrBxdf,
    core::{Bsdf, Color3, Material, Spectrum, SurfaceInteraction, TexturePtr},
};

pub struct GltfPbrMaterial {
    eta: f32,
    base_color: TexturePtr<Spectrum>,
    metallic: TexturePtr<f32>,
    roughness: TexturePtr<f32>,
    emit: TexturePtr<Spectrum>,
}

impl GltfPbrMaterial {
    pub fn new(
        eta: f32,
        base_color: TexturePtr<Spectrum>,
        metallic: TexturePtr<f32>,
        roughness: TexturePtr<f32>,
        emit: TexturePtr<Spectrum>,
    ) -> Self {
        Self {
            eta,
            base_color,
            metallic,
            roughness,
            emit,
        }
    }
}

impl Material for GltfPbrMaterial {
    fn emitted(&self, si: &crate::core::SurfaceInteraction) -> Color3 {
        return self.emit.evaluate(si);
    }

    fn compute_bsdf(&self, si: &SurfaceInteraction) -> Option<Bsdf> {
        let mut bsdf = Bsdf::new(si.normal);

        bsdf.set_raw(GltfPbrBxdf::new(
            self.eta,
            self.base_color.evaluate(si),
            self.metallic.evaluate(si),
            self.roughness.evaluate(si),
        ));

        Some(bsdf)
    }
}
