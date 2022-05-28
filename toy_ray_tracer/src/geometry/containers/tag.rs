use std::collections::{HashMap, HashSet};

use crate::{
    geometry::EnterContext,
    core::{Hittable, HittablePtr},
};

type PropertiesType = Option<HashMap<String, serde_json::Value>>;

pub struct TagsHittable {
    pub(crate) tags: HashSet<String>,
    _properties: PropertiesType,
    pub(crate) child: HittablePtr,
}

impl TagsHittable {
    pub fn new(tags: HashSet<String>, child: HittablePtr, properties: PropertiesType) -> Self {
        Self {
            tags,
            child,
            _properties: properties,
        }
    }
}

impl Hittable for TagsHittable {
    fn hit(
        &self,
        ray: &crate::core::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::core::HitRecord> {
        self.child.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<crate::core::AABB> {
        self.child.bounding_box(t0, t1)
    }

    fn accept(&self, visitor: &mut dyn crate::geometry::GeometryVisitor) {
        visitor.visit_tags_hittable(self);
    }

    fn walk(&self, walker: &mut dyn crate::geometry::GeometryWalker) {
        walker.enter_tags_hittable(EnterContext::new(self));
        self.child.walk(walker);
    }

    fn pdf_value(&self, origin: &crate::core::Point3, v: &crate::core::Vec3) -> f32 {
        self.child.pdf_value(origin, v)
    }

    fn random(&self, origin: &crate::core::Vec3) -> crate::core::Vec3 {
        self.child.random(origin)
    }
}
