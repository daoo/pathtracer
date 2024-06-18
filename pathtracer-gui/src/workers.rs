use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::JoinHandle,
    time,
};

use nalgebra::Vector3;
use rand::{rngs::SmallRng, SeedableRng};
use scene::camera::{Camera, Pinhole};
use tracing::{
    image_buffer::ImageBuffer,
    pathtracer::{Pathtracer, Subdivision},
    raylogger::{RayLoggerWithIteration, RayLoggerWriter},
};

pub struct RenderResult {
    pub iteration: u16,
    pub duration: time::Duration,
    pub image: ImageBuffer,
}

fn worker_loop(
    pathtracer: Arc<Pathtracer>,
    half: u32,
    start_pinhole: Pinhole,
    rx: Receiver<Pinhole>,
    tx: Sender<RenderResult>,
) {
    let mut rng = SmallRng::from_entropy();
    let mut pinhole = start_pinhole;
    let mut iteration = 0;
    let mut buffer = ImageBuffer::new(pinhole.width, pinhole.height);
    loop {
        match rx.try_recv() {
            Ok(new_pinhole) => {
                eprintln!("Resetting buffer {new_pinhole:?}");
                pinhole = new_pinhole;
                buffer = ImageBuffer::new(pinhole.width, pinhole.height);
                iteration = 0;
            }
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => return,
        }
        eprintln!(
            "Rendering {}x{} iteration {}",
            pinhole.width, pinhole.height, iteration
        );
        let t1 = time::Instant::now();
        let mut ray_logger = RayLoggerWithIteration {
            writer: &mut RayLoggerWriter::None(),
            iteration,
        };
        let subdivision = Subdivision {
            x1: 0,
            y1: if half == 0 { 0 } else { pinhole.height / 2 },
            x2: pinhole.width,
            y2: if half == 0 {
                pinhole.height / 2
            } else {
                pinhole.height
            },
        };
        pathtracer.render_subdivided_mut(
            &pinhole,
            &mut ray_logger,
            &mut rng,
            &mut buffer,
            subdivision,
        );
        let t2 = time::Instant::now();
        let duration = t2 - t1;
        iteration += 1;
        let _ = tx.send(RenderResult {
            iteration,
            duration,
            image: buffer.clone().div(iteration as f32).gamma_correct(),
        });
    }
}

pub struct Workers {
    camera: Camera,
    threads: Vec<JoinHandle<()>>,
    settings: Vec<Sender<Pinhole>>,
    result: Receiver<RenderResult>,
}

impl Workers {
    pub fn new(pathtracer: Arc<Pathtracer>, width: u32, height: u32) -> Workers {
        let (render_result_tx, render_result_rx) = mpsc::channel::<RenderResult>();
        let camera = pathtracer.scene.cameras[0].clone();
        let pinhole = Pinhole::new(&camera, width, height);
        let (render_settings_txs, threads): (Vec<_>, Vec<_>) = (0..=1)
            .map(|thread| {
                let pathtracer = pathtracer.clone();
                let pinhole = pinhole.clone();
                let (render_settings_tx, render_settings_rx) = mpsc::channel::<Pinhole>();
                let render_result_tx = render_result_tx.clone();
                let thread = std::thread::Builder::new()
                    .name(format!("Pathtracer Thread {thread}"))
                    .spawn(move || {
                        worker_loop(
                            pathtracer,
                            thread,
                            pinhole,
                            render_settings_rx,
                            render_result_tx,
                        )
                    })
                    .unwrap();
                (render_settings_tx, thread)
            })
            .unzip();

        Workers {
            camera,
            threads,
            settings: render_settings_txs,
            result: render_result_rx,
        }
    }

    pub fn send(&mut self, width: u32, height: u32, translation: Vector3<f32>) {
        self.camera = self.camera.translate(&translation);
        self.settings
            .iter()
            .for_each(|tx| tx.send(Pinhole::new(&self.camera, width, height)).unwrap());
    }

    pub fn try_recv(&self) -> Option<RenderResult> {
        self.result.try_recv().ok()
    }

    pub fn join(&mut self) {
        self.settings.clear();
        while let Some(thread) = self.threads.pop() {
            thread.join().unwrap();
        }
    }
}
