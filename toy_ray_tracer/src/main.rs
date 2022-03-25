mod aabb;
mod camera;
mod engine;
mod hittable;
mod hittable_list;
mod material;
mod materials;
mod nimage;
mod perlin;
mod ray;
mod scene;
mod scenes;
mod shapes;
mod texture;
mod textures;
mod transforms;
mod utils;
mod vec;

use crate::scene::RenderOptions;
use crate::{engine::Engine, utils::ExecutionTimer};
use clap::Parser;
use log::{debug, info};
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(long, help = "scene to render", default_value_t = String::from("earth"))]
    scene: String,

    #[clap(long, short = 'o', help = "output dir", default_value_t = String::from("./output"))]
    output_dir: String,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[clap(flatten)]
    render_opts: RenderOptions,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    {
        let mut log_builder = pretty_env_logger::formatted_builder();
        log_builder
            .filter_level(args.verbose.log_level_filter())
            .init();
    }

    debug!("use args={:#?}", args);

    let opt = args.render_opts;

    let scene_factory = scenes::get_scene_factory(&args.scene)?;
    let scene = scene_factory(opt);

    let engine = Engine::new();

    let output_dir = Path::new(&args.output_dir);
    let output_path = output_dir.join(format!("{}.png", scene.name()));

    {
        let _timer = ExecutionTimer::new(|start_time| {
            info!(
                "rendering finished, elapsed {:.2} s",
                start_time.elapsed().as_secs_f32()
            )
        });

        info!(
            "start to render scene={} size={}x{} nsamples={}",
            scene.name(),
            opt.width,
            opt.height,
            opt.nsamples
        );
        let img = engine.render(&scene, opt);
        img.save_to_png(&output_path)?;
    }

    info!(
        "rendered image has been written to {}",
        output_path.display().to_string(),
    );

    Ok(())
}
