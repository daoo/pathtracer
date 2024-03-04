use clap::Parser;
use ::pathtracer::camera::*;
use ::pathtracer::image_buffer::*;
use ::pathtracer::raytracer;
use ::pathtracer::scene::*;
use ::pathtracer::wavefront::*;
use std::fs;
use std::str;

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

    println!("Rendering...");
    raytracer::render(&scene, &pinhole, &mut buffer);

    println!("Writing {}...", args.output.display());
    buffer.save_with_format(&args.output, image::ImageFormat::Png).unwrap();
}
