use derive_new::new;

use crate::{
    camera::Camera,
    environment::{Sky, SkyPtr},
    hittable::{Hittable, HittablePtr},
};

#[derive(new)]
pub struct Scene {
    pub(crate) camera: Camera,
    pub(crate) world: HittablePtr,
    pub(crate) lights: HittablePtr,
    pub(crate) sky: SkyPtr,
    #[allow(dead_code)]
    pub(crate) name: String,
    #[allow(dead_code)]
    pub(crate) description: String,
}

impl Scene {
    #[allow(dead_code)]
    pub fn set_camera(&mut self, cam: Camera) {
        self.camera = cam;
    }

    #[allow(dead_code)]
    pub fn set_world(&mut self, world: HittablePtr) {
        self.world = world;
    }

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

    /// Set the scene's name.
    #[allow(dead_code)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get a reference to the scene's lights.
    #[must_use]
    pub fn lights(&self) -> &dyn Hittable {
        self.lights.as_ref()
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
