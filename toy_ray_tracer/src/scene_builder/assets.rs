use anyhow::{Ok, Result};
use std::path::{Path, PathBuf};
use url::Url;

use crate::core::Image;

pub struct AssetsManager {
    assets_dir: PathBuf,
    project_dir: PathBuf,
}

impl AssetsManager {
    pub fn new<P1: AsRef<Path>, P2: AsRef<Path>>(assets_dir: P1, project_dir: P2) -> Self {
        AssetsManager {
            assets_dir: assets_dir.as_ref().to_path_buf(),
            project_dir: project_dir.as_ref().to_path_buf(),
        }
    }

    pub fn load_path(&self, uri: &str) -> Result<PathBuf> {
        let url = Url::parse(uri);

        if let Some(err) = url.clone().err() {
            if err == url::ParseError::RelativeUrlWithoutBase {
                return Ok(self.project_dir.join(uri));
            }
            return Err(anyhow::Error::from(err));
        }

        let url = url?;

        let mut path = PathBuf::new();
        if url.scheme() == "assets" {
            path.push(&self.assets_dir);
        } else {
            path.push(&self.project_dir);
        }

        if url.has_host() {
            anyhow::bail!("Host not supported, got uri {}", url);
        }

        let url_path = Path::new(url.path());

        // ignore error
        let url_path = url_path.strip_prefix("/").unwrap_or(url_path);

        // TODO: add more error check
        path.push(url_path);

        Ok(path)
    }

    pub fn load_image(&self, uri: &str) -> Result<Image> {
        let path = self.load_path(uri)?;

        let image = Image::load_png(path)?;

        Ok(image)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use url::Url;

    #[test]
    fn test_url() {
        let url = Url::parse("file:///path/to/file.json").unwrap();
        println!("{:?}", url);
    }

    #[test]
    fn test_assets_manager() {
        let assets_dir = Path::new("./assets/");
        let project_dir = Path::new("./project");
        let manager = super::AssetsManager::new(assets_dir, project_dir);
        let path = manager
            .load_path("assets:///models/cmu_cow/spot.obj")
            .unwrap();
        println!("{:?}", path);
    }
}
