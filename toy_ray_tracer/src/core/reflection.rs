use std::rc::Rc;

use super::{vec3, Point2f, Spectrum, Vec3f};
use std::f32::consts::PI;

use enumflags2::{bitflags, BitFlags};

#[derive(Clone)]
pub struct Bsdf {
    pub eta: f32,
    pub normal: Vec3f,
    pub bxdfs: Vec<BxdfPtr>,
}

impl Bsdf {
    pub fn new(eta: f32, normal: Vec3f) -> Self {
        Self {
            eta,
            normal,
            bxdfs: Vec::with_capacity(8),
        }
    }

    pub fn add(&mut self, bxdf: BxdfPtr) {
        self.bxdfs.push(bxdf);
    }
}

#[derive(Default)]
pub struct BsdfSampleFRecord {
    pub f: Spectrum,
    pub sampled_flag: BxdfTypeFlags,
    pub pdf: f32,
    pub wi_world: Vec3f,
}

pub struct BxdfSampleFRecord {
    pub f: Spectrum,
    pub sampled_flag: BxdfTypeFlags,
    pub pdf: f32,
    pub wi: Vec3f,
}

impl Bsdf {
    fn world_to_local(&self, v: &Vec3f) -> Vec3f {
        // TODO: add local space
        return v.clone();
    }

    fn local_to_world(&self, v: &Vec3f) -> Vec3f {
        // TODO: add local space
        return v.clone();
    }

    // Without importance sampling, f(p, w_o, w_i)
    pub fn f(&self, wo: &Vec3f, wi: &Vec3f) -> Spectrum {
        let cos_theta_o = wo.dot(&self.normal);
        let cos_theta_i = wi.dot(&self.normal);

        let is_reflected = cos_theta_o * cos_theta_i > 0.0;

        let mut f = Spectrum::zeros();
        for bxdf in self.bxdfs.iter() {
            let cur_flag = bxdf.get_type();
            // either reflection or transmission
            if (is_reflected && cur_flag.contains(BxdfType::Reflection))
                || (!is_reflected && cur_flag.contains(BxdfType::Transmission))
            {
                f += bxdf.f(wo, wi);
            }
        }

        f
    }

    pub fn num_components(&self, flags: BxdfTypeFlags) -> usize {
        self.bxdfs
            .iter()
            .filter(|bxdf| bxdf.match_flags(flags))
            .count()
    }

