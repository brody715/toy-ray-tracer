use crate::core::{Material, Spectrum, TexturePtr};

pub struct DiffuseLight {
    emit: TexturePtr<Spectrum>,
}

impl DiffuseLight {
    pub fn new(emit: TexturePtr<Spectrum>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, si: &crate::core::SurfaceInteraction) -> Spectrum {
        self.emit.evaluate(si)
    }
}
