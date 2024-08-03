use clap::Parser;
use kdtree::{build::build_kdtree, sah::SahCost, MAX_DEPTH};
use scene::Scene;
use stage::Stage;
use tracing::pathtracer::Pathtracer;

mod stage;
mod worker;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    /// Maximum kd-tree depth
    #[arg(long, default_value_t = MAX_DEPTH as u32)]
    max_depth: u32,
    /// SAH kd-tree traverse cost
    #[arg(long, default_value_t = SahCost::default().traverse_cost)]
    traverse_cost: f32,
    /// SAH kd-tree intersect cost
    #[arg(long, default_value_t = SahCost::default().intersect_cost)]
    intersect_cost: f32,
    /// SAH kd-tree empty factor
    #[arg(long, default_value_t = SahCost::default().empty_factor)]
    empty_factor: f32,
}

fn main() {
    let args = Args::parse();
    let scene = Scene::read_obj_file_with_print_logging(&args.input);

    println!("Building kdtree...");
    let kdtree = build_kdtree(
        scene.geometries.clone(),
        args.max_depth,
        &SahCost {
            traverse_cost: args.traverse_cost,
            intersect_cost: args.intersect_cost,
            empty_factor: args.empty_factor,
        },
    );

    let pathtracer = Pathtracer {
        max_bounces: 16,
        scene,
        kdtree,
    };

    miniquad::start(Default::default(), move || {
        let camera = pathtracer.scene.cameras[0].clone();
        Box::new(Stage::new(pathtracer, camera))
    });
}
