use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::JoinHandle,
};

use glam::UVec2;
use kdtree::KdNode;
use time::Duration;
use tracing::{
    camera::Pinhole, image_buffer::ImageBuffer, measure::measure, pathtracer::Pathtracer,
    worker::render_parallel_subdivided,
};

pub struct RenderResult {
    pub iterations: u16,
    pub duration: Duration,
    pub buffer: ImageBuffer,
}

impl RenderResult {
    fn new(iterations: u16, duration: Duration, buffer: ImageBuffer) -> Self {
        RenderResult {
            iterations,
            duration,
            buffer,
        }
    }
}

fn worker_loop(
    pathtracer: &Pathtracer<KdNode>,
    start_pinhole: Pinhole,
    rx: &Receiver<Pinhole>,
    tx: &Sender<RenderResult>,
) {
    let mut pinhole = start_pinhole;
    let mut iteration = 0;
    let mut combined_buffer = ImageBuffer::new(pinhole.size);
    loop {
        loop {
            match rx.try_recv() {
                Ok(new_pinhole) => {
                    pinhole = new_pinhole;
                    combined_buffer = ImageBuffer::new(pinhole.size);
                    iteration = 0;
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => return,
            }
        }
        if iteration == 0 {
            let pinhole_small = Pinhole::new(
                pinhole.camera.clone(),
                UVec2::new(64, 64 * pinhole.size.y / pinhole.size.x),
            );
            let (duration, buffer) = measure(|| {
                render_parallel_subdivided(pathtracer, &pinhole_small, pinhole_small.size / 3)
            });
            let _ = tx.send(RenderResult::new(1, duration, buffer));
            iteration = 1;
        } else {
            let (duration, buffer) =
                measure(|| render_parallel_subdivided(pathtracer, &pinhole, pinhole.size / 4));
            combined_buffer += buffer;
            let _ = tx.send(RenderResult::new(
                iteration,
                duration,
                combined_buffer.clone(),
            ));
            iteration += 1;
        }
    }
}

pub(crate) struct Worker {
    thread: JoinHandle<()>,
    pinhole_tx: Sender<Pinhole>,
    result_rx: Receiver<RenderResult>,
}

impl Worker {
    pub fn spawn(pathtracer: Pathtracer<KdNode>, pinhole: Pinhole) -> Self {
        let (result_tx, result_rx) = mpsc::channel::<RenderResult>();
        let (pinhole_tx, pinhole_rx) = mpsc::channel::<Pinhole>();
        let thread = std::thread::Builder::new()
            .name("Pathtracer Worker".to_string())
            .spawn(move || worker_loop(&pathtracer, pinhole, &pinhole_rx, &result_tx))
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
