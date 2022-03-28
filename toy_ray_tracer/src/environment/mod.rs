use std::sync::Arc;

use crate::{ray::Ray, vec::Color3};

pub trait Sky: Send + Sync {
    fn color(&self, r: &Ray) -> Color3;
}

pub type SkyPtr = Arc<dyn Sky + Send + Sync>;

pub struct SolidSky {
    pub background: Color3,
}

impl Sky for SolidSky {
    fn color(&self, _r: &Ray) -> Color3 {
        self.background
    }
}
