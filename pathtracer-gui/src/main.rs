use clap::Parser;
use kdtree::{build::build_kdtree, build_sah::SahKdTreeBuilder};
use scene::{camera::Pinhole, Scene};
use stage::Stage;
use tracing::pathtracer::Pathtracer;

use crate::worker::Worker;

mod stage;
mod worker;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    #[arg(long, default_value_t = 20)]
    max_depth: u32,
    #[arg(long, default_value_t = 2.0)]
    traverse_cost: f32,
    #[arg(long, default_value_t = 1.0)]
    intersect_cost: f32,
    #[arg(long, default_value_t = 0.8)]
    empty_factor: f32,
}

fn main() {
    let args = Args::parse();
    let scene = Scene::read_obj_file_with_print_logging(&args.input);

    println!("Building kdtree...");
    let kdtree = build_kdtree(
        SahKdTreeBuilder {
            traverse_cost: args.traverse_cost,
            intersect_cost: args.intersect_cost,
            empty_factor: args.empty_factor,
            geometries: scene
                .triangle_data
                .iter()
                .map(|t| t.triangle.into())
                .collect(),
        },
        args.max_depth,
    );

    let pathtracer = Pathtracer {
        max_bounces: 16,
        scene,
        kdtree,
    };

    let camera = pathtracer.scene.cameras[0].clone();
    let pinhole = Pinhole::new(camera.clone(), 128, 128);
    let worker = Worker::spawn(pathtracer, pinhole);

    miniquad::start(Default::default(), move || {
        Box::new(Stage::new(worker, camera))
    });
}
