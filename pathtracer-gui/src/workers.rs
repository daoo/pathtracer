use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::JoinHandle,
    time,
};

use rand::{rngs::SmallRng, SeedableRng};
use scene::camera::{Camera, Pinhole};
use tracing::{image_buffer::ImageBuffer, pathtracer::Pathtracer, raylogger::RayLogger};

#[derive(Debug, Clone, Copy)]
pub struct RenderMeta {
    pub iteration: u16,
    pub duration: time::Duration,
}

pub struct RenderResult {
    pub meta: RenderMeta,
    pub image: egui::ImageData,
}

fn worker_loop(
    pathtracer: Arc<Pathtracer>,
    start_pinhole: Pinhole,
    rx: Receiver<Pinhole>,
    tx: Sender<RenderResult>,
) {
    let mut rng = SmallRng::from_entropy();
    let mut buffer = ImageBuffer::new(start_pinhole.width, start_pinhole.height);
    let mut pinhole = start_pinhole;
    let mut iteration = 0;
    loop {
        match rx.try_recv() {
            Ok(new_pinhole) => {
                eprintln!("Resetting buffer {new_pinhole:?}");
                buffer = ImageBuffer::new(new_pinhole.width, new_pinhole.height);
                pinhole = new_pinhole;
                iteration = 0;
            }
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => return,
        }
        eprintln!(
            "Rendering {}x{} iteration {}",
            buffer.width(),
            buffer.height(),
            iteration
        );
        let t1 = time::Instant::now();
        pathtracer.render(
            iteration,
            &pinhole,
            &mut RayLogger::None(),
            &mut buffer,
            &mut rng,
        );
        let t2 = time::Instant::now();
        let duration = t2 - t1;
        iteration += 1;
        let image = egui::ColorImage::from_rgba_unmultiplied(
            [buffer.width() as usize, buffer.height() as usize],
            &buffer.div(iteration as f32).gamma_correct().to_rgba8(),
        );
        let _ = tx.send(RenderResult {
            meta: RenderMeta {
                iteration,
                duration,
            },
            image: egui::ImageData::Color(Arc::new(image)),
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

    pub fn send(&self, width: u32, height: u32) {
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
