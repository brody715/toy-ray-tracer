use std::sync::Arc;

use crate::{
    accelerators::BVHAccel,
    core::{
        Camera, CameraOpt, LightPtr, MaterialPtr, PrimitivePtr, Project, Scene, ShapePtr,
        TexturePtr, Transform, Vec3f,
    },
    lights::{AreaLight, EnvironmentLight},
    materials::{Dielectric, DiffuseLight, Isotropic, Lambertian, Metal},
    primitives::{FlipFacePrimitive, GeometricPrimitive, PrimitiveList},
    shapes::Sphere,
    textures::ConstantTexture,
};
use anyhow::{ensure, Context, Ok, Result};

use super::types::{
    AcceleratorConfig, AorB, CameraConfig, JVec3f, MaterialConfig, PrimitiveConfig, ProjectConfig,
    SceneConfig, ShapeConfig, TextureConfig, TransformConfig,
};

impl Into<Vec3f> for JVec3f {
    fn into(self) -> Vec3f {
        self.0
    }
}

impl Into<Vec3f> for &JVec3f {
    fn into(self) -> Vec3f {
        self.0
    }
}

struct SceneBundle {
    primitives: Vec<PrimitivePtr>,
    lights: Vec<LightPtr>,
    camera: Option<Arc<Camera>>,
}

impl Default for SceneBundle {
    fn default() -> Self {
        Self {
            primitives: Vec::new(),
            lights: Vec::new(),
            camera: Default::default(),
        }
    }
}

impl SceneBundle {
    pub fn union_assign(&mut self, mut other: SceneBundle) {
        self.primitives.append(&mut other.primitives);
        if let Some(camera) = other.camera {
            self.camera = Some(camera);
        }
    }
}

pub struct Builder {
    transforms_stack: Vec<Transform>,
    cur_transform: Transform,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            transforms_stack: Vec::new(),
            cur_transform: Transform::identity(),
        }
    }

    fn enter_transform(&mut self, transform: Transform) {
        self.transforms_stack.push(self.cur_transform.clone());
        // TODO: check orders
        self.cur_transform = transform * self.cur_transform.clone();
    }

    fn exit_transform(&mut self) {
        self.cur_transform = self.transforms_stack.pop().unwrap();
    }
}

impl Builder {
    pub fn build_project(&mut self, conf: &ProjectConfig) -> Result<Project> {
        let scene_bundle = self.build_scene(&conf.scene)?;
        let camera = scene_bundle.camera.clone().context("camera is not set")?;

        let world = self.build_accelerator(&conf.accelerator, &scene_bundle.primitives)?;

        let aggregate = self.build_accelerator(&conf.accelerator, &scene_bundle.primitives);

        let scene = Scene::new(camera, world, scene_bundle.lights);

        let project = Project::new(conf.name.clone(), conf.settings.clone(), scene);
        Ok(project)
    }

    fn build_accelerator(
        &self,
        conf: &AcceleratorConfig,
        prims: &[PrimitivePtr],
    ) -> Result<PrimitivePtr> {
        let primitive: PrimitivePtr = match conf {
            AcceleratorConfig::Nop {} => Arc::new(PrimitiveList::from(prims)),
            AcceleratorConfig::BVH {} => Arc::new(BVHAccel::new(prims.to_vec(), 0.0, 1.0)),
        };
        Ok(primitive)
    }

    fn build_scene(&mut self, conf: &SceneConfig) -> Result<SceneBundle> {
        let mut derived_bundle = self.build_primitives(&conf.world)?;

        derived_bundle.camera = Some(self.build_camera(&conf.camera)?);

        for env in &conf.environments {
            let light = Arc::new(EnvironmentLight::new(env.l.into()));
            derived_bundle.lights.push(light);
        }

        ensure!(derived_bundle.lights.len() > 0);

        Ok(derived_bundle)
    }

