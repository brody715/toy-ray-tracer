use std::rc::Rc;

use super::{Color3, Vec3f};

#[derive(Clone)]
pub struct Bsdf {
    pub eta: f32,
    // shading and geometric normal
    pub normal: Vec3f,
    pub bxdfs: Vec<BxdfPtr>,
}

pub trait Bxdf {
    fn f(&self, wo: &Vec3f, wi: &Vec3f) -> Color3;

    // TODO: Add importance sampling
}

pub type BxdfPtr = Rc<dyn Bxdf>;
