use super::create_triangles;
use super::shape_list::ShapeList;
use crate::core::Ray;
use crate::core::Shape;
use crate::core::SurfaceInteraction;
use crate::core::Transform;
use crate::core::Vec3f;
use crate::core::AABB;

pub struct Cube {
    triangles: ShapeList,
}

impl Cube {
    pub fn new(p_min: Vec3f, p_max: Vec3f, object_to_world: Transform) -> Self {
        // cube has 8 vertices and 12 triangles

        let positions = vec![
            Vec3f::new(p_min.x, p_min.y, p_min.z),
            Vec3f::new(p_min.x, p_min.y, p_max.z),
            Vec3f::new(p_min.x, p_max.y, p_min.z),
            Vec3f::new(p_min.x, p_max.y, p_max.z),
            Vec3f::new(p_max.x, p_min.y, p_min.z),
            Vec3f::new(p_max.x, p_min.y, p_max.z),
            Vec3f::new(p_max.x, p_max.y, p_min.z),
            Vec3f::new(p_max.x, p_max.y, p_max.z),
        ];

        // 12 triangles, 36 indices
        let indices = vec![
            0, 1, 2, 1, 3, 2, 4, 6, 5, 5, 6, 7, 0, 2, 4, 4, 2, 6, 1, 5, 3, 5, 7, 3, 0, 4, 1, 4, 5,
            1, 2, 3, 6, 3, 7, 6,
        ];

        let triangles = create_triangles(indices, positions, object_to_world).unwrap();

        Self { triangles }
    }
}

impl Shape for Cube {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<SurfaceInteraction> {
        self.triangles.intersect(&ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.triangles.bounding_box(t0, t1)
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        // naive implementation
        self.triangles.intersect_p(ray)
    }

    fn sample_pdf(&self, point: &crate::core::Point3f, wi: &Vec3f) -> f32 {
        self.triangles.sample_pdf(point, wi)
    }

    fn sample_wi(&self, point: &crate::core::Point3f) -> Vec3f {
        self.triangles.sample_wi(point)
    }
}
