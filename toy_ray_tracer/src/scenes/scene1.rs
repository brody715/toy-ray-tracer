use std::sync::Arc;

use crate::camera::{Camera, CameraOpt};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::materials::{Dielectric, Lambertian, Metal};
use crate::scene::{RenderOptions, Scene};
use crate::shapes::{MovingSphere, Sphere, BVH};
use crate::textures::{CheckerTexture, ConstantTexture};
use crate::utils::random;
use crate::vec::{vec3, Vec3};

fn create_world() -> Box<dyn Hittable> {
    let origin = Vec3::new(4.0, 0.2, 0.0);
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::new(
        Arc::new(ConstantTexture::new(0.2, 0.3, 0.1)),
        Arc::new(ConstantTexture::new(0.9, 0.9, 0.9)),
    ));
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(checker)),
    ));
    for a in -10..10 {
        for b in -10..10 {
            let choose_material = random::f32();
            let center = Vec3::new(
                a as f32 + 0.9 * random::f32(),
                0.2,
                b as f32 + 0.9 * random::f32(),
            );
            if (center - origin).magnitude() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    world.add(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, 0.5 * random::f32(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
                            random::f32() * random::f32(),
                            random::f32() * random::f32(),
                            random::f32() * random::f32(),
                        )))),
                    ));
                } else if choose_material < 0.95 {
                    // metal
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + random::f32()),
                                0.5 * (1.0 + random::f32()),
                                0.5 * (1.0 + random::f32()),
                            ),
                            0.5 * random::f32(),
                        )),
                    ));
                } else {
                    // glass
                    world.add(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5))));
                }
            }
        }
    }
    world.add(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
            0.4, 0.2, 0.1,
        )))),
    ));
    world.add(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    ));
    Box::new(BVH::new(world.move_list(), 0.0, 1.0))
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
        String::from("scene1"),
        String::from("random sphere"),
    );
}
