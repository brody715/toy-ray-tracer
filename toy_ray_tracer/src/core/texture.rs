use std::sync::Arc;

use crate::core::Vec3;

pub trait Texture: Sync + Send {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub type TexturePtr = Arc<dyn Texture + Sync + Send>;
