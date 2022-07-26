use std::{rc::Rc, sync::Arc};

use crate::{
    accelerators::BVHAccel,
    core::{
        vec3, Camera, CameraOpt, MaterialPtr, PrimitiveContainerPtr, PrimitivePtr, Project, Scene,
        SceneBundle, Settings, ShapePtr, TexturePtr, Transform, Vec2f, Vec3f,
    },
    lights::{AreaLight, EnvironmentLight},
    materials::{Dielectric, DiffuseLight, GltfPbrMaterial, Lambertian, Metal, Transparent},
    primitives::{FlipFacePrimitive, GeometricPrimitive, PrimitiveList},
    shapes::{
        Cube, Cylinder, Disk, Pyramid, Rect, RegularPolygon, Sphere, Triangle, TriangleMeshStorage,
    },
    textures::{CheckerTexture, ConstantTexture, ImageTexture, ImageTextureParams},
};
use anyhow::{ensure, Context, Ok, Result};

use super::{
    loaders::{load_gltf_scenes, MeshLoader},
    types::{
        AcceleratorConfig, AorB, CameraConfig, JVec2f, JVec3f, MaterialConfig, PrimitiveConfig,
        ProjectConfig, SceneConfig, SceneCustomConfig, ShapeConfig, TextureConfig, TextureOrConst,
        TransformConfig, UriConfig,
    },
    AssetsManager,
};

impl Into<Vec3f> for JVec3f {
    fn into(self) -> Vec3f {
        Vec3f::new(self.0[0], self.0[1], self.0[2])
    }
}

impl Into<Vec3f> for &JVec3f {
    fn into(self) -> Vec3f {
        self.0.clone().into()
    }
}

impl Into<Vec2f> for JVec2f {
    fn into(self) -> Vec2f {
        Vec2f::new(self.0[0], self.0[1])
    }
}

pub struct Builder {
    settings: Option<Settings>,

    transforms_stack: Vec<Transform>,
    cur_transform: Transform,
    assets_manager: Rc<AssetsManager>,
}

impl Builder {
    pub fn new(assets_manager: Rc<AssetsManager>) -> Self {
        Self {
            settings: None,
            transforms_stack: Vec::new(),
            cur_transform: Transform::identity(),
            assets_manager,
        }
    }

    fn get_current_transform(&self) -> Transform {
        self.cur_transform.clone()
    }

    fn enter_transform(&mut self, transform: Transform) {
        self.transforms_stack.push(self.cur_transform.clone());
        // TODO: check orders
        self.cur_transform = transform * self.cur_transform.clone();
    }

    fn enter_transform_conf(&mut self, confs: &[TransformConfig]) -> Result<()> {
        let transform = self.build_transforms(confs)?;
        self.enter_transform(transform);
        Ok(())
    }

    fn exit_transform(&mut self) {
        self.cur_transform = self.transforms_stack.pop().unwrap();
    }

    fn get_settings(&self) -> &Settings {
        return self.settings.as_ref().unwrap();
    }
}

impl Builder {
    pub fn build_project(&mut self, conf: &ProjectConfig) -> Result<Project> {
        self.settings = Some(conf.settings.clone());

        let scene_bundle = self.build_scenes(&conf.scenes)?;
        let camera = scene_bundle.camera.clone().context("camera is not set")?;

        let world = self.build_accelerator(&conf.accelerator, &scene_bundle.primitives)?;

        let scene = Scene::new(camera, world, scene_bundle.lights);

        let project = Project::new(conf.name.clone(), conf.settings.clone(), scene);
        Ok(project)
    }

    fn build_accelerator(
        &self,
        conf: &AcceleratorConfig,
        prims: &[PrimitivePtr],
    ) -> Result<PrimitiveContainerPtr> {
        let primitive: PrimitiveContainerPtr = match conf {
            AcceleratorConfig::Nop {} => Arc::new(PrimitiveList::from(prims)),
            AcceleratorConfig::Bvh {} => Arc::new(BVHAccel::new(prims.to_vec(), 0.0, 1.0)),
        };
        Ok(primitive)
    }

    fn build_scenes(&mut self, confs: &[SceneConfig]) -> Result<SceneBundle> {
        let mut main_bundle = SceneBundle::default();

        for conf in confs {
            let bundle = match conf {
                SceneConfig::Uri { uri, transforms } => {
                    let gltf_path = self.assets_manager.load_path(uri)?;

                    let transform = self.build_transforms(&transforms)?;
                    let bundles = load_gltf_scenes(&gltf_path, transform)?;

                    let mut acc_bundle = SceneBundle::default();
                    for bundle in bundles {
                        acc_bundle.union_assign(bundle);
                    }
                    acc_bundle

                    // bundles.iter().reduce(|acc, elem| acc.uni)
                }
                SceneConfig::Custom(conf) => self.build_scene_custom(conf)?,
            };

            main_bundle.union_assign(bundle);
        }

        ensure!(main_bundle.lights.len() > 0);
        ensure!(main_bundle.camera.is_some());

        Ok(main_bundle)
    }

