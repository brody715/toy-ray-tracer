use crate::{hittable::NopHittable, hittable_list::HittableList};
use paste::paste;

use super::{
    containers::{TagsHittable, BVH},
    shapes::{
        AARect, Cube, Cylinder, Disk, Mesh, MovingSphere, Pyramid, Rect, SkyLight, Sphere, Triangle,
    },
    transforms::{FlipFace, Rotate, Transformed, Translate},
    volumes::ConstantMedium,
};

macro_rules! make_visitor_walker {
    ($name: ident { $($node:ident),* }) => {
        paste! {
            pub trait [<$name Visitor>] {
                $(
                    fn [<visit_ $node:snake>](&mut self, _n: &$node) {}
                )*
            }

            pub trait [<$name Walker>] {
                $(
                    fn [<enter_ $node:snake>](&mut self, _ctx: EnterContext<$node>) {}
                )*
            }
        }
    };
}

pub struct EnterContext<'a, T> {
    pub node: &'a T,
}

make_visitor_walker!(Geometry {
    HittableList,
    NopHittable,
    ConstantMedium,
    Rotate,
    Transformed,
    FlipFace,
    Cube,
    Rect,
    AARect,
    Sphere,
    MovingSphere,
    SkyLight,
    Translate,
    BVH,
    TagsHittable,
    Disk,
    Cylinder,
    Triangle,
    Mesh,
    Pyramid
});

impl<'a, T> EnterContext<'a, T> {
    pub fn new(node: &'a T) -> Self {
        Self { node }
    }
}
