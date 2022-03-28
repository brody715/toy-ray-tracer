use std::sync::Arc;

use crate::{
    geometry::{containers::TagsHittable, EnterContext, GeometryWalker},
    hittable::{Hittable, HittablePtr},
    hittable_list::HittableList,
};

pub fn try_get_light_from_world<'a>(world: &'a dyn Hittable) -> Option<HittablePtr> {
    // iterate and try extract with tags

    struct Walker {
        results: Vec<HittablePtr>,
    }

    impl GeometryWalker for Walker {
        fn enter_tags_hittable(&mut self, ctx: EnterContext<TagsHittable>) {
            let tags = &ctx.node.tags;

            if tags.contains("lights") || tags.contains("light") {
                self.results.push(ctx.node.child.clone());
            }
        }
    }

    let mut walker = Walker {
        results: Vec::new(),
    };

    world.walk(&mut walker);

    if walker.results.len() == 0 {
        return None;
    }

    let lights = Arc::new(HittableList::from(walker.results));
    return Some(lights);
}
