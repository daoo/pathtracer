use ::pathtracer::camera::*;
use ::pathtracer::image_buffer::ImageBuffer;
use ::pathtracer::pathtracer;
use ::pathtracer::scene::*;
use ::pathtracer::wavefront::*;
use clap::Parser;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::fs;
use std::str;
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
    #[arg(short, long, default_value_t = 1)]
    threads: u32,
}

fn work(
    thread: u32,
    width: usize,
    height: usize,
    max_bounces: u32,
    scene: &Scene,
    pinhole: &Pinhole,
    iterations: u32,
) -> ImageBuffer {
    let mut rng = SmallRng::from_entropy();
    let mut buffer = ImageBuffer::new(width, height);
    for iteration in 0..iterations {
        let t1 = Instant::now();
        pathtracer::render(max_bounces, scene, pinhole, &mut buffer, &mut rng);
        let t2 = Instant::now();
        let duration = t2 - t1;
        println!(
            "Thread {} rendered iteration {} / {} in {:?}...",
            thread,
            iteration + 1,
            iterations,
            duration
        );
    }
    buffer
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
    println!("Triangles: {}", scene.triangle_normals.len());

    let pinhole = Pinhole::new(&scene.cameras[0], args.width as f32 / args.height as f32);

    println!("Rendering {} iteration(s)...", args.iterations);

    let iterations_per_thread = args.iterations / args.threads;
    let buffers = (0..args.threads)
        .into_par_iter()
        .map(|thread| {
            work(
                thread,
                args.width,
                args.height,
                args.max_bounces,
                &scene,
                &pinhole,
                iterations_per_thread,
            )
        })
        .collect::<Vec<_>>();
    let buffer = ImageBuffer::sum(&buffers);

    println!("Writing {}...", args.output.display());
    buffer
        .div(args.iterations as f32)
        .save_with_format(&args.output, image::ImageFormat::Png)
        .unwrap();
}
