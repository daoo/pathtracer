use std::{
    ops::Add,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::JoinHandle,
    time,
};

use nalgebra::Vector2;
use rand::{rngs::SmallRng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use scene::camera::Pinhole;
use tracing::{
    image_buffer::ImageBuffer,
    pathtracer::Pathtracer,
    raylogger::{RayLoggerWithIteration, RayLoggerWriter},
};

pub struct RenderResult {
    pub iteration: u16,
    pub duration: time::Duration,
    pub image: ImageBuffer,
}

fn worker_loop(
    pathtracer: Arc<Pathtracer>,
    start_pinhole: Pinhole,
    rx: Receiver<Pinhole>,
    tx: Sender<RenderResult>,
) {
    let mut pinhole = start_pinhole;
    let mut iteration = 0;
    let mut combined_buffer = ImageBuffer::new(pinhole.width, pinhole.height);
    loop {
        loop {
            match rx.try_recv() {
                Ok(new_pinhole) => {
                    eprintln!("Resetting buffer {new_pinhole:?}");
                    pinhole = new_pinhole;
                    combined_buffer = ImageBuffer::new(pinhole.width, pinhole.height);
                    iteration = 0;
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => return,
            }
        }
        eprintln!(
            "Rendering {}x{} iteration {}",
            pinhole.width, pinhole.height, iteration
        );
        let t1 = time::Instant::now();
        let sub_width = 128;
        let sub_height = 128;
        let xs = 0..(pinhole.width / sub_width);
        let ys = 0..(pinhole.height / sub_height);
        let pixels = ys
            .flat_map(|y| {
                xs.clone()
                    .map(move |x| Vector2::new(x * sub_width, y * sub_height))
            })
            .collect::<Vec<_>>();
        let resulting_buffer = pixels
            .into_par_iter()
            .fold(
                || {
                    (
                        SmallRng::from_entropy(),
                        ImageBuffer::new(pinhole.width, pinhole.height),
                    )
                },
                |(mut rng, mut buffer), pixel| {
                    let mut ray_logger = RayLoggerWithIteration {
                        writer: &mut RayLoggerWriter::None(),
                        iteration,
                    };
                    pathtracer.render_subdivided_mut(
                        &pinhole,
                        &mut ray_logger,
                        &mut rng,
                        &mut buffer,
                        pixel,
                        Vector2::new(sub_width, sub_height),
                    );
                    (rng, buffer)
                },
            )
            .map(|(_, buffer)| buffer)
            .reduce_with(Add::add)
            .unwrap();
        combined_buffer.add_mut(&resulting_buffer);
        let t2 = time::Instant::now();
        let duration = t2 - t1;
        iteration += 1;
        let _ = tx.send(RenderResult {
            iteration,
            duration,
            image: combined_buffer.clone(),
        });
    }
}

pub fn spawn_worker(
    pathtracer: Pathtracer,
    pinhole: Pinhole,
) -> (JoinHandle<()>, Sender<Pinhole>, Receiver<RenderResult>) {
    let (render_result_tx, render_result_rx) = mpsc::channel::<RenderResult>();
    let (render_settings_tx, render_settings_rx) = mpsc::channel::<Pinhole>();
    let thread = std::thread::Builder::new()
        .name("Pathtracer Worker".to_string())
        .spawn(move || {
            worker_loop(
                Arc::new(pathtracer),
                pinhole,
                render_settings_rx,
                render_result_tx,
            )
        })
        .unwrap();

    (thread, render_settings_tx, render_result_rx)
}
