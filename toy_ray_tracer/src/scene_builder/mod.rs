mod js;
mod load;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use anyhow::Context;
use log::info;
use schemars::schema::{ArrayValidation, InstanceType, SchemaObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::HittableList;
use crate::core::MaterialPtr;
use crate::core::TexturePtr;
use crate::core::Vec3;
use crate::core::{self, Camera, CameraOpt};
use crate::core::{Hittable, HittablePtr};
use crate::core::{Project, Scene, Settings};
use crate::environment::{SkyPtr, SolidSky};
use crate::geometry::containers::{TagsHittable, BVH};
use crate::geometry::shapes::{
    Cube, Cylinder, Disk, Mesh, MeshLoadOptions, MovingSphere, Pyramid, Rect, SkyLight, Sphere,
    Triangle,
};
use crate::geometry::transforms::{Axis, FlipFace, Rotate, Transform, Transformed, Translate};
use crate::geometry::visitors::try_get_light_from_world;
use crate::geometry::volumes::ConstantMedium;
use crate::materials::{Dielectric, DiffuseLight, Isotropic, Lambertian, Metal};
use crate::math::SamplerType;
use crate::textures::{CheckerTexture, ConstantTexture, ImageTexture};

pub use load::load_project_config;

#[derive(Debug)]
pub struct JVec3(Vec3);

impl Serialize for JVec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for JVec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Deserialize::deserialize(deserializer)?))
    }
}

impl Into<Vec3> for JVec3 {
    fn into(self) -> Vec3 {
        self.0
    }
}

