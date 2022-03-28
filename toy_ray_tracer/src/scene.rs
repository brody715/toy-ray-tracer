use crate::{
    camera::Camera,
    environment::{Sky, SkyPtr},
    hittable::{Hittable, HittablePtr},
};

pub struct Scene {
    pub(crate) camera: Camera,
    pub(crate) world: HittablePtr,
    pub(crate) light_shape: HittablePtr,
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

    pub fn world(&self) -> &dyn Hittable {
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
    pub fn light_shape(&self) -> &dyn Hittable {
        self.light_shape.as_ref()
    }
}

#[derive(Debug, Clone, Copy, clap::Parser)]
pub struct RenderOptions {
    #[clap(long, help = "image width", default_value_t = 800)]
    pub width: usize,
    #[clap(long, help = "image height", default_value_t = 800)]
    pub height: usize,
    #[clap(long, help = "number of samples", default_value_t = 100)]
    pub nsamples: i32,
    #[clap(long, help = "max depth of ray tracing", default_value_t = 15)]
    pub max_depth: i32,
}

impl RenderOptions {
    #[allow(dead_code)]
    pub fn aspect(&self) -> f32 {
        return self.width as f32 / self.height as f32;
    }
}
