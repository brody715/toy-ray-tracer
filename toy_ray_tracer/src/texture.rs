use crate::vec::Vec3;

pub trait Texture: Sync {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}
