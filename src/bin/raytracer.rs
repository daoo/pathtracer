extern crate pathtracer;

use clap::Parser;
use nalgebra::{DMatrix, Vector3};
use pathtracer::camera::*;
use pathtracer::raytracer;
use pathtracer::scene::*;
use pathtracer::wavefront::*;
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
    let mut buffer: DMatrix<Vector3<f32>> = DMatrix::zeros(args.width, args.height);

    println!("Rendering...");
    raytracer::render(&scene, &pinhole, &mut buffer);

    println!("Writing {}...", args.output.display());
    let mut image = image::RgbImage::new(args.width as u32, args.height as u32);
    for y in 0..args.height {
        for x in 0..args.width {
            let c = buffer[(x, y)] * 255.0;
            image[(x as u32, y as u32)] = image::Rgb([c.x as u8, c.y as u8, c.z as u8]);
        }
    }
    image.save_with_format(args.output, image::ImageFormat::Png).unwrap();
}
