mod aabb;
mod camera;
mod hittable_list;
mod material;
mod nimage;
mod primitive;
mod project;
mod ray;
mod scene;
mod shape;
mod texture;
mod vec;
mod reflection;
mod spectrum;

pub use spectrum::Spectrum;
pub use aabb::AABB;
pub use camera::{Camera, CameraOpt};
pub use hittable_list::{HittableList, HittableListPtr};
pub use material::{Material, MaterialPtr, ScatterRecord};
pub use nimage::Image;
pub use primitive::{HitRecord, Primitive, PrimitivePtr, PrimitiveRef};
pub use ray::Ray;
pub use shape::{Shape, ShapePtr};
pub use texture::{Texture, TexturePtr};
pub use vec::{vec3, Color3, Point3f, Point2f, Vec2f, Vec3List, Vec3f, Vec4f};

pub use project::{Project, Settings};
pub use scene::Scene;
