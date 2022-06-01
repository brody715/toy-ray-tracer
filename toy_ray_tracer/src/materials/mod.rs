mod diffuse_light;
mod gltfpbr;
mod naive;
mod transparent;

pub use diffuse_light::DiffuseLight;
pub use gltfpbr::GltfPbrMaterial;
pub use naive::{Lambertian, Metal};
pub use transparent::Transparent;
