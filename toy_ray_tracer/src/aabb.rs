use crate::{ray::Ray, vec::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
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
    pub fn union_point3(&self, point: Vec3) -> AABB {
        let min = Vec3::new(
            self.min.x.min(point.x),
            self.min.y.min(point.y),
            self.min.z.min(point.z),
        );

        let max = Vec3::new(
            self.max.x.max(point.x),
            self.max.y.max(point.y),
            self.max.z.max(point.z),
        );

        AABB { min, max }
    }

    pub fn union_bbox(&self, bbox: &AABB) -> AABB {
        let min = Vec3::new(
            self.min.x.min(bbox.min.x),
            self.min.y.min(bbox.min.y),
            self.min.z.min(bbox.min.z),
        );

        let max = Vec3::new(
            self.max.x.max(bbox.max.x),
            self.max.y.max(bbox.max.y),
            self.max.z.max(bbox.max.z),
        );

        AABB { min, max }
    }
}

pub fn create_surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let min = Vec3::new(
        f32::min(box0.min.x, box1.min.x),
        f32::min(box0.min.y, box1.min.y),
        f32::min(box0.min.z, box1.min.z),
    );
    let max = Vec3::new(
        f32::max(box0.max.x, box1.max.x),
        f32::max(box0.max.y, box1.max.y),
        f32::max(box0.max.z, box1.max.z),
    );
    AABB { min, max }
}

pub fn create_sphere_box(center: &Vec3, radius: f32) -> AABB {
    return AABB {
        min: center - Vec3::new(radius, radius, radius),
        max: center + Vec3::new(radius, radius, radius),
    };
}
