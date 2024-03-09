use ::pathtracer::camera::*;
use ::pathtracer::geometry::triangle::Triangle;
use ::pathtracer::image_buffer::ImageBuffer;
use ::pathtracer::kdtree::build::build_kdtree_median;
use ::pathtracer::light::SphericalLight;
use ::pathtracer::material::*;
use ::pathtracer::pathtracer;
use ::pathtracer::scene::*;
use ::pathtracer::wavefront::*;
use clap::Parser;
use nalgebra::vector;
use nalgebra::UnitVector3;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::fs;
use std::str;
use std::sync::Arc;
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

fn test_scene() -> Scene {
    let triangles = vec![Triangle {
        v0: vector![0.0, 0.0, 0.0],
        v1: vector![1.0, 0.0, 0.0],
        v2: vector![1.0, 1.0, 0.0],
    }];
    let refraction = SpecularRefractiveMaterial {
        index_of_refraction: 1.55,
    };
    let reflection = DiffuseReflectiveMaterial {
        reflectance: vector![1.0, 0.0, 0.0],
    };
    let transparency_blend = BlendMaterial {
        first: refraction,
        second: reflection,
        factor: 1.0,
    };
    let specular = SpecularReflectiveMaterial {
        reflectance: vector![1.0, 0.0, 0.0],
    };
    let fresnel_blend = FresnelBlendMaterial {
        reflection: specular,
        refraction: transparency_blend.clone(),
        r0: 0.3,
    };
    let blend = BlendMaterial {
        first: fresnel_blend,
        second: transparency_blend,
        factor: 1.0,
    };
    let material = blend;
    let kdtree = build_kdtree_median(1, triangles);
    Scene {
        kdtree,
        triangle_normals: vec![TriangleNormals {
            n0: vector![0.0, 0.0, -1.0],
            n1: vector![0.0, 0.0, -1.0],
            n2: vector![0.0, 0.0, -1.0],
        }],
        triangle_texcoords: vec![TriangleTexcoords {
            uv0: vector![0.0, 0.0],
            uv1: vector![1.0, 0.0],
            uv2: vector![0.0, 1.0],
        }],
        triangle_materials: vec![Arc::new(material)],
        cameras: vec![Camera {
            position: vector![0.0, 0.0, -2.0],
            direction: UnitVector3::new_normalize(vector![0.0, 0.0, 1.0]),
            up: UnitVector3::new_normalize(vector![0.0, 1.0, 0.0]),
            right: UnitVector3::new_normalize(vector![1.0, 0.0, 0.0]),
            fov_degrees: 45.0,
        }],
        lights: vec![SphericalLight {
            center: vector![0.0, 4.0, -4.0],
            intensity: vector![1.0, 1.0, 1.0],
            radius: 1.0,
        }],
    }
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
    // let scene = test_scene();
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
