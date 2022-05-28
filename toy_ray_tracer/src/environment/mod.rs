use std::sync::Arc;

use crate::core::{Color3, Ray};

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
