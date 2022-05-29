use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cell::RefCell, sync::Arc};

use crate::core::{Scene, Settings};
use crate::{
    core::Project,
    core::ScatterRecord,
    math::{
        pdfs::{HittablePDF, MixturePDF},
        PDF,
    },
    utils::random,
};
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
                    trace!(
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
                    let c = self.trace_ray(&r, &scene, settings, max_depth);
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

    fn trace_ray<'a>(&self, r: &Ray, scene: &Scene, settings: &'a Settings, depth: i32) -> Color3 {
        // no more light is gathered when reach the limit
        if depth <= 0 {
            return Color3::zeros();
        }

        let world = &scene.world;

        if let Some(rec) = world.hit(r, 0.001, f32::MAX) {
            // TODO: remove usage of material
            let material = rec.material.unwrap();
            let emitted = material.emitted(&r, &rec);
            if let Some(ScatterRecord {
                specular_ray,
                attenuation,
                pdf: pdf_scatter,
            }) = material.scatter(r, &rec)
            {
                // 镜面反射
                if let Some(specular_ray) = specular_ray {
                    return vec3::elementwise_mult(
                        &attenuation,
                        &self.trace_ray(&specular_ray, scene, settings, depth - 1),
                    );
                }

                // 漫反射
                let mixture_pdf: MixturePDF;
                let light_shape = scene.area_lights[0].get_light_primitive().unwrap();
                let light_pdf = HittablePDF::new(rec.point, light_shape);

                let mixture_pdf: &dyn PDF = match &pdf_scatter {
                    Some(pdf_scatter) => {
                        mixture_pdf =
                            MixturePDF::new(settings.weight, &light_pdf, pdf_scatter.as_ref());
                        &mixture_pdf
                    }
                    None => &light_pdf,
                };

                let scattered = Ray::new(rec.point, mixture_pdf.generate_direction(), r.time());
                let pdf_val = mixture_pdf.pdf_value(&scattered.direction());

                let new_color = self.trace_ray(&scattered, scene, settings, depth - 1);
                return emitted
                    + vec3::elementwise_mult(&attenuation, &new_color)
                        * material.scattering_pdf(r, &rec, &scattered)
                        / pdf_val;
            }
            return emitted;
        }
        // TODO: support multiple infinitite lights
        return scene.infinite_lights[0].color(&r);
    }
}
