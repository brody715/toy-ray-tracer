use crate::core::{Material, Spectrum, TexturePtr};

pub struct DiffuseLight {
    emission: TexturePtr<Spectrum>,
}

impl DiffuseLight {
    pub fn new(emission: TexturePtr<Spectrum>) -> Self {
        Self { emission }
    }
}

impl Material for DiffuseLight {
    fn emission(&self, si: &crate::core::SurfaceInteraction) -> Spectrum {
        self.emission.evaluate(si)
    }

    fn compute_bsdf(&self, _si: &crate::core::SurfaceInteraction) -> Option<crate::core::Bsdf> {
        None
    }
}
