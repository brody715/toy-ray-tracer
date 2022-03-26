use crate::scene::{RenderOptions, Scene};
use anyhow::{Context, Result};
use std::{cell::RefCell, collections::HashMap, sync::Arc};

// mod cornell_box_foggy;
mod earth;
mod scene1;
// mod scene2;

type SceneFactoryFn = dyn Fn(RenderOptions) -> Scene + Send + Sync;

pub struct SceneFactory {
    producer: Box<SceneFactoryFn>,
    name: String,
}

impl SceneFactory {
    fn new(producer: Box<SceneFactoryFn>, name: &str) -> Self {
        return Self {
            producer,
            name: String::from(name),
        };
    }

    pub fn create_scene(&self, opt: RenderOptions) -> Scene {
        return (self.producer)(opt);
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }
}

type SceneRegistry = HashMap<String, Arc<SceneFactory>>;

macro_rules! register_scene {
    ($registry: ident, $scene_name: ident) => {
        $registry.insert(
            String::from(stringify!($scene_name)),
            Arc::new(SceneFactory::new(
                Box::new($scene_name::create_scene),
                stringify!($scene_name),
            )),
        );
    };
}

thread_local! {
    static SCENE_REGISTRY : RefCell<Option<Arc<SceneRegistry>>> = RefCell::new(None)
}

fn get_scene_registry() -> Arc<SceneRegistry> {
    SCENE_REGISTRY.with(|cell| {
        let mut sro = cell.borrow_mut();
        if sro.is_none() {
            // register all scenes if not registered
            let mut sr = SceneRegistry::new();
            register_scene!(sr, scene1);
            // register_scene!(sr, cornell_box_foggy);
            register_scene!(sr, earth);
            // register_scene!(sr, scene2);
            *sro = Some(Arc::new(sr));
        }
        sro.as_ref().unwrap().clone()
    })
}

pub fn get_scene_factory(name: &str) -> Result<Arc<SceneFactory>> {
    let sr = get_scene_registry();
    sr.as_ref()
        .get(name)
        .map(|v| v.clone())
        .context(format!("no such scene: {}", name))
}
