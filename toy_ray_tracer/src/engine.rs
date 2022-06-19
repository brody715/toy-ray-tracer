use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cell::RefCell, sync::Arc};

use crate::core::{Light, Scene, Settings, Vec3f};
use crate::{core::Project, utils::random};
use log::trace;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use thread_local::ThreadLocal;

use crate::{
    core::Image,
    core::Ray,
    core::{vec3, Color3, Vec3List},
    utils::ExecutionTimer,
};

pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        return Engine {};
    }

    pub fn render(&self, project: &Project) -> Image {
        let opts = project.settings();
        let width = opts.width;
        let height = opts.height;
        let nsamples = opts.nsamples;
        let max_depth = opts.max_depth;

        let scene = project.scene();
        let settings = project.settings();

        let camera = &scene.camera;

        let tasks_finished = Arc::new(AtomicUsize::new(0));
        let pixels_local = Arc::new(ThreadLocal::new());

        let unit_height = height / 100;

        (0..height).into_par_iter().for_each(|j: usize| {
            let _timer = ExecutionTimer::new(|start_time| {
                let val = tasks_finished.fetch_add(1, Ordering::Relaxed) + 1;
                if val % unit_height == 0 || val == height {
                    log::debug!(
                        "render elapsed {} ms, height_idx={}, progress={}/{} ({:.2}%)",
                        start_time.elapsed().as_millis(),
                        j,
                        val,
                        height,
                        (val as f32 / height as f32) * 100.0
                    )
                }
            });

            let plc = pixels_local.get_or(|| RefCell::new(Vec3List::new_with_size(width * height)));

            let mut pixels = plc.borrow_mut();

            // let light_pdf = HittablePDF::new(o, hittable)

            let rng = random::new_rng();
            for i in 0..width {
                for _s in 0..nsamples {
                    let u = (i as f32 + rng.f32()) / width as f32;
                    let v = (j as f32 + rng.f32()) / height as f32;
                    let r = camera.get_ray(u, v);
                    let c = self.trace_ray_loop(&r, &scene, settings, max_depth);
                    pixels[(height - j - 1) * width + i] += c;
                }
            }
        });

        let mut img = Image::new(width, height);
        {
            let _timer = ExecutionTimer::new(|start_time| {
                trace!("to_image elapsed {:} ms", start_time.elapsed().as_millis(),)
            });

            let pixels_local = Arc::try_unwrap(pixels_local).unwrap();
            let pixels = pixels_local
                .into_iter()
                .fold(Vec3List::new_with_size(width * height), |v1, v2| {
                    v1.add(&v2.borrow())
                });

            for (i, color) in pixels.iter().enumerate() {
                let color = vec3::sqrt(color / nsamples as f32);

                img.set_pixel(i, color);
            }
        }

        return img;
    }

    #[allow(dead_code)]
    fn trace_ray_loop<'a>(
        &self,
        ray: &Ray,
        scene: &Scene,
        settings: &'a Settings,
        max_depth: i32,
    ) -> Color3 {
        let mut ray = ray.clone();
        let mut color = Color3::zeros();
        let mut beta = Vec3f::new(1.0, 1.0, 1.0);

        let world = &scene.world;
        let lights = &scene.lights;

        for bounce in 0..max_depth {
            if let Some(si) = world.intersect(&ray, 0.001, f32::MAX) {
                let material = si.material.unwrap();
                let emission = if si.wo.dot(&si.normal) > 0.0 {
                    material.emission(&si)
                } else {
                    // Color3::zeros()
                    material.emission(&si)
                };

                color = color + vec3::elementwise_mult(&beta, &emission);

                if let Some(bsdf) = material.compute_bsdf(&si) {
                    let wo = &si.wo;

                    // delta 分布 brdf，如 镜面反射
                    if bsdf.is_delta() {
                        let wi = bsdf.sample_wi(wo);
                        let pdf = bsdf.sample_pdf(&wi, wo);
                        let bsdf_value = bsdf.f_cos(&wi, wo);

                        ray = Ray::new(si.point, wi, ray.time());
                        beta = vec3::elementwise_mult(&beta, &bsdf_value) / pdf;
                    } else {
                        // 非 delta 分布 brdf，如 漫反射
                        let lights = &scene.lights;

                        // bsdf and light sampling weight
                        let mis_weight = settings.mis_weight;

                        let wi = if random::f32() < mis_weight {
                            bsdf.sample_wi(wo)
                        } else {
                            lights.sample_wi(&si.point)
                        };

                        // no light ray
                        if vec3::is_near_zero(&wi) {
                            break;
                        }

                        let pdf = mis_weight * bsdf.sample_pdf(&wi, wo)
                            + (1.0 - mis_weight) * lights.sample_pdf(&si.point, &wi);

                        let bsdf_value = bsdf.f_cos(&wi, wo);

                        // black, stop trace
                        if vec3::is_black(&bsdf_value) || pdf < f32::EPSILON {
                            break;
                        }

                        ray = Ray::new(si.point, wi, ray.time());
                        let old_beta = beta;
                        beta = vec3::elementwise_mult(&beta, &bsdf_value) / pdf;

                        if log::max_level() >= log::Level::Trace {
                            log::trace!("wi: {:?}, wo: {:?}, normal: {:?}", &wi, &wo, &si.normal);
                            log::trace!(
                                "wi.norm(): {:?}, wo.norm(): {:?}, normal.norm(): {:?}",
                                &wi.norm(),
                                &wo.norm(),
                                &si.normal.norm()
                            );

                            log::trace!(
                                "pdf: {:?}, bsdf_value: {:?}, beta: {:?}, old_beta: {:?}",
                                pdf,
                                bsdf_value,
                                beta,
                                old_beta,
                            );
                        }
                    }
                } else {
                    // no bsdf for material, stop trace
                    break;
                }
            } else {
                // no hit, return environment lights
                color = color + vec3::elementwise_mult(&beta, &lights.background_l(&ray));
                break;
            }

            if beta == Vec3f::zeros() {
                break;
            }

            if bounce > 3 {
                // prop 0.99 or beta
                let rr_prob = f32::min(0.99, beta.max());

                // rr_prop stop trace
                if random::f32() >= rr_prob {
                    break;
                }

                // (1 - rr_prob) continue to trace
                beta *= 1.0 / rr_prob;
            }
        }

        return color;
    }
}