    fn build_scene_custom(&mut self, conf: &SceneCustomConfig) -> Result<SceneBundle> {
        self.enter_transform_conf(&conf.transforms)?;

        let mut bundle = self.build_world(&conf.world)?;

        if let Some(camera) = &conf.camera {
            let camera = self.build_camera(camera)?;
            bundle.camera = Some(camera);
        }

        for env in &conf.environments {
            let light = Arc::new(EnvironmentLight::new(env.l.into()));
            bundle.lights.push(light);
        }

        self.exit_transform();

        Ok(bundle)
    }

    fn build_camera(&self, conf: &CameraConfig) -> Result<Arc<Camera>> {
        let aspect = if let Some(aspect) = conf.aspect {
            aspect
        } else {
            self.get_settings().get_aspect()
        };

        Ok(Arc::new(Camera::new(CameraOpt {
            look_from: conf.look_from.into(),
            look_at: conf.look_at.into(),
            view_up: conf.view_up.into(),
            vertical_fov: conf.vertical_fov,
            aspect,
            aperture: conf.aperture,
            focus_dist: conf.focus_dist,
            time0: conf.time0,
            time1: conf.time1,
        })))
    }

    fn build_world(&mut self, confs: &[PrimitiveConfig]) -> Result<SceneBundle> {
        let mut bundle = SceneBundle::default();

        for conf in confs {
            self.enter_transform_conf(conf.get_transforms())?;

            match conf {
                PrimitiveConfig::Geom {
                    transforms: _,
                    shape,
                    material,
                    area_light,
                    flip_face,
                } => {
                    let shapes = self.build_shapes(shape, self.get_current_transform())?;
                    let material = self.build_material(material)?;

                    for shape in shapes {
                        let prim: PrimitivePtr =
                            Arc::new(GeometricPrimitive::new(shape, material.clone()));

                        let prim = if *flip_face {
                            Arc::new(FlipFacePrimitive::new(prim))
                        } else {
                            prim
                        };

                        if let Some(_area_light) = area_light {
                            let area_light = AreaLight::new(prim.clone());
                            bundle.lights.push(Arc::new(area_light));
                        }

                        bundle.primitives.push(prim);
                    }
                }
                PrimitiveConfig::Container {
                    transforms: _,
                    children,
                } => {
                    // build children
                    let children_bundle = self.build_world(&children)?;
                    bundle.union_assign(children_bundle);
                }
            }

            self.exit_transform();
        }

        Ok(bundle)
    }

    fn build_shapes(
        &self,
        conf: &ShapeConfig,
        object_to_world: Transform,
    ) -> Result<Vec<ShapePtr>> {
        let shapes: Vec<ShapePtr> = match conf {
            ShapeConfig::Sphere { center, radius } => {
                vec![Arc::new(Sphere::new(
                    center.into(),
                    *radius,
                    object_to_world,
                ))]
            }
            ShapeConfig::TriangleMesh {
                indices,
                positions,
                uvs,
                normals,
            } => {
                let positions: Vec<Vec3f> = positions.iter().map(|v| v.into()).collect();
                let normals: Vec<Vec3f> = normals.iter().map(|v| v.into()).collect();
                let uvs: Vec<Vec2f> = uvs.iter().map(|v| v.clone().into()).collect();
                let indices: Vec<usize> = indices.iter().map(|v| *v as usize).collect();
                let n_triangles = indices.len() / 3;

                let mesh = Arc::new(TriangleMeshStorage::try_new(
                    n_triangles,
                    indices,
                    positions,
                    normals,
                    uvs,
                    object_to_world,
                )?);

                let mut shapes: Vec<ShapePtr> = Vec::new();
                for i in 0..n_triangles {
                    let mesh_ptr = mesh.clone();
                    let shape = Arc::new(Triangle::new(i, mesh_ptr));
                    shapes.push(shape);
                }
                shapes
            }
            ShapeConfig::Uri(UriConfig { uri }) => {
                let mesh_loader = MeshLoader::new(self.assets_manager.clone());

                let mesh_bundle = mesh_loader.load(uri)?;
                let n_triangles = mesh_bundle.n_triangles;

                let mesh = Arc::new(TriangleMeshStorage::try_new(
                    mesh_bundle.n_triangles,
                    mesh_bundle.indices,
                    mesh_bundle.positions,
                    mesh_bundle.normals,
                    mesh_bundle.uvs,
                    object_to_world,
                )?);

                let mut shapes: Vec<ShapePtr> = Vec::new();
                for i in 0..n_triangles {
                    let mesh_ptr = mesh.clone();
                    let shape = Arc::new(Triangle::new(i, mesh_ptr));
                    shapes.push(shape);
                }
                shapes
            }
            ShapeConfig::Cube { p_min, p_max } => {
                vec![Arc::new(Cube::new(
                    p_min.into(),
                    p_max.into(),
                    object_to_world,
                ))]
            }
            ShapeConfig::Rect { v0, v1 } => {
                vec![Arc::new(Rect::new(v0.into(), v1.into(), object_to_world))]
            }
            ShapeConfig::Disk {
                center,
                radius,
                normal,
            } => {
                vec![Arc::new(Disk::new(
                    center.into(),
                    *radius,
                    normal.into(),
                    object_to_world,
                ))]
            }
            ShapeConfig::Cylinder {
                center0,
                center1,
                radius,
            } => {
                vec![Arc::new(Cylinder::new(
                    center0.into(),
                    center1.into(),
                    *radius,
                    object_to_world,
                ))]
            }
            ShapeConfig::Pyramid { v0, v1, v2, v3 } => {
                vec![Arc::new(Pyramid::new(
                    [v0.into(), v1.into(), v2.into(), v3.into()],
                    object_to_world,
                ))]
            }
            ShapeConfig::RegularPolygon {
                radius,
                num_sides,
                normal,
            } => {
                vec![Arc::new(RegularPolygon::new(
                    *radius,
                    *num_sides,
                    normal.into(),
                    object_to_world,
                ))]
            }
        };

        Ok(shapes)
    }

