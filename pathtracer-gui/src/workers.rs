use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::JoinHandle,
    time,
};

use nalgebra::Vector3;
use scene::camera::{Camera, Pinhole};
use tracing::{image_buffer::ImageBuffer, pathtracer::Pathtracer};

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
        combined_buffer.add_mut(&pathtracer.render(&pinhole, 0..pinhole.width, 0..pinhole.height));
        let t2 = time::Instant::now();
        let duration = t2 - t1;
        iteration += 1;
        let _ = tx.send(RenderResult {
            iteration,
            duration,
            image: combined_buffer
                .clone()
                .div(iteration as f32)
                .gamma_correct(),
        });
    }
}

pub struct Workers {
    camera: Camera,
    threads: Vec<JoinHandle<()>>,
    settings: Option<Sender<Pinhole>>,
    result: Receiver<RenderResult>,
}

impl Workers {
    pub fn new(pathtracer: Arc<Pathtracer>, width: u32, height: u32) -> Workers {
        let (render_settings_tx, render_settings_rx) = mpsc::channel::<Pinhole>();
        let (render_result_tx, render_result_rx) = mpsc::channel::<RenderResult>();
        let camera = pathtracer.scene.cameras[0].clone();
        let pinhole = Pinhole::new(&camera, width, height);
        let thread = std::thread::Builder::new()
            .name("Pathtracer Thread".to_string())
            .spawn(move || worker_loop(pathtracer, pinhole, render_settings_rx, render_result_tx))
            .unwrap();

        Workers {
            camera,
            threads: vec![thread],
            settings: Some(render_settings_tx),
            result: render_result_rx,
        }
    }

    pub fn send(&mut self, width: u32, height: u32, translation: Vector3<f32>) {
        self.camera = self.camera.translate(&translation);
        self.settings
            .as_ref()
            .unwrap()
            .send(Pinhole::new(&self.camera, width, height))
            .unwrap()
    }

    pub fn try_recv(&self) -> Option<RenderResult> {
        self.result.try_recv().ok()
    }

    pub fn join(&mut self) {
        self.settings = None;
        while let Some(thread) = self.threads.pop() {
            thread.join().unwrap();
        }
    }
}
