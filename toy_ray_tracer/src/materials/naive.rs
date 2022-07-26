use crate::{
    bxdfs::{LambertianReflection, NaiveDielectric, NaiveSpecularReflection},
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

pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn compute_bsdf(&self, si: &crate::core::SurfaceInteraction) -> Option<Bsdf> {
        let mut bsdf = Bsdf::new(si.normal);

        let ni_over_nt = if si.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        bsdf.set_raw(NaiveDielectric::new(ni_over_nt));
        Some(bsdf)
    }
}
