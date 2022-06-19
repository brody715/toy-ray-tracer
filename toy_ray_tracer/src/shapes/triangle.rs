use std::sync::Arc;

use anyhow::{ensure, Ok, Result};

use crate::{
    core::AABB,
    core::{vec3, Point2f, Shape, ShapePtr, Transform, Vec3f},
    core::{Point3f, Ray, SurfaceInteraction},
    utils::random,
};

use super::shape_list::ShapeList;

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

impl Triangle {
    // [p0, p1, p2]
    fn get_vertices(&self) -> [Vec3f; 3] {
        let mesh = self.mesh.as_ref();

        let idx = self.id * 3;
        let indices = &mesh.vertex_indices[idx..(idx + 3)];
        let p0 = mesh.positions[indices[0]];
        let p1 = mesh.positions[indices[1]];
        let p2 = mesh.positions[indices[2]];

        return [p0, p1, p2];
    }
}

impl Shape for Triangle {
    fn intersect(
        &self,
        ray: &crate::core::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<SurfaceInteraction> {
        // @see https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection
        // ray traingle intersection

        let mesh = self.mesh.as_ref();

        let idx = self.id * 3;
        let indices = &mesh.vertex_indices[idx..(idx + 3)];
        let p0 = mesh.positions[indices[0]];
        let p1 = mesh.positions[indices[1]];
        let p2 = mesh.positions[indices[2]];

        let p0p1 = p1 - p0;
        let p0p2 = p2 - p0;
        let pvec = ray.direction().cross(&p0p2);
        let det = p0p1.dot(&pvec);

        if det.abs() < 1e-6 {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = ray.origin() - p0;

        let b1 = tvec.dot(&pvec) * inv_det;
        if b1 < 0.0 || b1 > 1.0 {
            return None;
        }

        let qvec = tvec.cross(&p0p1);
        let b2 = ray.direction().dot(&qvec) * inv_det;
        if b2 < 0.0 || b1 + b2 > 1.0 {
            return None;
        }

        let b0 = 1.0 - b1 - b2;

        let hit_p = b0 * p0 + b1 * p1 + b2 * p2;

        let hit_t = p0p2.dot(&qvec) * inv_det;

        if hit_t < t_min || hit_t > t_max {
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

        let surface_normal = &mesh.surface_normals[self.id];
        let si = SurfaceInteraction::new(hit_t, hit_p, uv, -ray.direction(), *surface_normal);

        return Some(si);
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

    // TODO: Optimize light sampling
    fn sample_pdf(&self, point: &Point3f, wi: &Vec3f) -> f32 {
        let ray = Ray::new(*point, *wi, 0.0);
        if let Some(si) = self.intersect(&ray, 0.0, f32::INFINITY) {
            let area = self.mesh.areas[self.id];

            let distance_squared = si.t_hit * si.t_hit;
            let cosine = (wi.dot(&si.normal) / wi.norm()).abs();
            let pdf = distance_squared / (cosine * area);

            pdf
        } else {
            0.0
        }
    }

    fn sample_wi(&self, point: &Point3f) -> Vec3f {
        let [p0, p1, p2] = self.get_vertices();

        // uniform generate point on triangle
        let su0 = random::f32().sqrt();
        let bx = 1.0 - su0;
        let by = random::f32() * su0;

        let hit_p = p0 * bx + p1 * by + p2 * (1.0 - bx - by);
        return (hit_p - point).normalize();
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

    // computed values, cached for optimization
    // surface normals, by triangle_id
    pub surface_normals: Vec<Vec3f>,
    pub areas: Vec<f32>,
}

impl TriangleMeshStorage {
    pub fn try_new(
        n_triangles: usize,
        vertex_indices: Vec<usize>,
        positions: Vec<Point3f>,
        normals: Vec<Vec3f>,
        uvs: Vec<Point2f>,
        object_to_world: Transform,
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

        // start to transform
        let positions: Vec<_> = positions
            .iter()
            .map(|p| object_to_world.transform_point3(p))
            .collect();

        let vertex_normals = vertex_normals
            .iter()
            .map(|n| object_to_world.transform_normal(n))
            .collect();

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

        let areas: Vec<f32> = vertex_indices[..]
            .chunks(3)
            .map(|indices| {
                let v0 = positions[indices[0]];
                let v1 = positions[indices[1]];
                let v2 = positions[indices[2]];

                let area = 0.5 * (v1 - v0).cross(&(v2 - v0)).norm();
                area
            })
            .collect();

        Ok(Self {
            n_triangles,
            vertex_indices,
            positions,
            vertex_normals,
            uvs,
            surface_normals,
            areas,
        })
    }
}

pub fn create_triangles(
    indices: Vec<usize>,
    positions: Vec<Vec3f>,
    transform: Transform,
) -> Result<ShapeList> {
    let n_triangles = indices.len() / 3;
    let mesh = Arc::new(TriangleMeshStorage::try_new(
        indices.len() / 3,
        indices,
        positions,
        vec![],
        vec![],
        transform,
    )?);

    let mut triangles: Vec<ShapePtr> = vec![];

    for id in 0..n_triangles {
        triangles.push(Arc::new(Triangle::new(id, mesh.clone())));
    }

    Ok(ShapeList::from(triangles))
}