    fn build_material(&self, conf: &MaterialConfig) -> Result<MaterialPtr> {
        let material: MaterialPtr = match conf {
            MaterialConfig::Lambertian { albedo } => {
                let albedo = self.build_texture_or_vec3f(&albedo)?;
                Arc::new(Lambertian::new(albedo))
            }
            MaterialConfig::Metal { albedo, fuzz } => {
                Arc::new(Metal::new(self.build_texture_or_vec3f(&albedo)?, *fuzz))
            }
            MaterialConfig::Dielectric { ir } => Arc::new(Dielectric::new(*ir)),
            MaterialConfig::DiffuseLight { emit } => {
                Arc::new(DiffuseLight::new(self.build_texture_or_vec3f(&emit)?))
            }
            MaterialConfig::Transparent {
                eta,
                roughness,
                albedo,
            } => Arc::new(Transparent::new(
                *eta,
                self.build_texture_or_f32(&roughness)?,
                self.build_texture_or_vec3f(&albedo)?,
            )),
            MaterialConfig::GltfPbr {
                eta,
                base_color,
                roughness,
                metallic,
                emit,
            } => Arc::new(GltfPbrMaterial::new(
                *eta,
                self.build_texture_or_vec3f(base_color)?,
                self.build_texture_or_f32(metallic)?,
                self.build_texture_or_f32(roughness)?,
                self.build_texture_or_vec3f(emit)?,
            )),
        };

        Ok(material)
    }

    fn build_texture_or_vec3f(&self, conf: &TextureOrConst<JVec3f>) -> Result<TexturePtr<Vec3f>> {
        let texture: TexturePtr<Vec3f> = match conf.clone() {
            AorB::A(conf) => self.build_texture_vec3f(&conf)?,
            AorB::B(value) => Arc::new(ConstantTexture::new(value.into())),
        };

        Ok(texture)
    }

    fn build_texture_vec3f(&self, conf: &TextureConfig<JVec3f>) -> Result<TexturePtr<Vec3f>> {
        Ok(match conf {
            TextureConfig::ConstantTexture { value } => {
                Arc::new(ConstantTexture::new(value.into()))
            }
            TextureConfig::ImageTexture { uri } => {
                let image = self.assets_manager.load_image(&uri)?;
                Arc::new(ImageTexture::from_image(
                    image,
                    ImageTextureParams {
                        scale: vec3::scalar(1.0),
                        flip: true,
                    },
                ))
            }
            TextureConfig::CheckerTexture { odd, even } => {
                let odd = self.build_texture_or_vec3f(odd.as_ref())?;
                let even = self.build_texture_or_vec3f(even.as_ref())?;

                Arc::new(CheckerTexture::new(odd, even))
            }
        })
    }

    fn build_texture_or_f32(&self, conf: &TextureOrConst<f32>) -> Result<TexturePtr<f32>> {
        let texture: TexturePtr<f32> = match conf {
            AorB::A(conf) => self.build_texture_f32(conf)?,
            AorB::B(value) => Arc::new(ConstantTexture::new(*value)),
        };

        Ok(texture)
    }

    fn build_texture_f32(&self, conf: &TextureConfig<f32>) -> Result<TexturePtr<f32>> {
        Ok(match conf {
            TextureConfig::ConstantTexture { value } => Arc::new(ConstantTexture::new(*value)),
            TextureConfig::ImageTexture { uri } => {
                let _image = self.assets_manager.load_image(&uri)?;
                todo!()
            }
            TextureConfig::CheckerTexture { odd, even } => {
                let odd = self.build_texture_or_f32(odd.as_ref())?;
                let even = self.build_texture_or_f32(even.as_ref())?;
                Arc::new(CheckerTexture::new(odd, even))
            }
        })
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