    // Importance sampling
    // sample_f, f with sampling or delta distribution
    // u -> random sampled points
    pub fn sample_f(
        &self,
        wo_world: &Vec3f,
        u: Point2f,
        bsdf_flags: BxdfTypeFlags,
    ) -> BsdfSampleFRecord {
        let mut rec = BsdfSampleFRecord::default();
        // 随机选择一个 BxDF 进行采样
        let matching_comps = self.num_components(bsdf_flags);
        if matching_comps == 0 {
            rec.pdf = 0.0;
            return rec;
        }

        let comp = std::cmp::min(
            (u[0] * matching_comps as f32).floor() as usize,
            matching_comps - 1,
        );

        let (bxdf, bxdf_index) = {
            let mut bxdf: Option<_> = None;
            let mut bxdf_index: usize = 0;

            let mut count = comp;
            for i in 0..self.bxdfs.len() {
                let matched = self.bxdfs[i].match_flags(bsdf_flags);
                if matched && count == 0 {
                    count -= 1;
                    bxdf = self.bxdfs.get(i);
                    bxdf_index = i;
                    break;
                } else {
                    // fix count, if greater
                    if matched {
                        count -= 1;
                    }
                }
            }
            (bxdf, bxdf_index)
        };

        if let Some(bxdf) = bxdf {
            // Remap BxDF sample u to [0, 1]^2
            let u_remapped = Point2f::new(
                (u[0] * matching_comps as f32 - comp as f32).min(0.99999994),
                u[1],
            );

            let wo = self.world_to_local(wo_world);
            if wo[2] == 0.0 {
                return BsdfSampleFRecord::default();
            }

            rec.pdf = 0.0;
            // Sample chosen BxDF, get rec
            let bxdf_rec = bxdf.sample_f(wo_world, u_remapped, bxdf.get_type());

            if bxdf_rec.pdf == 0.0 {
                if rec.sampled_flag != BxdfTypeFlags::empty() {
                    rec.sampled_flag = BxdfTypeFlags::empty();
                }
                return rec;
            }

            {
                rec.f = bxdf_rec.f;
                rec.pdf = bxdf_rec.pdf;
                rec.wi_world = self.local_to_world(&bxdf_rec.wi);
                rec.sampled_flag = bxdf.get_type();
            }

            // Compute overall PDF with all matching BxDFs, if not specular
            if bxdf.match_flags(BxdfType::Specular.into()) && matching_comps > 1 {
                for i in 0..self.bxdfs.len() {
                    if i == bxdf_index {
                        continue;
                    }

                    let bxdf = &self.bxdfs[i];
                    if bxdf.match_flags(bsdf_flags) {
                        rec.pdf += bxdf.pdf(wo_world, &rec.wi_world);
                    }
                }
            }

            // average pdf
            if matching_comps > 1 {
                rec.pdf /= matching_comps as f32;
            }

            // Compute value for BSDF for sampled direction
            // If not specular
            if !bxdf.match_flags(BxdfType::Specular.into()) && matching_comps > 1 {
                let reflected = rec.wi_world.dot(&self.normal) * wo_world.dot(&self.normal) > 0.0;

                let mut f = Spectrum::zeros();

                for bxdf in self.bxdfs.iter() {
                    let cur_flag = bxdf.get_type();
                    // either reflection or transmission
                    if (reflected && cur_flag.contains(BxdfType::Reflection))
                        || (!reflected && cur_flag.contains(BxdfType::Transmission))
                    {
                        f += bxdf.f(wo_world, &rec.wi_world);
                    }
                }

                rec.f = f;
            }

            return rec;
        }

        return BsdfSampleFRecord::default();
    }

    // pdf sample pdf
    pub fn pdf(&self, wo: &Vec3f, wi: &Vec3f) -> f32 {
        let mut pdf = 0.0;
        let mut n_matching = 0;

        for bxdf in self.bxdfs.iter() {
            pdf += bxdf.pdf(wo, wi);
            n_matching += 1;
        }

        if n_matching > 0 {
            pdf / (n_matching as f32)
        } else {
            0.0
        }
    }
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BxdfType {
    Reflection,
    Transmission,
    Diffuse,
    Glossy,
    Specular,
    All,
}

pub type BxdfTypeFlags = BitFlags<BxdfType>;

impl Default for BxdfType {
    fn default() -> Self {
        BxdfType::All
    }
}

pub trait Bxdf {
    fn match_flags(&self, flags: BxdfTypeFlags) -> bool {
        self.get_type().contains(flags)
    }

    fn get_type(&self) -> BxdfTypeFlags;

    // Without importance sampling, f(p, w_o, w_i)
    fn f(&self, wo: &Vec3f, wi: &Vec3f) -> Spectrum;

    // Importance sampling
    fn sample_f(&self, wo: &Vec3f, u: Point2f, bxdf_flags: BxdfTypeFlags) -> BxdfSampleFRecord;

    // pdf sample pdf
    fn pdf(&self, wo: &Vec3f, wi: &Vec3f) -> f32;
}

pub type BxdfPtr = Rc<dyn Bxdf>;

pub struct LambertianReflection {
    pub albedo: Spectrum,
}

impl LambertianReflection {
    pub fn new(albedo: Spectrum) -> Self {
        Self { albedo }
    }
}

// impl Bxdf for LambertianReflection {
//     fn get_type(&self) -> BxdfTypeFlags {
//         BxdfType::Diffuse | BxdfType::Reflection
//     }

//     fn f(&self, wo: &Vec3f, wi: &Vec3f) -> Spectrum {
//         self.albedo * FRAC_1_PI
//     }

//     fn pdf(&self, wo: &Vec3f, wi: &Vec3f) -> f32 {
//         todo!()
//     }

//     fn sample_f(&self, wo: &Vec3f) -> BxdfSampleFRecord {
//         todo!()
//     }
// }

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
