use std::{
    ops::Add,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::JoinHandle,
    time,
};

use glam::UVec2;
use rand::{rngs::SmallRng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use scene::camera::Pinhole;
use tracing::{
    image_buffer::ImageBuffer,
    measure::measure,
    pathtracer::Pathtracer,
    raylogger::{RayLoggerWithIteration, RayLoggerWriter},
};

pub struct RenderResult {
    pub iterations: u16,
    pub duration: time::Duration,
    pub buffer: ImageBuffer,
}

impl RenderResult {
    fn new(iterations: u16, duration: time::Duration, buffer: ImageBuffer) -> Self {
        RenderResult {
            iterations,
            duration,
            buffer,
        }
    }
}

fn render_subdivided(pathtracer: &Pathtracer, pinhole: &Pinhole, sub_size: UVec2) -> ImageBuffer {
    let count_x = pinhole.width / sub_size.x;
    let count_y = pinhole.height / sub_size.y;
    (0..count_x * count_y)
        .into_par_iter()
        .fold(
            || {
                (
                    SmallRng::from_entropy(),
                    ImageBuffer::new(pinhole.width, pinhole.height),
                )
            },
            |(mut rng, mut buffer), i| {
                let pixel = UVec2::new(i % count_x * sub_size.x, i / count_x * sub_size.y);
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
                    eprintln!(
                        "New pinhole position={:?} direction={:?} size={:?}",
                        new_pinhole.camera.position,
                        new_pinhole.camera.direction,
                        new_pinhole.size(),
                    );
                    pinhole = new_pinhole;
                    combined_buffer = ImageBuffer::new(pinhole.width, pinhole.height);
                    iteration = 0;
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => return,
            }
        }
        if iteration == 0 {
            let pinhole_small = Pinhole::new(
                pinhole.camera.clone(),
                64,
                64 * pinhole.height / pinhole.width,
            );
            eprintln!("Rendering scaled down size={:?}", pinhole_small.size());
            let (duration, buffer) = measure(|| {
                render_subdivided(&pathtracer, &pinhole_small, pinhole_small.size() / 3)
            });
            let _ = tx.send(RenderResult::new(1, duration, buffer));
        }

        eprintln!("Rendering size={:?}", pinhole.size());
        let (duration, buffer) =
            measure(|| render_subdivided(&pathtracer, &pinhole, pinhole.size() / 4));
        combined_buffer += buffer;
        iteration += 1;
        let _ = tx.send(RenderResult::new(
            iteration,
            duration,
            combined_buffer.clone(),
        ));
    }
}

pub(crate) struct Worker {
    thread: JoinHandle<()>,
    pinhole_tx: Sender<Pinhole>,
    result_rx: Receiver<RenderResult>,
}

impl Worker {
    pub fn spawn(pathtracer: Pathtracer, pinhole: Pinhole) -> Self {
        let (result_tx, result_rx) = mpsc::channel::<RenderResult>();
        let (pinhole_tx, pinhole_rx) = mpsc::channel::<Pinhole>();
        let thread = std::thread::Builder::new()
            .name("Pathtracer Worker".to_string())
            .spawn(move || worker_loop(Arc::new(pathtracer), pinhole, pinhole_rx, result_tx))
            .unwrap();
        Self {
            thread,
            pinhole_tx,
            result_rx,
        }
    }

    pub fn send(&self, pinhole: Pinhole) {
        self.pinhole_tx.send(pinhole).unwrap();
    }

    pub fn try_receive(&self) -> Option<RenderResult> {
        let mut previous = self.result_rx.try_recv();
        while previous.is_ok() {
            let result = self.result_rx.try_recv();
            if result.is_ok() {
                previous = result;
            } else {
                return previous.ok();
            }
        }
        None
    }

    pub fn join(self) {
        drop(self.pinhole_tx);
        self.thread.join().unwrap();
    }
}
