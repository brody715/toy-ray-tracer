use std::sync::Arc;

use derive_new::new;

use crate::core::{HitRecord, Primitive, PrimitivePtr};
use crate::core::{Ray, AABB};
use crate::utils::random;

#[derive(new)]
pub struct PrimitiveList {
    #[new(default)]
    items: Vec<PrimitivePtr>,
}

impl From<&[PrimitivePtr]> for PrimitiveList {
    fn from(list: &[PrimitivePtr]) -> Self {
        PrimitiveList { items: list.into() }
    }
}

impl PrimitiveList {
    #[allow(dead_code)]
    pub fn add(&mut self, primitive: impl Primitive + 'static) {
        self.items.push(Arc::new(primitive))
    }
}

impl Primitive for PrimitiveList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for h in self.items.iter() {
            if let Some(hit) = h.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let bbox = self.items.iter().fold(None, |acc, item| {
            AABB::union_optional_bbox(&acc, &item.bounding_box(t0, t1))
        });
        bbox
    }

    fn pdf_value(&self, origin: &crate::core::Point3f, v: &crate::core::Vec3f) -> f32 {
        let weight = 1.0 / self.items.len() as f32;

        let sum = self
            .items
            .iter()
            .map(|h| h.pdf_value(origin, v) * weight)
            .fold(0.0 as f32, |acc, v| acc + v);

        return sum;
    }

    fn random(&self, origin: &crate::core::Vec3f) -> crate::core::Vec3f {
        let idx = random::usize(0..self.items.len());
        return self.items[idx].random(origin);
    }
}
