use std::sync::Arc;

use derive_new::new;

use crate::{
    camera::{Camera, CameraOpt},
    hittable::{EmptyHittable, Hittable, HittablePtr},
    vec::{Color3, Vec3},
};

#[derive(new)]
pub struct Scene {
    pub(crate) camera: Camera,
    pub(crate) world: HittablePtr,
    pub(crate) sky: Color3,
    #[allow(dead_code)]
    pub(crate) name: String,
    #[allow(dead_code)]
    pub(crate) description: String,
}

impl Scene {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            camera: Self::default_camera(),
            world: Arc::new(EmptyHittable::new()),
            sky: Vec3::zeros(),
            name: String::from("[no-name]"),
            description: String::from(""),
        }
    }

    pub fn default_camera() -> Camera {
        Camera::new(CameraOpt {
            look_from: Vec3::new(13.0, 2.0, 3.0),
            look_at: Vec3::zeros(),
            view_up: Vec3::new(0.0, 1.0, 0.0),
            vertical_fov: 20.0,
            aspect: 1.0,
            aperture: 0.0,
            focus_dist: 10.0,
            time0: 0.0,
            time1: 1.0,
        })
    }

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
    pub fn background(&self) -> Color3 {
        self.sky
    }

    /// Set the scene's background.
    #[allow(dead_code)]
    pub fn set_background(&mut self, background: Color3) {
        self.sky = background;
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
    pub fn aspect(&self) -> f32 {
        return self.width as f32 / self.height as f32;
    }
}
