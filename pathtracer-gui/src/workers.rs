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
    measure::measure,
    pathtracer::Pathtracer,
    raylogger::{RayLoggerWithIteration, RayLoggerWriter},
};

pub struct RenderResult {
    pub iterations: u16,
    pub duration: time::Duration,
    pub image: egui::ColorImage,
}

impl RenderResult {
    fn from_buffer(iterations: u16, duration: time::Duration, buffer: ImageBuffer) -> Self {
        let size = [buffer.width as usize, buffer.height as usize];
        let pixels = buffer
            .into_rgba_iter(iterations)
            .map(|p| egui::Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
            .collect();
        let image = egui::ColorImage { size, pixels };
        RenderResult {
            iterations,
            duration,
            image,
        }
    }
}

fn render_subdivided(
    pathtracer: &Pathtracer,
    pinhole: &Pinhole,
    sub_size: Vector2<u32>,
) -> ImageBuffer {
    let count_x = pinhole.width / sub_size.x;
    let count_y = pinhole.height / sub_size.y;
    eprintln!(
        "Rendering size={:?} sub_size={:?} count={:?}",
        pinhole.size().as_slice(),
        sub_size.as_slice(),
        [count_x, count_y]
    );
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
                let pixel = Vector2::new(i % count_x * sub_size.x, i / count_x * sub_size.y);
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
                        new_pinhole.camera.position.as_slice(),
                        new_pinhole.camera.direction.as_slice(),
                        new_pinhole.size().as_slice(),
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
            let (duration, buffer) = measure(|| {
                render_subdivided(&pathtracer, &pinhole_small, pinhole_small.size() / 3)
            });
            let _ = tx.send(RenderResult::from_buffer(1, duration, buffer));
        }

        let (duration, buffer) =
            measure(|| render_subdivided(&pathtracer, &pinhole, pinhole.size() / 4));
        combined_buffer += buffer;
        iteration += 1;
        let _ = tx.send(RenderResult::from_buffer(
            iteration,
            duration,
            combined_buffer.clone(),
        ));
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
