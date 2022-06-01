use std::rc::Rc;

use super::{vec3, Vec3f};
use std::f32::consts::PI;

pub struct Bsdf {
    normal: Vec3f,
    bxdf: BxdfPtr,
}

impl Bsdf {
    pub fn new(normal: Vec3f, bxdf: BxdfPtr) -> Self {
        Self { normal, bxdf }
    }

    pub fn set_bxdf(&mut self, bxdf: BxdfPtr) {
        self.bxdf = bxdf;
    }
}

impl Bsdf {
    pub fn is_delta(&self) -> bool {
        self.bxdf.is_delta()
    }

    pub fn f(&self, wi: &Vec3f, wo: &Vec3f) -> Vec3f {
        self.bxdf.f(wi, wo)
    }

    pub fn sample(&self, wo: &Vec3f) -> BxdfSampleRecord {
        self.bxdf.sample(wo, &self.normal)
    }
}

pub struct BxdfSampleRecord {
    pub wi: Vec3f,
    pub pdf: f32,
}

pub trait Bxdf {
    // delta distribution, like specular
    fn is_delta(&self) -> bool;

    // f_r(w_i, w_o) -> value, brdf function
    fn f(&self, wi: &Vec3f, wo: &Vec3f) -> Vec3f;

    // importance sampling
    fn sample(&self, wo: &Vec3f, normal: &Vec3f) -> BxdfSampleRecord;
}

pub type BxdfPtr = Rc<dyn Bxdf>;

// schlick of fresnel
pub fn fresnet_schlick(specular: &Vec3f, normal: &Vec3f, outgoing: &Vec3f) -> Vec3f {
    if specular == &Vec3f::zeros() {
        return Vec3f::zeros();
    }

    let cosine = normal.dot(outgoing);

    let n = (1.0 - cosine.abs()).clamp(0.0, 1.0);
    let one = Vec3f::new(1.0, 1.0, 1.0);
    return specular + (one - specular) * n.powi(5);
}

// dielectrics of the fresnel
pub fn fresnel_dielectric(eta: f32, normal: &Vec3f, outgoing: &Vec3f) -> f32 {
    let cosw = normal.dot(outgoing).abs();
    let sin2 = 1.0 - cosw * cosw;
    let eta2 = eta * eta;

    let cos2t = 1.0 - sin2 / eta2;
    if cos2t < 0.0 {
        return 1.0;
    }

    let t0 = cos2t.sqrt();
    let t1 = eta * t0;
    let t2 = eta * cosw;

    let rs = (cosw - t1) / (cosw + t1);
    let rp = (t0 - t2) / (t0 + t2);

    return (rs * rs + rp * rp) / 2.0;
}

pub fn fresnel_conductor(eta: &Vec3f, etak: &Vec3f, normal: &Vec3f, outgoing: &Vec3f) -> Vec3f {
    let cosw = normal.dot(outgoing);
    if cosw < 0.0 {
        return Vec3f::zeros();
    }

    let cosw = cosw.clamp(-1.0, 1.0);
    let cosw2 = cosw * cosw;
    let cosw2v = vec3::scalar(cosw2);
    let sinw2 = (1.0 - cosw2).clamp(0.0, 1.0);
    let eta2 = vec3::elementwise_mult(eta, eta);
    let etak2 = vec3::elementwise_mult(etak, etak);

    let t0 = eta2 - etak2 - cosw2v;
    let a2_plus_b2 =
        vec3::sqrt(vec3::elementwise_mult(&t0, &t0) + 4.0 * vec3::elementwise_mult(&eta2, &etak2));
    let t1 = a2_plus_b2 + cosw2v;

    let a = vec3::sqrt((a2_plus_b2 + t0) / 2.0);
    let t2 = 2.0 * a * cosw;
    let rs = vec3::elementwise_div(&(t1 - t2), &(t1 + t2));

    let t3 = cosw2 * a2_plus_b2 + vec3::scalar(sinw2 * sinw2);
    let t4 = t2 * sinw2;
    let rp = vec3::elementwise_mult(&rs, &vec3::elementwise_div(&(t3 - t4), &(t3 + t4)));

    return (rp + rs) / 2.0;
}

// microfacet distribution evaluation
// @see http://graphicrants.blogspot.com/2013/08/specular-brdf-reference.html
pub fn microfacet_distribution(roughness: f32, normal: &Vec3f, halfway: &Vec3f) -> f32 {
    // TODO: Support ggx

    let cosine = normal.dot(halfway);
    if cosine <= 0.0 {
        return 0.0;
    }

    let roughness2 = roughness * roughness;
    let cosine2 = cosine * cosine;

    return roughness2
        / (PI * (cosine2 * roughness2 + 1.0 - cosine2) * (cosine2 * roughness2 + 1.0 - cosine2));
}
