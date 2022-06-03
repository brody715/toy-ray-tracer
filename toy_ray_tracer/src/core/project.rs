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
    #[serde(default = "Settings::default_mis_weight")]
    pub mis_weight: f32,
}

impl Settings {
    pub fn default_mis_weight() -> f32 {
        return 0.5;
    }

    pub fn get_aspect(&self) -> f32 {
        return self.width as f32 / self.height as f32;
    }
}

pub struct Project {
    pub(crate) name: String,
    pub(crate) settings: Settings,
    pub(crate) scene: Scene,
}

impl Project {
    pub fn new(name: String, settings: Settings, scene: Scene) -> Self {
        Self {
            name,
            settings,
            scene,
        }
    }

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
