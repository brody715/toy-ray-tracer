mod aabb;
mod camera;
mod engine;
mod environment;
mod geometry;
mod hittable;
mod hittable_list;
mod material;
mod materials;
mod nimage;
mod perlin;
mod project;
mod ray;
mod scene;
mod scene_builder;
mod scenes;
mod texture;
mod textures;
mod utils;
mod vec;

use crate::scene::RenderOptions;
use crate::scene_builder::{load_project_config, Buildable};
use crate::{engine::Engine, utils::ExecutionTimer};
use anyhow::Ok;
use clap::{Args, Parser, Subcommand};
use log::{debug, info};
use scene_builder::ProjectConfig;
use schemars::schema_for;
use std::path::Path;

#[derive(Args, Debug)]
struct RenderCmdArgs {
    #[clap(long, help = "scene to render", default_value_t = String::from("earth"))]
    scene: String,

    #[clap(long, short = 'o', help = "output dir", default_value_t = String::from("./output"))]
    output_dir: String,

    #[clap(flatten)]
    render_opts: RenderOptions,

    #[clap(long, short = 'p', help = "project file")]
    project_file: String,
}

#[derive(Args, Debug)]
struct GenerateCmdArgs {}

#[derive(Subcommand)]
enum Commands {
    Render(RenderCmdArgs),
    Generate(GenerateCmdArgs),
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder
        .filter_level(cli.verbose.log_level_filter())
        .init();

    match cli.command {
        Commands::Render(args) => run_render(args),
        Commands::Generate(args) => run_generate(args),
    }
}

fn run_generate(_args: GenerateCmdArgs) -> anyhow::Result<()> {
    let schema = schema_for!(ProjectConfig);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    Ok(())
}

fn run_render(args: RenderCmdArgs) -> anyhow::Result<()> {
    debug!("use args={:#?}", args);

    let project_config: ProjectConfig = load_project_config(&args.project_file)?;

    let project = project_config.build();

    let opt = project.settings();

    let engine = Engine::new();

    let output_dir = Path::new(&args.output_dir);
    let output_path = output_dir.join(format!("{}.png", project.name()));

    {
        let _timer = ExecutionTimer::new(|start_time| {
            info!(
                "rendering finished, elapsed {:.2} s",
                start_time.elapsed().as_secs_f32()
            )
        });

        info!(
            "start to render scene={} size={}x{} nsamples={}",
            project.name(),
            opt.width,
            opt.height,
            opt.nsamples
        );
        let img = engine.render(&project);
        img.save_to_png(&output_path)?;
    }

    info!(
        "rendered image has been written to {}",
        output_path.display().to_string(),
    );

    Ok(())
}
