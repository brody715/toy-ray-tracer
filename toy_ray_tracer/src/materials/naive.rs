use crate::{
    bxdfs::{LambertianReflection, NaiveSpecularReflection},
    core::{Bsdf, Material, Spectrum, SurfaceInteraction, TexturePtr},
};

pub struct Lambertian {
    albedo: TexturePtr<Spectrum>,
}

impl Lambertian {
    pub fn new(albedo: TexturePtr<Spectrum>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn compute_bsdf(&self, si: &SurfaceInteraction) -> Option<Bsdf> {
        let mut bsdf = Bsdf::new(si.normal);

        let albedo = self.albedo.evaluate(si);
        bsdf.set_raw(LambertianReflection::new(albedo));

        Some(bsdf)
    }
}

pub struct Metal {
    albedo: TexturePtr<Spectrum>,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: TexturePtr<Spectrum>, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn compute_bsdf(&self, si: &SurfaceInteraction) -> Option<Bsdf> {
        let mut bsdf = Bsdf::new(si.normal);

        bsdf.set_raw(NaiveSpecularReflection::new(
            self.albedo.evaluate(si),
            self.fuzz,
        ));

        Some(bsdf)
    }
}
