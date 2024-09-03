use kdtree::{build::build_kdtree, sah::SahCost};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use scene::{camera::Pinhole, Scene};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::{ray_bouncer::RayBouncer, size::Size};

pub(crate) fn kdtree_ray_tester(
    input: PathBuf,
    output: Option<PathBuf>,
    size: Size,
    bounces: u32,
    sah: SahCost,
) {
    let scene = Scene::read_obj_file_with_print_logging(&input);

    println!("Building kdtree...");
    let kdtree = build_kdtree(scene.geometries(), &sah);

    println!("Testing up to {} rays...", size.x * size.y * bounces);
    let camera = Pinhole::new(scene.cameras()[0].clone(), size.as_uvec2());
    let bouncer = RayBouncer {
        scene,
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
