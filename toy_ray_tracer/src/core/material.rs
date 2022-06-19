use std::sync::Arc;

use crate::core::SurfaceInteraction;

use super::Bsdf;
use super::Color3;

pub trait Material: Sync {
    fn emission(&self, _si: &SurfaceInteraction) -> Color3 {
        Color3::zeros()
    }

    fn compute_bsdf(&self, _si: &SurfaceInteraction) -> Option<Bsdf> {
        None
    }
}

pub type MaterialPtr = Arc<dyn Material + Sync + Send>;
