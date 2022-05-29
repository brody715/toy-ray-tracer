use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::Scene;

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub output_dir: String,
    pub width: usize,
    pub height: usize,
    pub nsamples: i32,
    pub max_depth: i32,
    #[serde(default = "Settings::default_pdf_weight")]
    pub weight: f32,
}

impl Settings {
    pub fn default_pdf_weight() -> f32 {
        return 0.5;
    }
}

pub struct Project {
    pub(crate) name: String,
    pub(crate) settings: Settings,
    pub(crate) scene: Scene,
}

impl Project {
    pub fn new(name: String, settings: Settings, scene: Scene) -> Self { Self { name, settings, scene } }

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
