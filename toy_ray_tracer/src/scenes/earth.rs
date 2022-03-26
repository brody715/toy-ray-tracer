use crate::camera::{Camera, CameraOpt};
use crate::geometry::shapes::Sphere;
use crate::hittable::HittablePtr;
use crate::materials::Lambertian;
use crate::nimage;
use crate::scene::{RenderOptions, Scene};

use crate::textures::ImageTexture;
use crate::vec::{vec3, Vec3};
use std::sync::Arc;

fn create_world() -> HittablePtr {
    let img = nimage::Image::load_png("assets/earthmap.jpg").expect("failed to load earth");
    let texture = Arc::new(ImageTexture::new(img));
    let earth = Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(texture)),
    );
    Arc::new(earth)
}

pub fn create_scene(opt: RenderOptions) -> Scene {
    let camera = Camera::new(CameraOpt {
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::zeros(),
        view_up: vec3::YUP,
        vertical_fov: 20.0,
        aspect: opt.aspect(),
        aperture: 0.0,
        focus_dist: 10.0,
        time0: 0.0,
        time1: 1.0,
    });

    let world = create_world();

    return Scene::new(
        camera,
        world,
        Vec3::new(0.7, 0.8, 1.0),
        String::from("earth"),
        String::from("image texture earth"),
    );
}
