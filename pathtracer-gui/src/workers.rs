use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
    time,
};

use rand::{rngs::SmallRng, SeedableRng};
use scene::camera::Pinhole;
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
    start_pinhole: Pinhole,
    rx: Receiver<Pinhole>,
    tx: Sender<RenderResult>,
) {
    let mut pinhole = start_pinhole;
    let mut iteration = 0;
    let mut combined_buffer = ImageBuffer::new(pinhole.width, pinhole.height);
    loop {
        match rx.try_recv() {
            Ok(new_pinhole) => {
                eprintln!("Resetting buffer {new_pinhole:?}");
                pinhole = new_pinhole;
                combined_buffer = ImageBuffer::new(pinhole.width, pinhole.height);
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
        let [tl, tr, bl, br] = thread::scope(|s| {
            let spawn = |x, y, w, h| {
                let pathtracer = pathtracer.clone();
                let pinhole = pinhole.clone();
                s.spawn(move || {
                    let mut rng = SmallRng::from_entropy();
                    let mut buffer = ImageBuffer::new(pinhole.width, pinhole.height);
                    let mut ray_logger = RayLoggerWithIteration {
                        writer: &mut RayLoggerWriter::None(),
                        iteration,
                    };
                    let subdivision = Subdivision {
                        x1: x,
                        y1: y,
                        x2: x + w,
                        y2: y + h,
                    };
                    pathtracer.render_subdivided_mut(
                        &pinhole,
                        &mut ray_logger,
                        &mut rng,
                        &mut buffer,
                        subdivision,
                    );
                    buffer
                })
            };
            let w = pinhole.width / 2;
            let h = pinhole.height / 2;
            let tl = spawn(0, 0, w, h);
            let tr = spawn(w, 0, w, h);
            let bl = spawn(0, h, w, h);
            let br = spawn(w, h, w, h);
            [
                tl.join().unwrap(),
                tr.join().unwrap(),
                bl.join().unwrap(),
                br.join().unwrap(),
            ]
        });
        let t2 = time::Instant::now();
        let duration = t2 - t1;
        iteration += 1;
        combined_buffer.add_mut(&tl.add(&tr).add(&bl).add(&br));
        let _ = tx.send(RenderResult {
            iteration,
            duration,
            image: combined_buffer.div(iteration as f32).gamma_correct(),
        });
    }
}

pub struct Workers {
    threads: Vec<JoinHandle<()>>,
    settings: Vec<Sender<Pinhole>>,
    result: Receiver<RenderResult>,
}

impl Workers {
    pub fn new(pathtracer: Pathtracer, pinhole: Pinhole) -> Workers {
        let (render_result_tx, render_result_rx) = mpsc::channel::<RenderResult>();
        let (render_settings_tx, render_settings_rx) = mpsc::channel::<Pinhole>();
        let thread = std::thread::Builder::new()
            .name("Pathtracer Thread".to_string())
            .spawn(move || {
                worker_loop(
                    Arc::new(pathtracer),
                    pinhole,
                    render_settings_rx,
                    render_result_tx,
                )
            })
            .unwrap();

        Workers {
            threads: vec![thread],
            settings: vec![render_settings_tx],
            result: render_result_rx,
        }
    }

    pub fn send(&mut self, pinhole: &Pinhole) {
        self.settings
            .iter()
            .for_each(|tx| tx.send(pinhole.clone()).unwrap());
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
