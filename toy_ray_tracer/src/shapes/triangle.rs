use std::sync::Arc;

use anyhow::{ensure, Result};

use crate::{
    core::AABB,
    core::{vec3, Point2f, Shape, Vec3f},
    core::{HitRecord, Point3f},
};

#[derive(Clone)]
pub struct Triangle {
    id: usize,
    mesh: Arc<TriangleMeshStorage>,
}

impl Triangle {
    pub fn new(id: usize, mesh: Arc<TriangleMeshStorage>) -> Self {
        Self { id, mesh }
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: &crate::core::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // @see https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection
        // ray traingle intersection

        let mesh = self.mesh.as_ref();

        let idx = self.id * 3;
        let indices = &mesh.vertex_indices[idx..(idx + 3)];
        let v0 = mesh.positions[indices[0]];
        let v1 = mesh.positions[indices[1]];
        let v2 = mesh.positions[indices[2]];

        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;
        let pvec = ray.direction().cross(&v0v2);
        let det = v0v1.dot(&pvec);

        if det.abs() < 1e-6 {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = ray.origin() - v0;

        let b1 = tvec.dot(&pvec) * inv_det;
        if b1 < 0.0 || b1 > 1.0 {
            return None;
        }

        let qvec = tvec.cross(&v0v1);
        let b2 = ray.direction().dot(&qvec) * inv_det;
        if b2 < 0.0 || b1 + b2 > 1.0 {
            return None;
        }

        let b0 = 1.0 - b1 - b2;

        let p = b0 * v0 + b1 * v1 + b2 * v2;

        let t = v0v2.dot(&qvec) * inv_det;

        if t < t_min || t > t_max {
            return None;
        }

        let uvs: [Point2f; 3] = if mesh.uvs.is_empty() {
            [
                Point2f::new(0.0, 0.0),
                Point2f::new(1.0, 0.0),
                Point2f::new(1.0, 1.0),
            ]
        } else {
            let m_uvs = &mesh.uvs;
            [m_uvs[indices[0]], m_uvs[indices[1]], m_uvs[indices[2]]]
        };

        let uv = uvs[0] * b0 + uvs[1] * b1 + uvs[2] * b2;

        let mut rec = HitRecord::new(t, uv, p);
        rec.set_face_normal(ray, &mesh.surface_normals[self.id]);

        return Some(rec);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let mesh = self.mesh.as_ref();

        let idx = self.id * 3;
        let indices = &mesh.vertex_indices[idx..(idx + 3)];
        let v0 = mesh.positions[indices[0]];
        let v1 = mesh.positions[indices[1]];
        let v2 = mesh.positions[indices[2]];

        let mut min = vec3::min(&vec3::min(&v0, &v1), &v2);
        let mut max = vec3::max(&vec3::max(&v0, &v1), &v2);

        for axis in 0..3 {
            if min[axis] == max[axis] {
                min[axis] -= 0.001;
                max[axis] += 0.001;
            }
        }

        let bbox = AABB::new(min, max);
        Some(bbox)
    }
}

pub struct TriangleMeshStorage {
    pub n_triangles: usize,
    // vertex indices, 3 * n_triangles, [0, 0, 0, 1, 1, 1, id, id, id], id=<triangle id>
    pub vertex_indices: Vec<usize>,
    // 3 * n_triangles
    pub positions: Vec<Point3f>,

    // vertex normals, can be empty
    pub vertex_normals: Vec<Vec3f>,
    // vertex uv, can be empty
    pub uvs: Vec<Point2f>,

    // surface normals, by triangle_id
    pub surface_normals: Vec<Vec3f>,
}

impl TriangleMeshStorage {
    pub fn try_new(
        n_triangles: usize,
        vertex_indices: Vec<usize>,
        positions: Vec<Point3f>,
        normals: Vec<Vec3f>,
        uvs: Vec<Point2f>,
    ) -> Result<Self> {
        let n_vertices = n_triangles * 3;
        ensure!(n_vertices == vertex_indices.len());

        let max_idx = vertex_indices.iter().max().unwrap_or(&0).to_owned();
        ensure!(max_idx < positions.len());

        let vertex_normals = if !normals.is_empty() {
            ensure!(max_idx < normals.len());
            normals
        } else {
            vec![]
        };

        let uvs = if !uvs.is_empty() {
            ensure!(max_idx < uvs.len());
            uvs
        } else {
            vec![]
        };

        let surface_normals: Vec<Vec3f> = vertex_indices[..]
            .chunks(3)
            .map(|indices| {
                let v0 = positions[indices[0]];
                let v1 = positions[indices[1]];
                let v2 = positions[indices[2]];

                let n = (v1 - v0).cross(&(v2 - v0)).normalize();

                n
            })
            .collect();

        Ok(Self {
            n_triangles,
            vertex_indices,
            positions,
            vertex_normals,
            uvs,
            surface_normals,
        })
    }
}
