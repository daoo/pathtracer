use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    ops::Add,
    sync::mpsc::Sender,
    time::{Duration, Instant},
};

use glam::UVec2;
use kdtree::KdNode;
use rand::{rngs::SmallRng, SeedableRng};
use scene::camera::Pinhole;

use crate::{
    image_buffer::ImageBuffer,
    pathtracer::Pathtracer,
    raylogger::{RayLoggerWithIteration, RayLoggerWriter},
};

fn create_ray_logger(thread: u32) -> RayLoggerWriter {
    if cfg!(feature = "ray_logging") {
        let path = format!("./tmp/raylog{thread}.bin");
        RayLoggerWriter::create(path).unwrap()
    } else {
        RayLoggerWriter::None()
    }
}

pub fn render_iterations(
    thread: u32,
    pathtracer: &Pathtracer<KdNode>,
    camera: &Pinhole,
    size: UVec2,
    iterations: u32,
    tx: &Sender<Duration>,
) -> ImageBuffer {
    let mut rng = SmallRng::from_entropy();
    let mut buffer = ImageBuffer::new(size);
    let mut ray_logger = create_ray_logger(thread);
    for iteration in 0..iterations {
        let t1 = Instant::now();
        pathtracer.render_mut(
            camera,
            &mut ray_logger.with_iteration(iteration as u16),
            &mut rng,
            &mut buffer,
        );
        let t2 = Instant::now();
        let duration = t2 - t1;
        tx.send(duration).unwrap();
    }
    buffer
}

pub fn render_subdivided(
    pathtracer: &Pathtracer<KdNode>,
    pinhole: &Pinhole,
    sub_size: UVec2,
) -> ImageBuffer {
    let count = pinhole.size / sub_size;
    (0..count.x * count.y)
        .into_par_iter()
        .fold(
            || (SmallRng::from_entropy(), ImageBuffer::new(pinhole.size)),
            |(mut rng, mut buffer), i| {
                let pixel = UVec2::new(i % count.x * sub_size.x, i / count.x * sub_size.y);
                let mut ray_logger = RayLoggerWithIteration {
                    writer: &mut RayLoggerWriter::None(),
                    iteration: 0,
                };
                pathtracer.render_subdivided_mut(
                    pinhole,
                    &mut ray_logger,
                    &mut rng,
                    &mut buffer,
                    pixel,
                    sub_size,
                );
                (rng, buffer)
            },
        )
        .map(|(_, buffer)| buffer)
        .reduce_with(Add::add)
        .unwrap()
}
