use kdtree::{build::build_kdtree, sah::SahCost};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use scene::Scene;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};
use tracing::{camera::Pinhole, light::SphericalLight, material::Material};
use wavefront::read_obj_and_mtl_with_print_logging;

use crate::{ray_bouncer::RayBouncer, size::Size};

pub(crate) fn kdtree_ray_tester(
    input: PathBuf,
    output: Option<PathBuf>,
    size: Size,
    bounces: u32,
    sah: SahCost,
) {
    let (obj, mtl, mtl_path) = read_obj_and_mtl_with_print_logging(&input).unwrap();
    let scene = Scene::build_with_print_logging(&obj, &mtl);

    println!("Building kdtree...");
    let kdtree = build_kdtree(scene.geometries(), &sah);

    println!("Testing up to {} rays...", size.x * size.y * bounces);
    let camera = Pinhole::new(mtl.cameras[0].clone().into(), size.as_uvec2());
    let image_directory = mtl_path.parent().unwrap();
    let bouncer = RayBouncer {
        scene,
        materials: mtl
            .materials
            .iter()
            .map(|m| Material::load_from_mtl(image_directory, m))
            .collect(),
        lights: mtl.lights.iter().map(SphericalLight::from).collect(),
        kdtree,
        camera,
        size: size.as_uvec2(),
        bounces,
    };

    let xs = 0..size.x;
    let ys = 0..size.y;
    let pixels = ys
        .flat_map(|y| xs.clone().map(move |x| (x, y)))
        .collect::<Vec<_>>();
    let pixel_count = pixels.len();
    let fails = pixels
        .into_par_iter()
        .enumerate()
        .filter_map(|(i, pixel)| {
            let result = bouncer.bounce_pixel(pixel);
            if let Some(fail) = &result {
                eprintln!(
                    "Fail on pixel {} x {} ({} / {})",
                    pixel.0, pixel.1, i, pixel_count
                );
                eprintln!("  {:?}", fail.ray);
                eprintln!("  Expected: {:?}", fail.reference);
                eprintln!("    Actual: {:?}", fail.kdtree);
            }
            result
        })
        .collect::<Vec<_>>();
    println!("Found {} fails", fails.len());

    if let Some(path) = output {
        println!("Writing failed rays to {path:?}...");
        let mut logger = BufWriter::new(File::create(path).unwrap());
        fails.iter().enumerate().for_each(|(i, fail)| {
            logger.write_all(&fail.as_bytes(i as u16)).unwrap();
        });
    }
}
