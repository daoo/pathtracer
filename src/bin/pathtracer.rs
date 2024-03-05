use ::pathtracer::camera::*;
use ::pathtracer::image_buffer::ImageBuffer;
use ::pathtracer::pathtracer;
use ::pathtracer::scene::*;
use ::pathtracer::wavefront::*;
use clap::Parser;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::fs;
use std::str;
use std::time::Duration;
use std::time::Instant;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, required = true)]
    input: std::path::PathBuf,
    #[arg(short, long, required = true)]
    output: std::path::PathBuf,
    #[arg(short, long, default_value_t = 512)]
    width: usize,
    #[arg(short, long, default_value_t = 512)]
    height: usize,
    #[arg(short, long, default_value_t = 10)]
    max_bounces: u32,
    #[arg(short, long, default_value_t = 16)]
    iterations: u32,
}

fn main() {
    let args = Args::parse();

    println!("Loading {}...", args.input.display());
    let obj = obj::obj(str::from_utf8(&fs::read(&args.input).unwrap()).unwrap());
    let mtl_path = args.input.parent().unwrap().join(&obj.mtl_lib);
    println!("Loading {}...", mtl_path.display());
    let mtl = mtl::mtl(str::from_utf8(&fs::read(mtl_path).unwrap()).unwrap());
    println!("Building scene...");
    let scene = Scene::from_wavefront(&obj, &mtl);
    println!("Triangles: {}", scene.triangles.len());

    let pinhole = Pinhole::new(&scene.cameras[0], args.width as f32 / args.height as f32);
    let mut buffer = ImageBuffer::new(args.width, args.height);

    let mut rng = SmallRng::from_entropy();
    println!("Rendering {} iteration(s)...", args.iterations);
    let mut time_sum = Duration::new(0, 0);
    for iteration in 0..args.iterations {
        let t1 = Instant::now();
        pathtracer::render(args.max_bounces, &scene, &pinhole, &mut buffer, &mut rng);
        let t2 = Instant::now();
        let duration = t2 - t1;
        time_sum += duration;
        println!(
            "Rendered iteration {} / {} in {:?} (mean: {:?})...",
            iteration + 1,
            args.iterations,
            duration,
            time_sum / (iteration + 1)
        );
    }

    println!("Writing {}...", args.output.display());
    buffer
        .div(args.iterations as f32)
        .save_with_format(&args.output, image::ImageFormat::Png)
        .unwrap();
}
