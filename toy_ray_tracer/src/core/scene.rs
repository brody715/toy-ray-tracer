use std::sync::Arc;

use crate::{core::Camera, lights::LightList};

use super::{LightPtr, PrimitiveContainerPtr, PrimitivePtr};

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

pub struct SceneBundle {
    pub primitives: Vec<PrimitivePtr>,
    pub lights: Vec<LightPtr>,
    pub camera: Option<Arc<Camera>>,
}

impl Default for SceneBundle {
    fn default() -> Self {
        Self {
            primitives: Vec::new(),
            lights: Vec::new(),
            camera: Default::default(),
        }
    }
}

impl SceneBundle {
    pub fn union_assign(&mut self, mut other: SceneBundle) {
        self.primitives.append(&mut other.primitives);
        self.lights.append(&mut other.lights);
        if let Some(camera) = other.camera {
            self.camera = Some(camera);
        }
    }
}
