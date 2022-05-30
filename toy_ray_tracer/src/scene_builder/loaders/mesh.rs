use std::rc::Rc;

use crate::{
    core::{Point2f, Point3f, Vec3f},
    scene_builder::AssetsManager,
};
use anyhow::{self, Ok, Result};

pub struct MeshLoader {
    pub assets_manager: Rc<AssetsManager>,
}

pub struct MeshBundle {
    pub n_triangles: usize,
    pub indices: Vec<usize>,
    pub positions: Vec<Point3f>,
    pub normals: Vec<Vec3f>,
    pub uvs: Vec<Point2f>,
}

impl MeshLoader {
    pub fn new(assets_manager: Rc<AssetsManager>) -> Self {
        Self { assets_manager }
    }

    fn triangle_align_indices<T: Clone>(
        position_indices: &Vec<usize>,
        src_indices: &Vec<usize>,
        src_datas: &[T],
    ) -> Vec<T> {
        if src_datas.is_empty() {
            return Vec::new();
        }

        // vertex_idx from 0 to n_vertices
        // position_indices[vertex_idx] = position_idx
        // uv_indices[vertex_idx] = uv_idx
        // position = positions[position_idx]
        // uv = uvs[uv_idx]
        // remap all position_idx == uv_idx

        // mapping
        // uvs[position_idx] = uvs[uv_indices[vert_idx]]

        let mut target: Vec<T> = Vec::new();
        target.resize(src_datas.len(), src_datas[0].clone());

        for (vertice_idx, position_idx) in position_indices.iter().enumerate() {
            target[*position_idx] = src_datas[src_indices[vertice_idx]].clone();
        }

        target
    }

    fn vec_u32_to_usize(src: &[u32]) -> Vec<usize> {
        src.iter().map(|&x| x as usize).collect()
    }
}

impl MeshLoader {
    pub fn load(&self, uri: &String) -> Result<MeshBundle> {
        let path = self.assets_manager.load_path(uri)?;

        let (models, _) = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                triangulate: true,
                ..Default::default()
            },
        )?;
        // only load first model
        if models.len() != 1 {
            anyhow::bail!("only support one model, but got {}", models.len());
        }

        let model = models.get(0).unwrap();
        let tmesh = &model.mesh;
        let positions: Vec<Point3f> = tmesh.positions[..]
            .chunks(3)
            .map(|chunk| Point3f::new(chunk[0], chunk[1], chunk[2]))
            .collect();

        let texcoords: Vec<Point2f> = tmesh.texcoords[..]
            .chunks(2)
            .map(|chunk| Point2f::new(chunk[0], chunk[1]))
            .collect();

        let normals: Vec<Vec3f> = tmesh.normals[..]
            .chunks(3)
            .map(|chunk| Vec3f::new(chunk[0], chunk[1], chunk[2]))
            .collect();

        let vertices_indices = Self::vec_u32_to_usize(&tmesh.indices);
        let texcoord_indices = Self::vec_u32_to_usize(&tmesh.texcoord_indices);
        let normal_indices = Self::vec_u32_to_usize(&tmesh.normal_indices);
        // align all indices (positions, uvs, normals, ...)
        let texcoords =
            Self::triangle_align_indices(&vertices_indices, &texcoord_indices, &texcoords);
        let normals = Self::triangle_align_indices(&vertices_indices, &normal_indices, &normals);

        Ok(MeshBundle {
            n_triangles: tmesh.indices.len() / 3,
            indices: vertices_indices,
            positions,
            normals,
            uvs: texcoords,
        })
    }
}
