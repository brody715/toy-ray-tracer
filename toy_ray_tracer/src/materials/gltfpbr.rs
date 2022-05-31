use crate::core::{Color3, Material, Ray, ScatterRecord, Spectrum, SurfaceInteraction, TexturePtr};

pub struct GltfPbrMaterial {
    ior: f32,
    base_color: TexturePtr<Spectrum>,
    metallic: TexturePtr<f32>,
    roughness: TexturePtr<f32>,
    emitted_color: TexturePtr<Spectrum>,
}

impl GltfPbrMaterial {
    pub fn new(
        ior: f32,
        base_color: TexturePtr<Spectrum>,
        metallic: TexturePtr<f32>,
        roughness: TexturePtr<f32>,
        emitted_color: TexturePtr<Spectrum>,
    ) -> Self {
        Self {
            ior,
            base_color,
            metallic,
            roughness,
            emitted_color,
        }
    }
}

impl Material for GltfPbrMaterial {
    fn scatter(&self, ray: &Ray, si: &SurfaceInteraction) -> Option<ScatterRecord> {
        todo!()
    }

    fn scattering_pdf(
        &self,
        ray: &crate::core::Ray,
        si: &crate::core::SurfaceInteraction,
        scattered: &crate::core::Ray,
    ) -> f32 {
        todo!()
    }

    fn emitted(&self, _ray: &crate::core::Ray, si: &crate::core::SurfaceInteraction) -> Color3 {
        return self.emitted_color.evaluate(si);
    }
}
