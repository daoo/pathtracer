use clap::Parser;
use pathtracer::{camera::Pinhole, image_buffer::ImageBuffer, scene::Scene};
use std::{fs, str};
use wavefront::{mtl, obj};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, required = true)]
    input: std::path::PathBuf,

    #[arg(short, long, required = true)]
    output: std::path::PathBuf,
    #[arg(short, long, default_value_t = 512)]
    width: u32,
    #[arg(short, long, default_value_t = 512)]
    height: u32,

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

    println!("Loading {}...", args.input.display());
    let obj = obj::obj(str::from_utf8(&fs::read(&args.input).unwrap()).unwrap());
    let mtl_path = args.input.parent().unwrap().join(&obj.mtl_lib);
    println!("Loading {}...", mtl_path.display());
    let mtl = mtl::mtl(str::from_utf8(&fs::read(mtl_path).unwrap()).unwrap());
    println!("Building scene...");
    let scene = Scene::from_wavefront(
        &obj,
        &mtl,
        args.max_depth,
        args.traverse_cost,
        args.intersect_cost,
        args.empty_factor,
    );
    println!("Triangles: {}", scene.triangle_normals.len());

    let pinhole = Pinhole::new(&scene.cameras[0], args.width as f32 / args.height as f32);
    let mut buffer = ImageBuffer::new(args.width, args.height);

    println!("Rendering...");
    pathtracer::raytracer::render(&scene, &pinhole, &mut buffer);

    println!("Writing {}...", args.output.display());
    buffer.gamma_correct().save_png(&args.output).unwrap();
}
