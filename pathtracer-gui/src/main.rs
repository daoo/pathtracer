use clap::Parser;
use geometry::geometry::from_wavefront;
use glam::Vec3;
use kdtree::{build::build_kdtree, sah::SahCost};
use miniquad::conf::Conf;
use stage::Stage;
use tracing::{light::Light, material::Material, pathtracer::Pathtracer};
use wavefront::read_obj_and_mtl_with_print_logging;

mod stage;
mod worker;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

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
    let (obj, mtl, mtl_path) = read_obj_and_mtl_with_print_logging(&args.input).unwrap();
    let (geometries, properties) = from_wavefront(&obj, &mtl);

    println!("Building kdtree...");
    let accelerator = build_kdtree(
        &geometries,
        &SahCost {
            traverse_cost: args.traverse_cost,
            intersect_cost: args.intersect_cost,
            empty_factor: args.empty_factor,
        },
    );

    let image_directory = mtl_path.parent().unwrap();
    let materials = mtl
        .materials
        .iter()
        .map(|m| Material::load_from_mtl(image_directory, m))
        .collect();
    let lights = mtl.lights.iter().map(Light::from).collect();
    let pathtracer = Pathtracer {
        max_bounces: 16,
        geometries,
        properties,
        materials,
        lights,
        environment: Vec3::new(0.8, 0.8, 0.8),
        accelerator,
    };

    miniquad::start(Conf::default(), move || {
        Box::new(Stage::new(pathtracer, mtl.cameras[0].clone().into()))
    });
}
