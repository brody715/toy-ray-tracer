use crate::core::Texture;

pub struct ConstantTexture<T> {
    value: T,
}

impl<T> ConstantTexture<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Send + Sync + Clone> Texture<T> for ConstantTexture<T> {
    fn evaluate(&self, _si: &crate::core::SurfaceInteraction) -> T {
        self.value.clone()
    }
}
