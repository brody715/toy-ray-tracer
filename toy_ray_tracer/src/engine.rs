use std::{cell::RefCell, sync::Arc};

use crate::utils::random;
use log::trace;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use thread_local::ThreadLocal;

use crate::{
    hittable::Hittable,
    nimage::Image,
    ray::Ray,
    scene::{RenderOptions, Scene},
    utils::ExecutionTimer,
    vec::{vec3, Color3, Vec3List},
};

pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        return Engine {};
    }

    pub fn render(&self, scene: &Scene, opts: RenderOptions) -> Image {
        let width = opts.width;
        let height = opts.height;
        let nsamples = opts.nsamples;
        let max_depth = opts.max_depth;

        let world = scene.world();
        let camera = scene.camera();
        let background = scene.background();

        let pixels_local = Arc::new(ThreadLocal::new());
        (0..nsamples).into_par_iter().for_each(|s: i32| {
            let _timer = ExecutionTimer::new(|start_time| {
                trace!(
                    "render elapsed {} ms, sample_idx={}",
                    start_time.elapsed().as_millis(),
                    s
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
                    let c = self.get_ray_color(&r, world, &background, max_depth);
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

    fn get_ray_color(
        &self,
        r: &Ray,
        world: &dyn Hittable,
        background: &Color3,
        depth: i32,
    ) -> Color3 {
        // no more light is gathered when reach the limit
        if depth <= 0 {
            return Color3::zeros();
        }

        if let Some(rec) = world.hit(r, 0.001, f32::MAX) {
            let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
            if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec) {
                let nc = self.get_ray_color(&scattered, world, background, depth - 1);
                return emitted + vec3::elementwise_mult(&attenuation, &nc);
            }
            return emitted;
        }
        return *background;
    }
}
