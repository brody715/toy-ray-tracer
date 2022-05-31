use std::sync::Arc;

use super::SurfaceInteraction;

pub trait Texture<T>: Sync + Send {
    fn evaluate(&self, si: &SurfaceInteraction) -> T;
}

pub type TexturePtr<T> = Arc<dyn Texture<T> + Sync + Send>;
