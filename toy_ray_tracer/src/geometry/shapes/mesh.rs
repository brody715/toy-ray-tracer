use core::fmt;
use std::{io::BufReader, path::Path, sync::Arc};

use anyhow::{anyhow, Context, Ok};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    geometry::{shapes::Triangle, EnterContext},
    hittable::{Hittable, HittablePtr},
    hittable_list::HittableList,
    material::MaterialPtr,
    vec::{Vec2, Vec3},
};

// TODO: Optmize mesh to use shared data
pub struct Mesh {
    items: HittablePtr,
}

impl Mesh {
    pub fn new(faces: Vec<Arc<Triangle>>) -> Self {
        let items: Vec<HittablePtr> = faces
            .clone()
            .into_iter()
            .map(|v| v as HittablePtr)
            .collect();
        let items = Arc::new(HittableList::from(items));

        Self { items }
    }
}

impl Hittable for Mesh {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        self.items.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<crate::aabb::AABB> {
        self.items.bounding_box(t0, t1)
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_mesh(self)
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_mesh(EnterContext::new(self));

        self.items.walk(walker);
    }
}

// Just For Test
#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MeshLoadOptions {
    scale: f32,
}

impl Default for MeshLoadOptions {
    fn default() -> Self {
        Self { scale: 1.0 }
    }
}

impl Mesh {
    pub fn try_from_obj_file<P: AsRef<Path> + fmt::Debug>(
        path: P,
        material: MaterialPtr,
        opt: MeshLoadOptions,
    ) -> anyhow::Result<Mesh> {
        let (models, _) = tobj::load_obj(
            &path,
            &tobj::LoadOptions {
                triangulate: true,
                ..Default::default()
            },
        )
        .context("failed to load obj file")?;

        Self::from_obj_model(models, material, opt)
    }

    pub fn try_from_obj_str(
        obj_str: &str,
        material: MaterialPtr,
        opt: MeshLoadOptions,
    ) -> anyhow::Result<Mesh> {
        let mut reader = BufReader::new(obj_str.as_bytes());
        let (models, _) = tobj::load_obj_buf(
            &mut reader,
            &tobj::LoadOptions {
                triangulate: true,
                ..Default::default()
            },
            |_| Err(tobj::LoadError::MaterialParseError),
        )?;

        Self::from_obj_model(models, material, opt)
    }

    fn from_obj_model(
        models: Vec<tobj::Model>,
        material: MaterialPtr,
        opt: MeshLoadOptions,
    ) -> anyhow::Result<Mesh> {
        // only load first model
        let model = models.get(0).ok_or(anyhow!("no model found"))?;

        let mesh = &model.mesh;

        let mut faces: Vec<Arc<Triangle>> = Vec::new();
        for f in (0..mesh.indices.len()).step_by(3) {
            let vertices_indices = &mesh.indices[f..(f + 3)];

            let vertices: Vec<Vec3> = vertices_indices
                .iter()
                .map(|idx| {
                    let idx = idx.clone() as usize * 3;
                    let v = Vec3::from_column_slice(&mesh.positions[idx..(idx + 3)]);
                    // try scale first
                    let v = v * opt.scale;
                    v
                })
                .collect();

            let vertices: [Vec3; 3] = vertices
                .try_into()
                .map_err(|_| anyhow!("failed to convert vertices to [_;3]"))?;

            let texcoords = if !mesh.texcoord_indices.is_empty() {
                let texture_indices = &mesh.texcoord_indices[f..(f + 3)];
                let texcoords: Vec<Vec2> = texture_indices
                    .iter()
                    .map(|idx| {
                        let idx = idx.clone() as usize * 2;
                        Vec2::from_column_slice(&mesh.texcoords[idx..(idx + 2)])
                    })
                    .collect();
                let texcoords: [Vec2; 3] = texcoords
                    .try_into()
                    .map_err(|_| anyhow!("failed to convert texcoords to [_;2]"))?;
                Some(texcoords)
            } else {
                None
            };

            let triangle = Arc::new(Triangle::new(vertices, texcoords, material.clone()));
            faces.push(triangle);
        }

        Ok(Self::new(faces))
    }
}
