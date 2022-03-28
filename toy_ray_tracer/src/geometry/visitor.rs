use crate::{hittable::NopHittable, hittable_list::HittableList};
use paste::paste;

use super::{
    containers::{TagsHittable, BVH},
    shapes::{AARect, Cube, MovingSphere, NopLight, Rect, Sphere},
    transforms::{FlipFace, Rotate, Translate},
    volumes::ConstantMedium,
};

macro_rules! make_visitor_listener {
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

make_visitor_listener!(Geometry {
    HittableList,
    NopHittable,
    ConstantMedium,
    Rotate,
    FlipFace,
    Cube,
    Rect,
    AARect,
    Sphere,
    MovingSphere,
    NopLight,
    Translate,
    BVH,
    TagsHittable
});

impl<'a, T> EnterContext<'a, T> {
    pub fn new(node: &'a T) -> Self {
        Self { node }
    }
}
