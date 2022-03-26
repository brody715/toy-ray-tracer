use std::sync::Arc;

use derive_new::new;

use crate::aabb::{self, AABB};
use crate::hittable::{HitRecord, Hittable, HittablePtr};
use crate::ray::Ray;

#[derive(new)]
pub struct HittableList {
    #[new(default)]
    list: Vec<HittablePtr>,
}

impl From<Vec<HittablePtr>> for HittableList {
    fn from(list: Vec<HittablePtr>) -> Self {
        HittableList { list: list.into() }
    }
}

impl HittableList {
    pub fn add(&mut self, hittable: impl Hittable + 'static) {
        self.list.push(Arc::new(hittable))
    }
}

impl Hittable for HittableList {
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
                            Some(bbox) => Some(aabb::create_surrounding_box(&acc, &bbox)),
                            _ => None,
                        }
                    }),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
