use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::scene::{RenderOptions, Scene};

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub output_dir: String,
    pub width: usize,
    pub height: usize,
    pub nsamples: i32,
    pub max_depth: i32,
}

impl Into<RenderOptions> for Settings {
    fn into(self) -> RenderOptions {
        RenderOptions {
            width: self.width,
            height: self.height,
            nsamples: self.nsamples,
            max_depth: self.max_depth,
        }
    }
}

pub struct Project {
    pub(crate) name: String,
    pub(crate) settings: Settings,
    pub(crate) scene: Scene,
}

impl Project {
    /// Get a reference to the project's settings.
    #[must_use]
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Get a reference to the project's scene.
    #[must_use]
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    /// Get a reference to the project's name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
