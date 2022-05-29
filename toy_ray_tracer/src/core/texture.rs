use std::sync::Arc;

use crate::core::Vec3f;

pub trait Texture: Sync + Send {
    fn value(&self, u: f32, v: f32, p: &Vec3f) -> Vec3f;
}

pub type TexturePtr = Arc<dyn Texture + Sync + Send>;
