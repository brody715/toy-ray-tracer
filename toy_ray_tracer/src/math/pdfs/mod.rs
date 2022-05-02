use crate::utils::random;
use std::f32::consts::PI;

use crate::{
    hittable::HittableRef,
    vec::{vec3, Point3, Vec3},
};

use super::{ONB, PDF};

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    #[must_use]
    #[allow(dead_code)]
    pub fn new(uvw: ONB) -> Self {
        Self { uvw }
    }
}

impl From<Vec3> for CosinePDF {
    fn from(v: Vec3) -> Self {
        Self {
            uvw: ONB::build_form_w(&v),
        }
    }
}

impl PDF for CosinePDF {
    fn pdf_value(&self, direction: &crate::vec::Vec3) -> f32 {
        let cosine = direction.normalize().dot(&self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate_direction(&self) -> crate::vec::Vec3 {
        self.uvw.local(vec3::random_cosine_direction())
    }
}

pub struct HittablePDF<'a> {
    o: Point3,
    hittable: HittableRef<'a>,
}

impl<'a> HittablePDF<'a> {
    pub fn new(o: Point3, hittable: HittableRef<'a>) -> Self {
        Self { o, hittable }
    }
}

impl<'a> PDF for HittablePDF<'a> {
    fn pdf_value(&self, direction: &crate::vec::Vec3) -> f32 {
        return self.hittable.pdf_value(&self.o, &direction);
    }

    fn generate_direction(&self) -> crate::vec::Vec3 {
        return self.hittable.random(&self.o);
    }
}

pub struct NopPDF;

impl PDF for NopPDF {
    fn pdf_value(&self, _direction: &Vec3) -> f32 {
        1.0
    }

    fn generate_direction(&self) -> Vec3 {
        todo!()
    }
}

pub struct WrapperPDF<'a> {
    p: &'a dyn PDF,
}

impl<'a> WrapperPDF<'a> {
    #[allow(dead_code)]
    pub fn new(p: &'a dyn PDF) -> Self {
        Self { p }
    }
}

impl<'a> PDF for WrapperPDF<'a> {
    fn pdf_value(&self, direction: &Vec3) -> f32 {
        return self.p.pdf_value(direction);
    }

    fn generate_direction(&self) -> Vec3 {
        return self.p.generate_direction();
    }
}

pub struct MixturePDF<'a> {
    p: [&'a dyn PDF; 2],
}

impl<'a> MixturePDF<'a> {
    #[must_use]
    pub fn new(p0: &'a dyn PDF, p1: &'a dyn PDF) -> Self {
        Self { p: [p0, p1] }
    }
}

const WEIGHT: f32 = 0.5;
impl<'a> PDF for MixturePDF<'a> {
    fn pdf_value(&self, direction: &Vec3) -> f32 {
        return WEIGHT * self.p[0].pdf_value(direction) + (1.0 - WEIGHT) * self.p[1].pdf_value(direction);
    }

    fn generate_direction(&self) -> Vec3 {
        if random::f32() < WEIGHT {
            self.p[0].generate_direction()
        } else {
            self.p[1].generate_direction()
        }
    }
}
