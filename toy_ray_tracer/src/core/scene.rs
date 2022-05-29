use crate::{
    core::Camera,
    environment::{Sky, SkyPtr},
    core::{Primitive, PrimitivePtr},
};

pub struct Scene {
    pub(crate) camera: Camera,
    pub(crate) world: PrimitivePtr,
    pub(crate) light_shape: PrimitivePtr,
    pub(crate) sky: SkyPtr,
    #[allow(dead_code)]
    pub(crate) name: String,
    #[allow(dead_code)]
    pub(crate) description: String,
}

impl Scene {
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn world(&self) -> &dyn Primitive {
        self.world.as_ref()
    }

    /// Get the scene's background.
    #[must_use]
    pub fn sky(&self) -> &dyn Sky {
        self.sky.as_ref()
    }

    /// Get a reference to the scene's name.
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the scene's light_shape.
    #[must_use]
    pub fn light_shape(&self) -> &dyn Primitive {
        self.light_shape.as_ref()
    }
}