impl JsonSchema for JVec3 {
    fn schema_name() -> String {
        format!("Vec3f")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<f32>().into()),
                min_items: Some(3),
                max_items: Some(3),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TransformParam {
    Rotate { axis: JVec3, angle: f32 },
    Translate { offset: JVec3 },
    Scale { scale: JVec3 },
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct FromFileFragment {
    path: String,
}

pub trait Buildable {
    type Out;
    fn build(self) -> Self::Out;
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub settings: Settings,
    pub scene: SceneConfig,
}

impl Buildable for ProjectConfig {
    type Out = Project;

    fn build(self) -> Project {
        Project {
            name: self.name,
            settings: self.settings,
            scene: self.scene.build(),
        }
    }
}

#[derive(JsonSchema, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Properties {
    sampler: SamplerType,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WorldConfig {
    Objects(Vec<GeometryConfig>),
    Custom(GeometryConfig),
}

impl Buildable for WorldConfig {
    type Out = HittablePtr;

    fn build(self) -> Self::Out {
        match self {
            WorldConfig::Objects(items) => Arc::new(BVH::new(items.build(), 0.0, 0.0)),
            WorldConfig::Custom(item) => item.build(),
        }
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct SceneConfig {
    pub camera: CameraConfig,
    pub sky: SkyConfig,
    pub world: WorldConfig,
    pub light: Option<GeometryConfig>,
}

impl Buildable for SceneConfig {
    type Out = Scene;

    fn build(self) -> Self::Out {
        let sky = self.sky.build();

        let world = self.world.build();
        let light_shape = match self.light {
            Some(lights) => lights.build(),
            None => {
                let try_lights = try_get_light_from_world(world.as_ref());

                match try_lights {
                    Some(lights) => {
                        info!("found lights");
                        lights
                    }
                    None => Arc::new(SkyLight {}),
                }
            }
        };

        Scene {
            camera: self.camera.build(),
            world,
            light_shape,
            sky,
            name: String::from("no-name"),
            description: String::from(""),
        }
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct CameraConfig {
    pub look_from: JVec3,
    pub look_at: JVec3,
    pub view_up: JVec3,
    pub vertical_fov: f32,
    pub aspect: f32,
    pub aperture: f32,
    pub focus_dist: f32,
    pub time0: f32,
    pub time1: f32,
}

impl Buildable for CameraConfig {
    type Out = Camera;

    fn build(self) -> Self::Out {
        Camera::new(CameraOpt {
            look_from: self.look_from.into(),
            look_at: self.look_at.into(),
            view_up: self.view_up.into(),
            vertical_fov: self.vertical_fov,
            aspect: self.aspect,
            aperture: self.aperture,
            focus_dist: self.focus_dist,
            time0: self.time0,
            time1: self.time1,
        })
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SkyConfig {
    Solid { background: JVec3 },
}

impl Buildable for SkyConfig {
    type Out = SkyPtr;

    fn build(self) -> Self::Out {
        match self {
            SkyConfig::Solid { background } => Arc::new(SolidSky {
                background: background.into(),
            }),
        }
    }
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MaterialConfig {
    Lambertian { albedo: TextureConfig },
    Metal { albedo: TextureConfig, fuzz: f32 },
    Dielectric { ir: f32 },
    DiffuseLight { emit: TextureConfig },
    Isotropic { albedo: TextureConfig },
}

impl Buildable for MaterialConfig {
    type Out = MaterialPtr;

    fn build(self) -> Self::Out {
        match self {
            MaterialConfig::Lambertian { albedo } => Arc::new(Lambertian::new(albedo.build())),
            MaterialConfig::Metal { albedo, fuzz } => Arc::new(Metal::new(albedo.build(), fuzz)),
            MaterialConfig::Dielectric { ir } => Arc::new(Dielectric::new(ir)),
            MaterialConfig::DiffuseLight { emit } => Arc::new(DiffuseLight::new(emit.build())),
            MaterialConfig::Isotropic { albedo } => Arc::new(Isotropic::new(albedo.build())),
        }
    }
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TextureConfig {
    ConstantTexture {
        color: JVec3,
    },
    ImageTexture {
        file_path: String,
    },
    CheckerTexture {
        odd: Box<TextureConfig>,
        even: Box<TextureConfig>,
    },
    NopTexture(NopTextureConfig),
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct NopTextureConfig {
    name: String,
}

impl Default for NopTextureConfig {
    fn default() -> Self {
        NopTextureConfig {
            name: String::from("nop"),
        }
    }
}

impl Buildable for TextureConfig {
    type Out = TexturePtr;

    fn build(self) -> Self::Out {
        match self {
            TextureConfig::ConstantTexture { color } => {
                Arc::new(ConstantTexture::new(color.into()))
            }
            TextureConfig::ImageTexture { file_path } => {
                let img = core::Image::load_png(file_path.clone())
                    .expect(format!("failed to load image texture: {}", &file_path).as_str());
                Arc::new(ImageTexture::new(img))
            }
            TextureConfig::CheckerTexture { odd, even } => {
                let odd = odd.build();
                let even = even.build();
                Arc::new(CheckerTexture::new(odd, even))
            }
            TextureConfig::NopTexture(_) => todo!(),
        }
    }
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MeshObjectConfig {
    FilePath(String),
    RawString(String),
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GeometryConfig {
    Sphere {
        center: JVec3,
        radius: f32,
        material: MaterialConfig,
    },
    MovingSphere {
        center0: JVec3,
        center1: JVec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: MaterialConfig,
    },
    Cube {
        p_min: JVec3,
        p_max: JVec3,
        material: MaterialConfig,
    },
    Rect {
        v0: JVec3,
        v1: JVec3,
        material: MaterialConfig,
    },
    Triangle {
        v0: JVec3,
        v1: JVec3,
        v2: JVec3,
        material: MaterialConfig,
    },
    Disk {
        properties: Option<Properties>,
        center: JVec3,
        radius: f32,
        normal: JVec3,
        material: MaterialConfig,
    },
    Cylinder {
        center0: JVec3,
        center1: JVec3,
        radius: f32,
        material: MaterialConfig,
    },
    Pyramid {
        v0: JVec3,
        v1: JVec3,
        v2: JVec3,
        v3: JVec3,
        material: MaterialConfig,
    },
    #[serde(rename = "mesh")]
    Mesh {
        from_obj: MeshObjectConfig,
        load_options: Option<MeshLoadOptions>,
        material: MaterialConfig,
    },
    #[serde(rename = "bvh")]
    BVH {
        children: Vec<GeometryConfig>,
        time0: f32,
        time1: f32,
    },
    Tags {
        tags: Vec<String>,
        child: Box<GeometryConfig>,
        properties: Option<HashMap<String, serde_json::Value>>,
    },
    List {
        children: Vec<GeometryConfig>,
    },
    Rotate {
        axis: Axis,
        angle: f32,
        child: Box<GeometryConfig>,
    },
    Transforms {
        params: Vec<TransformParam>,
        child: Box<GeometryConfig>,
    },
    Translate {
        offset: JVec3,
        child: Box<GeometryConfig>,
    },
    FlipFace {
        child: Box<GeometryConfig>,
    },
    ConstantMedium {
        boundary: Box<GeometryConfig>,
        density: f32,
        texture: TextureConfig,
    },
}

impl<T: Buildable> Buildable for Vec<T> {
    type Out = Vec<T::Out>;

    fn build(self) -> Self::Out {
        self.into_iter().map(|o| o.build()).collect()
    }
}

impl Buildable for GeometryConfig {
    type Out = HittablePtr;

    fn build(self) -> Self::Out {
        match self {
            GeometryConfig::Sphere {
                center,
                radius,
                material,
            } => Arc::new(Sphere::new(center.into(), radius, material.build())),
            GeometryConfig::Cube {
                p_min,
                p_max,
                material,
            } => Arc::new(Cube::new(p_min.into(), p_max.into(), material.build())),
            GeometryConfig::Triangle {
                v0,
                v1,
                v2,
                material,
            } => Arc::new(Triangle::new(
                [v0.into(), v1.into(), v2.into()],
                None,
                material.build(),
            )),
            GeometryConfig::Disk {
                properties,
                center,
                radius,
                normal,
                material,
            } => {
                let mut disk = Disk::new(center.into(), radius, normal.into(), material.build());
                if let Some(properties) = properties {
                    disk.set_sampler(properties.sampler);
                }
                Arc::new(disk)
            }
            GeometryConfig::BVH {
                children,
                time0,
                time1,
            } => Arc::new(BVH::new(children.build(), time0, time1)),
            GeometryConfig::List { children } => Arc::new(HittableList::from(children.build())),
            GeometryConfig::MovingSphere {
                center0,
                center1,
                time0,
                time1,
                radius,
                material,
            } => Arc::new(MovingSphere::new(
                center0.into(),
                center1.into(),
                time0,
                time1,
                radius,
                material.build(),
            )),
            GeometryConfig::Rotate { axis, angle, child } => {
                Arc::new(Rotate::new(axis, child.build(), angle))
            }
            GeometryConfig::Translate { offset, child } => {
                Arc::new(Translate::new(child.build(), offset.into()))
            }
            GeometryConfig::ConstantMedium {
                boundary,
                density,
                texture,
            } => Arc::new(ConstantMedium::new(
                boundary.build(),
                density,
                texture.build(),
            )),
            GeometryConfig::Rect { v0, v1, material } => {
                Arc::new(Rect::new(v0.into(), v1.into(), material.build()))
            }
            GeometryConfig::FlipFace { child } => Arc::new(FlipFace::new(child.build())),
            GeometryConfig::Tags {
                tags,
                child,
                properties,
            } => {
                let tags: HashSet<String> = tags.iter().map(|t| String::from(t)).collect();
                Arc::new(TagsHittable::new(tags, child.build(), properties))
            }
            GeometryConfig::Cylinder {
                center0,
                center1,
                radius,
                material,
            } => Arc::new(Cylinder::new(
                center0.into(),
                center1.into(),
                radius,
                material.build(),
            )),
            GeometryConfig::Transforms { params, child } => {
                let mut transform = Transform::identity();
                for param in params {
                    match param {
                        TransformParam::Rotate { axis, angle } => {
                            transform = Transform::rotate(axis.into(), angle) * transform;
                        }
                        TransformParam::Translate { offset } => {
                            transform = Transform::translate(offset.into()) * transform;
                        }
                        TransformParam::Scale { scale } => {
                            transform = Transform::scale(scale.into()) * transform;
                        }
                    }
                }
                Arc::new(Transformed::new(child.build(), transform))
            }
            GeometryConfig::Mesh {
                from_obj,
                material,
                load_options: load_option,
            } => {
                let material = material.build();
                let load_options = match load_option {
                    Some(o) => o,
                    None => MeshLoadOptions::default(),
                };
                let mesh = match from_obj {
                    MeshObjectConfig::FilePath(path) => {
                        Mesh::try_from_obj_file(&path, material, load_options)
                            .context(format!("load mesh file failed: {}", &path))
                    }
                    MeshObjectConfig::RawString(content) => {
                        Mesh::try_from_obj_str(&content, material, load_options)
                    }
                };

                let mesh = mesh.expect("failed to create mesh");
                Arc::new(mesh)
            }
            GeometryConfig::Pyramid {
                v0,
                v1,
                v2,
                v3,
                material,
            } => Arc::new(Pyramid::new(
                [v0.into(), v1.into(), v2.into(), v3.into()],
                material.build(),
            )),
        }
    }
}

// impl From<WorldConfig> for World {
//     fn from(p: WorldConfig) -> Self {
//         let objects = Arc::new(BVH::new(p.objects, 0.0, 1.0));
//         Self {
//             sky: p.sky,
//             objects,
//         }
//     }
// }
