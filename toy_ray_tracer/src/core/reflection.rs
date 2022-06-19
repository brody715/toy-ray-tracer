use super::{Spectrum, Vec3f};

pub struct Bsdf {
    normal: Vec3f,
    bxdf: BxdfPtr,
}

impl Bsdf {
    pub fn new(normal: Vec3f) -> Self {
        Self {
            normal,
            bxdf: Box::new(NopBxdf {}),
        }
    }

    pub fn set(&mut self, bxdf: BxdfPtr) {
        self.bxdf = bxdf
    }

    pub fn set_raw<T: Bxdf + 'static>(&mut self, bxdf: T) {
        self.bxdf = Box::new(bxdf);
    }
}

impl Bsdf {
    // delta distribution, like specular
    #[inline]
    pub fn is_delta(&self) -> bool {
        self.bxdf.is_delta()
    }

    // f_r(w_i, w_o) -> value, brdf function
    #[inline]
    #[allow(dead_code)]
    fn f(&self, wi: &Vec3f, wo: &Vec3f) -> Spectrum {
        self.bxdf.f(wi, wo, &self.normal)
    }

    // f_r(w_i, w_o) * \cos\theta, brdf with cos attenuation
    #[inline]
    pub fn f_cos(&self, wi: &Vec3f, wo: &Vec3f) -> Spectrum {
        self.bxdf.f(wi, wo, &self.normal) * self.cosine(wi)
    }

    #[inline]
    fn cosine(&self, wi: &Vec3f) -> f32 {
        f32::abs(wi.dot(&self.normal))
    }

    // importance sampling
    #[inline]
    pub fn sample_wi(&self, wo: &Vec3f) -> Vec3f {
        self.bxdf.sample_wi(wo, &self.normal)
    }

    #[inline]
    pub fn sample_pdf(&self, wi: &Vec3f, wo: &Vec3f) -> f32 {
        self.bxdf.sample_pdf(wi, wo, &self.normal)
    }
}

pub trait Bxdf {
    // delta distribution, like specular
    fn is_delta(&self) -> bool;

    // f_r(w_i, w_o) -> value, brdf function
    fn f(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> Vec3f;

    // importance sampling
    fn sample_wi(&self, wo: &Vec3f, normal: &Vec3f) -> Vec3f;

    fn sample_pdf(&self, wi: &Vec3f, wo: &Vec3f, normal: &Vec3f) -> f32;
}

pub type BxdfPtr = Box<dyn Bxdf>;

pub struct NopBxdf {}
impl Bxdf for NopBxdf {
    fn is_delta(&self) -> bool {
        todo!()
    }

    fn f(&self, _wi: &Vec3f, _wo: &Vec3f, _normal: &Vec3f) -> Vec3f {
        todo!()
    }

    fn sample_wi(&self, _wo: &Vec3f, _normal: &Vec3f) -> Vec3f {
        todo!()
    }

    fn sample_pdf(&self, _wi: &Vec3f, _wo: &Vec3f, _normal: &Vec3f) -> f32 {
        todo!()
    }
}
