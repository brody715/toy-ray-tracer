use std::sync::Arc;

use super::SurfaceInteraction;

pub trait TextureData: Sync + Send + Clone {}

impl<T> TextureData for T where T: Sync + Send + Clone {}

pub trait Texture<T: TextureData>: Sync + Send {
    fn evaluate(&self, si: &SurfaceInteraction) -> T;
}

pub type TexturePtr<T> = Arc<dyn Texture<T> + Sync + Send>;
