use crate::scene::{RenderOptions, Scene};
use anyhow::{Context, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod cornell_box_foggy;
mod earth;
mod scene1;
mod scene2;

type SceneFactory = dyn Fn(RenderOptions) -> Scene;
type SceneRegistry = HashMap<String, Rc<SceneFactory>>;

macro_rules! register_scene {
    ($registry: ident, $scene_name: ident) => {
        $registry.insert(
            String::from(stringify!($scene_name)),
            Rc::new($scene_name::create_scene),
        );
    };
}

thread_local! {
    static SCENE_REGISTRY : RefCell<Option<Rc<SceneRegistry>>> = RefCell::new(None)
}

fn get_scene_registry() -> Rc<SceneRegistry> {
    SCENE_REGISTRY.with(|cell| {
        let mut sro = cell.borrow_mut();
        if sro.is_none() {
            // register all scenes if not registered
            let mut sr = SceneRegistry::new();
            register_scene!(sr, scene1);
            register_scene!(sr, cornell_box_foggy);
            register_scene!(sr, earth);
            register_scene!(sr, scene2);
            *sro = Some(Rc::new(sr));
        }
        sro.as_ref().unwrap().clone()
    })
}

pub fn get_scene_factory(name: &str) -> Result<Rc<SceneFactory>> {
    let sr = get_scene_registry();
    sr.as_ref()
        .get(name)
        .map(|v| v.clone())
        .context(format!("no such scene: {}", name))
}
