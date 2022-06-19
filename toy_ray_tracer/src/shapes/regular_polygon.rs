use crate::core::{Shape, Transform, Vec3f};

use super::{create_triangles, ShapeList};

// 正多边形由 `n_sides` 个三角形组成，定义于 xy 平面，规定中心坐标为 `center`，中心与每个顶点的距离为 `radius`，中心序号为 0，顶点序号从 1 开始逆时针排列。
// 如正三角形顶点序号为 [1, 2, 3] （逆时针），由三个三角形 [(0, 1, 2), (0, 2, 3), (0, 3, 1)]，法向量向外 (+z)
//   1
//   0
// 2   3
// 规定，所有的正多边形的 1 号顶点位于中心的正上方，比如正方形变为正菱形。默认。使用时可通过 `object_to_world` 调整位置和方向。

pub struct RegularPolygon {
    triangles: ShapeList,
}

impl RegularPolygon {
    // 通过不断地逆时针旋转 \theta，得到所有的顶点坐标
    pub fn new(radius: f32, n_sides: usize, object_to_world: Transform) -> Self {
        if n_sides < 3 {
            panic!("n_sides must be >= 3");
        }

        // rotation origin
        let center = Vec3f::zeros();

        let rotate_transform = Transform::rotate(Vec3f::new(0.0, 0.0, 1.0), 360.0 / n_sides as f32);

        let v1 = center + Vec3f::new(0.0, radius, 0.0);
        let mut positions: Vec<Vec3f> = vec![center, v1];

        for i in 2..(n_sides + 1) {
            let v = rotate_transform.transform_point3(&positions[i - 1]);
            positions.push(v);
        }

        let mut indices: Vec<usize> = vec![];
        for i in 1..(n_sides + 1) {
            indices.push(0);
            indices.push(i);
            if i == n_sides {
                indices.push(1)
            } else {
                indices.push(i + 1)
            }
        }

        let triangles = create_triangles(indices, positions, object_to_world).unwrap();

        Self { triangles }
    }
}

impl Shape for RegularPolygon {
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<crate::core::AABB> {
        self.triangles.bounding_box(t0, t1)
    }

    fn intersect(
        &self,
        ray: &crate::core::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::core::SurfaceInteraction> {
        self.triangles.intersect(ray, t_min, t_max)
    }

    fn intersect_p(&self, ray: &crate::core::Ray) -> bool {
        self.triangles.intersect_p(ray)
    }

    fn sample_pdf(&self, point: &crate::core::Point3f, wi: &Vec3f) -> f32 {
        self.triangles.sample_pdf(point, wi)
    }

    fn sample_wi(&self, point: &crate::core::Point3f) -> Vec3f {
        self.triangles.sample_wi(point)
    }
}
