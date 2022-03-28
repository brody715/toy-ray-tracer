use std::sync::atomic::{AtomicI32, Ordering};
use std::{cell::RefCell, sync::Arc};

use crate::{
    environment::Sky,
    material::ScatterRecord,
    math::{
        pdfs::{HittablePDF, MixturePDF, WrapperPDF},
        PDF,
    },
    project::Project,
    utils::random,
};
use log::trace;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use thread_local::ThreadLocal;

use crate::{
    hittable::Hittable,
    nimage::Image,
    ray::Ray,
    utils::ExecutionTimer,
    vec::{vec3, Color3, Vec3List},
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

        let world = scene.world();
        let camera = scene.camera();
        let sky = scene.sky();
        let lights = scene.lights();

        let tasks_finished = Arc::new(AtomicI32::new(0));
        let pixels_local = Arc::new(ThreadLocal::new());

        (0..nsamples).into_par_iter().for_each(|s: i32| {
            let _timer = ExecutionTimer::new(|start_time| {
                let val = tasks_finished.fetch_add(1, Ordering::Relaxed) + 1;
                trace!(
                    "render elapsed {} ms, sample_idx={}, progress={}/{} ({:.2}%)",
                    start_time.elapsed().as_millis(),
                    s,
                    val,
                    nsamples,
                    (val as f32 / nsamples as f32) * 100.0
                )
            });

            let plc = pixels_local.get_or(|| RefCell::new(Vec3List::new_with_size(width * height)));

            let mut pixels = plc.borrow_mut();

            let rng = random::new_rng();
            for j in 0..height {
                for i in 0..width {
                    let u = (i as f32 + rng.f32()) / width as f32;
                    let v = (j as f32 + rng.f32()) / height as f32;
                    let r = camera.get_ray(u, v);
                    let c = self.get_ray_color(&r, world, sky, lights, max_depth);
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

    fn get_ray_color<'a>(
        &self,
        r: &Ray,
        world: &'a dyn Hittable,
        sky: &dyn Sky,
        lights: &'a dyn Hittable,
        depth: i32,
    ) -> Color3 {
        // no more light is gathered when reach the limit
        if depth <= 0 {
            return Color3::zeros();
        }

        if let Some(rec) = world.hit(r, 0.001, f32::MAX) {
            let emitted = rec.material.emitted(&r, &rec);
            if let Some(ScatterRecord {
                specular_ray,
                attenuation,
                pdf: pdf_scatter,
            }) = rec.material.scatter(r, &rec)
            {
                if let Some(specular_ray) = specular_ray {
                    return vec3::elementwise_mult(
                        &attenuation,
                        &self.get_ray_color(&specular_ray, world, sky, lights, depth - 1),
                    );
                }

                let pdf_light = HittablePDF::new(rec.p, lights);

                let mixture_pdf: Box<dyn PDF> = match &pdf_scatter {
                    Some(pdf_scatter) => {
                        Box::new(MixturePDF::new(&pdf_light, pdf_scatter.as_ref()))
                    }
                    None => Box::new(WrapperPDF::new(&pdf_light)),
                };

                let scattered = Ray::new(rec.p, mixture_pdf.generate(), r.time());
                let pdf_val = mixture_pdf.value(&scattered.direction());

                let nc = self.get_ray_color(&scattered, world, sky, lights, depth - 1);

                return emitted
                    + vec3::elementwise_mult(&attenuation, &nc)
                        * rec.material.scattering_pdf(r, &rec, &scattered)
                        / pdf_val;
            }
            return emitted;
        }
        return sky.color(&r);
    }
}
