extern crate pathtracer;

use nalgebra::{DMatrix, Vector3};
use pathtracer::camera::*;
use pathtracer::raytracer;
use pathtracer::scene::*;
use pathtracer::wavefront::*;
use std::env;
use std::fs;
use std::path::Path;
use std::str;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let obj_path = Path::new(&args[1]);
    let out_path = Path::new(&args[2]);

    println!("Loading {}...", obj_path.display());
    let obj = obj::obj(str::from_utf8(&fs::read(obj_path).unwrap()).unwrap());
    let mtl_path = obj_path.parent().unwrap().join(&obj.mtl_lib);
    println!("Loading {}...", mtl_path.display());
    let mtl = mtl::mtl(str::from_utf8(&fs::read(mtl_path).unwrap()).unwrap());
    println!("Building scene...");
    let scene = Scene::from_wavefront(&obj, &mtl);
    println!("Triangles: {}", scene.triangles.len());

    let width: u32 = 128;
    let height: u32 = 128;
    let pinhole = Pinhole::new(&scene.cameras[0], width as f32 / height as f32);
    let mut buffer: DMatrix<Vector3<f32>> = DMatrix::zeros(width as usize, height as usize);

    println!("Rendering...");
    raytracer::render(&scene, &pinhole, &mut buffer);

    println!("Writing {}...", out_path.display());
    let mut image = image::RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let c = buffer[(x as usize, y as usize)] * 255.0;
            image[(x, y)] = image::Rgb([c.x as u8, c.y as u8, c.z as u8]);
        }
    }
    image.save_with_format(out_path, image::ImageFormat::Png).unwrap();
}
