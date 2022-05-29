use std::sync::Arc;

use derive_new::new;

use crate::core::{Ray, AABB};
use crate::core::{HitRecord, Primitive, PrimitivePtr};
use crate::utils::random;

#[derive(new)]
pub struct HittableList {
    #[new(default)]
    list: Vec<PrimitivePtr>,
}

#[allow(dead_code)]
pub type HittableListPtr = Arc<HittableList>;

impl From<Vec<PrimitivePtr>> for HittableList {
    fn from(list: Vec<PrimitivePtr>) -> Self {
        HittableList { list: list.into() }
    }
}

impl HittableList {
    pub fn add(&mut self, hittable: impl Primitive + 'static) {
        self.list.push(Arc::new(hittable))
    }
}

impl Primitive for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for h in self.list.iter() {
            if let Some(hit) = h.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        match self.list.first() {
            Some(first) => {
                match first.bounding_box(t0, t1) {
                    Some(bbox) => self.list.iter().skip(1).try_fold(bbox, |acc, hittable| {
                        match hittable.bounding_box(t0, t1) {
                            Some(bbox) => Some(acc.union_bbox(&bbox)),
                            _ => None,
                        }
                    }),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn pdf_value(&self, origin: &crate::core::Point3f, v: &crate::core::Vec3f) -> f32 {
        let weight = 1.0 / self.list.len() as f32;

        let sum = self
            .list
            .iter()
            .map(|h| h.pdf_value(origin, v) * weight)
            .fold(0.0 as f32, |acc, v| acc + v);

        return sum;
    }

    fn random(&self, origin: &crate::core::Vec3f) -> crate::core::Vec3f {
        let idx = random::usize(0..self.list.len());
        return self.list[idx].random(origin);
    }
}
