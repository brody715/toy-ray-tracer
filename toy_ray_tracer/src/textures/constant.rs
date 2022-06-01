use crate::core::{Texture, TextureData};

pub struct ConstantTexture<T> {
    value: T,
}

impl<T> ConstantTexture<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: TextureData> Texture<T> for ConstantTexture<T> {
    fn evaluate(&self, _si: &crate::core::SurfaceInteraction) -> T {
        self.value.clone()
    }
}
