use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::Settings;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct JVec3f(pub [f32; 3]);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct JVec2f(pub [f32; 2]);

impl JVec3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }
}

impl JVec2f {
    #[allow(dead_code)]
    fn new(x: f32, y: f32) -> Self {
        Self([x, y])
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub settings: Settings,
    pub scenes: Vec<SceneConfig>,
    #[serde(default)]
    pub accelerator: AcceleratorConfig,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AcceleratorConfig {
    Nop {},
    Bvh {},
}

impl Default for AcceleratorConfig {
    fn default() -> Self {
        AcceleratorConfig::Bvh {}
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SceneConfig {
    Uri {
        #[serde(default)]
        transforms: Vec<TransformConfig>,
        uri: String,
    },
    Custom(SceneCustomConfig),
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct SceneCustomConfig {
    #[serde(default)]
    pub transforms: Vec<TransformConfig>,
    pub camera: Option<CameraConfig>,
    #[serde(default)]
    pub world: Vec<PrimitiveConfig>,
    #[serde(default)]
    pub environments: Vec<EnvironmentConfig>,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct SceneExtendItemConfig {}

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
    pub aspect: Option<f32>,
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
            aspect: None,
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

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct UriConfig {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ShapeConfig {
    Sphere {
        center: JVec3f,
        radius: f32,
    },
    Cube {
        p_min: JVec3f,
        p_max: JVec3f,
    },
    Rect {
        v0: JVec3f,
        v1: JVec3f,
    },
    Disk {
        center: JVec3f,
        radius: f32,
        normal: JVec3f,
    },
    Cylinder {
        center0: JVec3f,
        center1: JVec3f,
        radius: f32,
    },
    Pyramid {
        v0: JVec3f,
        v1: JVec3f,
        v2: JVec3f,
        v3: JVec3f,
    },
    TriangleMesh {
        indices: Vec<usize>,
        positions: Vec<JVec3f>,
        #[serde(default)]
        uvs: Vec<JVec2f>,
        #[serde(default)]
        normals: Vec<JVec3f>,
    },
    Uri(UriConfig),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(untagged)]
pub enum AorB<A, B> {
    A(A),
    B(B),
}

pub type TextureOrConst<T> = AorB<TextureConfig<T>, T>;

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MaterialConfig {
    Lambertian {
        albedo: TextureOrConst<JVec3f>,
    },
    Metal {
        albedo: TextureOrConst<JVec3f>,
        fuzz: f32,
    },
    Dielectric {
        ir: f32,
    },
    DiffuseLight {
        emit: TextureOrConst<JVec3f>,
    },
    Transparent {
        eta: f32,
        roughness: TextureOrConst<f32>,
        albedo: TextureOrConst<JVec3f>,
    },
    GltfPbr {
        #[serde(default = "default_gltf_pbr_eta")]
        eta: f32,
        base_color: TextureOrConst<JVec3f>,
        roughness: TextureOrConst<f32>,
        metallic: TextureOrConst<f32>,
        #[serde(default = "default_texture_vec3f")]
        emit: TextureOrConst<JVec3f>,
    },
}

fn default_gltf_pbr_eta() -> f32 {
    1.5
}

fn default_texture_vec3f() -> TextureOrConst<JVec3f> {
    return TextureOrConst::B(JVec3f::new(0.0, 0.0, 0.0));
}

#[derive(JsonSchema, Deserialize, Serialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TextureConfig<T> {
    ConstantTexture {
        value: T,
    },
    ImageTexture {
        uri: String,
    },
    CheckerTexture {
        odd: Box<TextureOrConst<T>>,
        even: Box<TextureOrConst<T>>,
    },
}
