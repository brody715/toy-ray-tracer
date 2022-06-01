mod aabb;
mod camera;
mod interaction;
pub(crate) mod light;
mod material;
mod nimage;
mod primitive;
mod project;
mod ray;
pub mod reflection;
mod scene;
mod shape;
mod spectrum;
mod texture;
mod transform;
mod vec;

pub use aabb::AABB;
pub use camera::{Camera, CameraOpt};
pub use interaction::SurfaceInteraction;
pub use light::{Light, LightPtr, LightType};
pub use material::{Material, MaterialPtr};
pub use nimage::Image;
pub use primitive::{
    Primitive, PrimitiveContainer, PrimitiveContainerPtr, PrimitivePtr, PrimitiveRef,
};
pub use ray::Ray;
pub use reflection::{Bsdf, Bxdf, BxdfPtr};
pub use shape::{Shape, ShapePtr};
pub use spectrum::Spectrum;
pub use texture::{Texture, TextureData, TexturePtr};
pub use transform::Transform;
pub use vec::{vec3, Color3, Point2f, Point3f, Vec2f, Vec3List, Vec3f, Vec4f};

pub use project::{Project, Settings};
pub use scene::Scene;
