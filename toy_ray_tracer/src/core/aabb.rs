use crate::core::{Ray, Vec3f};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3f,
    pub max: Vec3f,
}

impl AABB {
    pub fn new(min: Vec3f, max: Vec3f) -> Self {
        AABB { min, max }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let t1 = (self.max[a] - ray.origin()[a]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

impl AABB {
    pub fn union_point3(&self, point: Vec3f) -> AABB {
        let min = Vec3f::new(
            self.min.x.min(point.x),
            self.min.y.min(point.y),
            self.min.z.min(point.z),
        );

        let max = Vec3f::new(
            self.max.x.max(point.x),
            self.max.y.max(point.y),
            self.max.z.max(point.z),
        );

        AABB { min, max }
    }

    pub fn union_bbox(&self, bbox: &AABB) -> AABB {
        let min = Vec3f::new(
            self.min.x.min(bbox.min.x),
            self.min.y.min(bbox.min.y),
            self.min.z.min(bbox.min.z),
        );

        let max = Vec3f::new(
            self.max.x.max(bbox.max.x),
            self.max.y.max(bbox.max.y),
            self.max.z.max(bbox.max.z),
        );

        AABB { min, max }
    }

    pub fn union_optional_bbox(box0: &Option<AABB>, box1: &Option<AABB>) -> Option<AABB> {
        match (box0, box1) {
            (Some(box0), Some(box1)) => Some(box0.union_bbox(box1)),
            (Some(box0), None) => Some(box0.clone()),
            (None, Some(box1)) => Some(box1.clone()),
            (None, None) => None,
        }
    }
}
