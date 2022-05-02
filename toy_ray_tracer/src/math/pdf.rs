use crate::vec::Vec3;

// probability density Function
pub trait PDF {
    // generate pdf value p(w_o)
    fn pdf_value(&self, direction: &Vec3) -> f32;
    // generate ray direction w_o
    fn generate_direction(&self) -> Vec3;
}