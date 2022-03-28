use crate::vec::Vec3;

// probability density Function
pub trait PDF {
    fn value(&self, direction: &Vec3) -> f32;
    fn generate(&self) -> Vec3;
}
