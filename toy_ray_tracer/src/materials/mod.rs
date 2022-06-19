mod diffuse_light;
mod gltfpbr;
mod naive;
mod transparent;

pub use diffuse_light::DiffuseLight;
pub use gltfpbr::GltfPbrMaterial;
pub use naive::{Lambertian, Metal};
pub use transparent::Transparent;

// utiltiy functions

// clamp_roughness to make roughness not too small
pub fn clamp_roughness(roughness: f32) -> f32 {
    const MIN_ROUGHNESS: f32 = 0.009;
    roughness.clamp(MIN_ROUGHNESS, 1.0)
}
