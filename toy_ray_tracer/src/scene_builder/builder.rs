use std::{rc::Rc, sync::Arc};

use crate::{
    accelerators::BVHAccel,
    core::{
        Camera, CameraOpt, LightPtr, MaterialPtr, PrimitivePtr, Project, Scene, ShapePtr,
        TexturePtr, Transform, Vec2f, Vec3f,
    },
    lights::{AreaLight, EnvironmentLight},
    materials::{Dielectric, DiffuseLight, Isotropic, Lambertian, Metal},
    primitives::{FlipFacePrimitive, GeometricPrimitive, PrimitiveList},
    shapes::{Cube, Cylinder, Disk, Pyramid, Rect, Sphere, Triangle, TriangleMeshStorage},
    textures::{CheckerTexture, ConstantTexture, ImageTexture},
};
use anyhow::{ensure, Context, Ok, Result};

use super::{
    loaders::MeshLoader,
    types::{
        AcceleratorConfig, AorB, CameraConfig, JVec2f, JVec3f, MaterialConfig, PrimitiveConfig,
        ProjectConfig, SceneConfig, ShapeConfig, TextureConfig, TransformConfig, UriConfig,
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
    assets_manager: Rc<AssetsManager>,
}

impl Builder {
    pub fn new(assets_manager: Rc<AssetsManager>) -> Self {
        Self {
            transforms_stack: Vec::new(),
            cur_transform: Transform::identity(),
            assets_manager,
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
            AcceleratorConfig::Bvh {} => Arc::new(BVHAccel::new(prims.to_vec(), 0.0, 1.0)),
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
                    transforms: _,
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
                    self.enter_transform(transform);

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
        let shapes: Vec<ShapePtr> = match conf {
            ShapeConfig::Sphere { center, radius } => {
                vec![Arc::new(Sphere::new(center.into(), *radius))]
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
                vec![Arc::new(Cube::new(p_min.into(), p_max.into()))]
            }
            ShapeConfig::Rect { v0, v1 } => {
                vec![Arc::new(Rect::new(v0.into(), v1.into()))]
            }
            ShapeConfig::Disk {
                center,
                radius,
                normal,
            } => {
                vec![Arc::new(Disk::new(center.into(), *radius, normal.into()))]
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
                ))]
            }
            ShapeConfig::Pyramid { v0, v1, v2, v3 } => {
                vec![Arc::new(Pyramid::new([
                    v0.into(),
                    v1.into(),
                    v2.into(),
                    v3.into(),
                ]))]
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
            MaterialConfig::Isotropic { albedo } => {
                Arc::new(Isotropic::new(self.build_texture_or_vec3f(&albedo)?))
            }
        };

        Ok(material)
    }

    fn build_texture_or_vec3f(&self, conf: &AorB<TextureConfig, JVec3f>) -> Result<TexturePtr> {
        let texture: TexturePtr = match conf {
            AorB::A(conf) => self.build_texture(conf)?,
            AorB::B(value) => Arc::new(ConstantTexture::new(value.into())),
        };

        Ok(texture)
    }

    fn build_texture(&self, conf: &TextureConfig) -> Result<TexturePtr> {
        Ok(match conf {
            TextureConfig::ConstantTexture { value } => {
                Arc::new(ConstantTexture::new(value.into()))
            }
            TextureConfig::ImageTexture { uri } => {
                let image = self.assets_manager.load_image(uri)?;
                Arc::new(ImageTexture::new(image))
            }
            TextureConfig::CheckerTexture { odd, even } => {
                let odd = self.build_texture(odd.as_ref())?;
                let even = self.build_texture(even.as_ref())?;

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
