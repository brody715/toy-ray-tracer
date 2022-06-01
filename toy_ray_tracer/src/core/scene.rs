use std::sync::Arc;

use crate::{core::Camera, lights::LightList};

use super::{LightPtr, PrimitiveContainerPtr};

pub struct Scene {
    pub(crate) camera: Arc<Camera>,
    pub(crate) world: PrimitiveContainerPtr,
    pub lights: LightList,
}

impl Scene {
    pub fn new(camera: Arc<Camera>, world: PrimitiveContainerPtr, lights: Vec<LightPtr>) -> Self {
        Self {
            camera,
            world,
            lights: lights.into(),
        }
    }
}
