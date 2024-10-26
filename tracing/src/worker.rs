use image::RgbImage;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{ops::Add, sync::mpsc::Sender, thread};
use time::Duration;

use glam::UVec2;
use kdtree::{IntersectionAccelerator, KdNode};
use rand::{rngs::SmallRng, SeedableRng};

use crate::{
    camera::Pinhole,
    image_buffer::ImageBuffer,
    measure,
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

fn render_iterations<Accelerator>(
    thread: u32,
    pathtracer: &Pathtracer<Accelerator>,
    camera: &Pinhole,
    size: UVec2,
    iterations: u32,
    tx: &Sender<Duration>,
) -> ImageBuffer
where
    Accelerator: IntersectionAccelerator,
{
    let mut rng = SmallRng::from_entropy();
    let mut buffer = ImageBuffer::new(size);
    let mut ray_logger = create_ray_logger(thread);
    for iteration in 0..iterations {
        let (duration, _) = measure::measure(|| {
            pathtracer.render_mut(
                camera,
                &mut ray_logger.with_iteration(iteration as u16),
                &mut rng,
                &mut buffer,
            )
        });
        tx.send(duration).unwrap();
    }
    buffer
}

pub fn render_parallel_subdivided(
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

pub fn render_parallel_iterations<Accelerator>(
    pathtracer: &Pathtracer<Accelerator>,
    camera: Pinhole,
    size: UVec2,
    threads: u32,
    iterations_per_thread: u32,
    tx: Sender<Duration>,
) -> (Duration, RgbImage)
where
    Accelerator: IntersectionAccelerator + Send + Sync,
{
    let total_iterations = threads * iterations_per_thread;
    thread::scope(|s| {
        let (duration, buffer) = measure::measure(|| {
            let threads = (0..threads)
                .map(|i| {
                    let tx = &tx;
                    let pathtracer = &pathtracer;
                    let camera = &camera;
                    s.spawn(move || {
                        render_iterations(i, pathtracer, camera, size, iterations_per_thread, tx)
                    })
                })
                .collect::<Vec<_>>();

            let buffers = threads.into_iter().map(|t| t.join().unwrap());
            buffers.reduce(Add::add).unwrap()
        });

        let image = RgbImage::from_raw(
            buffer.size.x,
            buffer.size.y,
            buffer.to_rgb8(total_iterations as u16),
        );
        (duration, image.unwrap())
    })
}
