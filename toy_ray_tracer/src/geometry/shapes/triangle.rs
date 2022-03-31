use crate::{
    aabb::AABB,
    geometry::EnterContext,
    hittable::{HitRecord, Hittable},
    material::MaterialPtr,
    vec::{vec3, Vec2, Vec3},
};

#[derive(Clone)]
pub struct Triangle {
    vertices: [Vec3; 3],
    // texture position for vertices
    texcoords: [Vec2; 3],
    normal: Vec3,
    material: MaterialPtr,
}

impl Triangle {
    pub fn new(vertices: [Vec3; 3], texcoords: Option<[Vec2; 3]>, material: MaterialPtr) -> Self {
        let normal = (vertices[1] - vertices[0])
            .cross(&(vertices[2] - vertices[0]))
            .normalize();

        let texcoords = if let Some(textures) = texcoords {
            textures
        } else {
            [Vec2::zeros(), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)]
        };

        Self {
            vertices,
            texcoords,
            normal,
            material,
        }
    }
}

impl Hittable for Triangle {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        // @see https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection
        // ray traingle intersection

        let v0v1 = self.vertices[1] - self.vertices[0];
        let v0v2 = self.vertices[2] - self.vertices[0];
        let pvec = ray.direction().cross(&v0v2);
        let det = v0v1.dot(&pvec);

        if det.abs() < 1e-6 {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = ray.origin() - self.vertices[0];

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

        let p = b0 * self.vertices[0] + b1 * self.vertices[1] + b2 * self.vertices[2];

        let t = v0v2.dot(&qvec) * inv_det;

        if t < t_min || t > t_max {
            return None;
        }

        let uvw = b0 * self.texcoords[0] + b1 * self.texcoords[1] + b2 * self.texcoords[2];

        let mut rec = HitRecord::new(t, uvw[0], uvw[1], p, self.material.as_ref());
        rec.set_face_normal(ray, &self.normal);

        return Some(rec);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let mut min = vec3::min(
            &vec3::min(&self.vertices[0], &self.vertices[1]),
            &self.vertices[2],
        );
        let mut max = vec3::max(
            &vec3::max(&self.vertices[0], &self.vertices[1]),
            &self.vertices[2],
        );

        for axis in 0..3 {
            if min[axis] == max[axis] {
                min[axis] -= 0.001;
                max[axis] += 0.001;
            }
        }

        let bbox = AABB::new(min, max);
        Some(bbox)
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_triangle(self);
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_triangle(EnterContext::new(self));
    }
}
