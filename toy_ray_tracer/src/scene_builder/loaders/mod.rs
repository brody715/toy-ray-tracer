mod gltf;
mod js;
mod mesh;

pub use gltf::load_gltf_scenes;
pub use mesh::{MeshBundle, MeshLoader};

use anyhow::{anyhow, Ok};

use super::types::ProjectConfig;
use js::load_from_js;
use std::{ffi::OsStr, path::Path};

pub fn load_project_config<P: AsRef<Path>>(path: P) -> anyhow::Result<ProjectConfig> {
    let extension = path.as_ref().extension().and_then(OsStr::to_str);

    if let Some(ext) = extension {
        let content = std::fs::read_to_string(path.as_ref())?;
        return match ext {
            "js" => load_from_js(&content, path),
            "json" => load_from_json(&content),
            _ => Err(anyhow!(
                "unsupported file format: {}",
                path.as_ref().display()
            )),
        };
    }

    Err(anyhow!(
        "unsupported file format: {}",
        path.as_ref().display()
    ))
}

pub fn load_from_json(str: &str) -> anyhow::Result<ProjectConfig> {
    let project_config: ProjectConfig = serde_json::from_str(str)?;
    Ok(project_config)
}
