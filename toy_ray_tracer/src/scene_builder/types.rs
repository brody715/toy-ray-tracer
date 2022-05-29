use schemars::{
    schema::{ArrayValidation, InstanceType, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};

use crate::core::{Settings, Vec3f};

#[derive(Debug, Clone, Copy)]
pub struct JVec3f(pub Vec3f);

impl JVec3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3f::new(x, y, z))
    }
}

impl Serialize for JVec3f {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for JVec3f {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Deserialize::deserialize(deserializer)?))
    }
}

impl JsonSchema for JVec3f {
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
pub struct ProjectConfig {
    pub name: String,
    pub settings: Settings,
    pub scene: SceneConfig,
    #[serde(default)]
    pub accelerator: AcceleratorConfig,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AcceleratorConfig {
    Nop {},
    BVH {},
}

impl Default for AcceleratorConfig {
    fn default() -> Self {
        AcceleratorConfig::BVH {}
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct SceneConfig {
    pub camera: CameraConfig,
    pub world: Vec<PrimitiveConfig>,
    pub environments: Vec<EnvironmentConfig>,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct EnvironmentConfig {
    pub l: JVec3f,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(default)]
pub struct CameraConfig {
    pub look_from: JVec3f,
    pub look_at: JVec3f,
    pub view_up: JVec3f,
    pub vertical_fov: f32,
    pub aspect: f32,
    pub aperture: f32,
    pub focus_dist: f32,
    pub time0: f32,
    pub time1: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            look_from: JVec3f::new(0.0, 0.0, 0.0),
            look_at: JVec3f::new(0.0, 0.0, -1.0),
            view_up: JVec3f::new(0.0, 1.0, 0.0),
            vertical_fov: 90.0,
            aspect: 1.0,
            aperture: 0.0,
            focus_dist: 1.0,
            time0: 0.0,
            time1: 0.0,
        }
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PrimitiveConfig {
    // Geom leaf node
    Geom {
        #[serde(default)]
        transforms: Vec<TransformConfig>,
        shape: ShapeConfig,
        material: MaterialConfig,
        area_light: Option<AreaLightConfig>,
        #[serde(default)]
        flip_face: bool,
    },
    // Container brach node, contains either Container or Geom
    Container {
        #[serde(default)]
        transforms: Vec<TransformConfig>,

        children: Vec<PrimitiveConfig>,
    },
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct AreaLightConfig {}

impl PrimitiveConfig {
    pub fn get_transforms(&self) -> &[TransformConfig] {
        match self {
            PrimitiveConfig::Geom { transforms, .. } => transforms,
            PrimitiveConfig::Container { transforms, .. } => transforms,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TransformConfig {
    Translate {
        offset: JVec3f,
    },
    Rotate {
        axis: JVec3f,
        // in degree
        angle: f32,
    },
    Scale {
        scale: JVec3f,
    },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ShapeConfig {
    Sphere { center: JVec3f, radius: f32 },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(untagged)]
pub enum AorB<A, B> {
    A(A),
    B(B),
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MaterialConfig {
    Lambertian {
        albedo: AorB<TextureConfig, JVec3f>,
    },
    Metal {
        albedo: AorB<TextureConfig, JVec3f>,
        fuzz: f32,
    },
    Dielectric {
        ir: f32,
    },
    DiffuseLight {
        emit: AorB<TextureConfig, JVec3f>,
    },
    Isotropic {
        albedo: AorB<TextureConfig, JVec3f>,
    },
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TextureConfig {
    ConstantTexture { value: JVec3f },
    ImageTexture { file_path: String },
}