    fn build_camera(&self, conf: &CameraConfig) -> Result<Arc<Camera>> {
        Ok(Arc::new(Camera::new(CameraOpt {
            look_from: conf.look_from.into(),
            look_at: conf.look_at.into(),
            view_up: conf.view_up.into(),
            vertical_fov: conf.vertical_fov,
            aspect: conf.aspect,
            aperture: conf.aperture,
            focus_dist: conf.focus_dist,
            time0: conf.time0,
            time1: conf.time1,
        })))
    }

    fn build_primitives(&mut self, confs: &[PrimitiveConfig]) -> Result<SceneBundle> {
        let mut bundle = SceneBundle::default();

        for conf in confs {
            let transforms = conf.get_transforms();
            let transform = self.build_transforms(transforms)?;

            self.enter_transform(transform.clone());

            match conf {
                PrimitiveConfig::Geom {
                    transforms,
                    shape,
                    material,
                    area_light,
                    flip_face,
                } => {
                    let shapes = self.build_shapes(shape)?;
                    let material = self.build_material(material)?;

                    for shape in shapes {
                        let prim: PrimitivePtr = Arc::new(GeometricPrimitive::new(
                            shape,
                            transform.clone(),
                            material.clone(),
                            None,
                        ));

                        let prim = if *flip_face {
                            Arc::new(FlipFacePrimitive::new(prim))
                        } else {
                            prim
                        };

                        if let Some(area_light) = area_light {
                            let area_light = AreaLight::new(prim.clone());
                            bundle.lights.push(Arc::new(area_light));
                        }

                        bundle.primitives.push(prim);
                    }
                }
                PrimitiveConfig::Container {
                    transforms,
                    children,
                } => {
                    // build children
                    let children_bundle = self.build_primitives(&children)?;
                    bundle.union_assign(children_bundle);
                }
            }

            self.exit_transform();
        }

        Ok(bundle)
    }

    fn build_shapes(&self, conf: &ShapeConfig) -> Result<Vec<ShapePtr>> {
        let shapes: Vec<ShapePtr> = match *conf {
            ShapeConfig::Sphere { center, radius } => {
                vec![Arc::new(Sphere::new(center.into(), radius))]
            }
        };

        Ok(shapes)
    }

    fn build_material(&self, conf: &MaterialConfig) -> Result<MaterialPtr> {
        let material: MaterialPtr = match conf {
            MaterialConfig::Lambertian { albedo } => {
                let albedo = self.build_texture(&albedo)?;
                Arc::new(Lambertian::new(albedo))
            }
            MaterialConfig::Metal { albedo, fuzz } => {
                Arc::new(Metal::new(self.build_texture(&albedo)?, *fuzz))
            }
            MaterialConfig::Dielectric { ir } => Arc::new(Dielectric::new(*ir)),
            MaterialConfig::DiffuseLight { emit } => {
                Arc::new(DiffuseLight::new(self.build_texture(&emit)?))
            }
            MaterialConfig::Isotropic { albedo } => {
                Arc::new(Isotropic::new(self.build_texture(&albedo)?))
            }
        };

        Ok(material)
    }

    fn build_texture(&self, conf: &AorB<TextureConfig, JVec3f>) -> Result<TexturePtr> {
        let texture: TexturePtr = match conf {
            AorB::A(conf) => match conf {
                TextureConfig::ConstantTexture { value } => {
                    Arc::new(ConstantTexture::new(value.into()))
                }
                TextureConfig::ImageTexture { file_path } => todo!(),
            },
            AorB::B(value) => Arc::new(ConstantTexture::new(value.into())),
        };

        Ok(texture)
    }

    fn build_transforms(&self, confs: &[TransformConfig]) -> Result<Transform> {
        let transforms = confs.iter().map(|conf| match *conf {
            TransformConfig::Translate { offset } => Transform::translate(offset.into()),
            TransformConfig::Rotate { axis, angle } => Transform::rotate(axis.into(), angle),
            TransformConfig::Scale { scale } => Transform::scale(scale.into()),
        });

        let transform = transforms.fold(Transform::identity(), |acc, x| x * acc);

        Ok(transform)
    }
}
