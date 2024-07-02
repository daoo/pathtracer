use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    thread::JoinHandle,
    time,
};

use egui::Vec2;
use nalgebra::Vector3;
use scene::camera::{Camera, Pinhole};
use tracing::pathtracer::Pathtracer;

use crate::workers::{spawn_worker, RenderResult};

struct RenderState {
    iteration: u16,
    duration: time::Duration,
}

fn spawn_receiver_thread(
    rx: Receiver<RenderResult>,
    render_ptr: Arc<Mutex<Option<RenderState>>>,
    image_ptr: Arc<Mutex<Option<egui::ColorImage>>>,
    ctx: egui::Context,
) -> Result<JoinHandle<()>, std::io::Error> {
    std::thread::Builder::new()
        .name("GUI receiver".to_string())
        .spawn(move || {
            while let Ok(result) = rx.recv() {
                render_ptr.lock().unwrap().replace(RenderState {
                    iteration: result.iteration,
                    duration: result.duration,
                });
                image_ptr
                    .lock()
                    .unwrap()
                    .replace(egui::ColorImage::from_rgba_unmultiplied(
                        [
                            result.image.width() as usize,
                            result.image.height() as usize,
                        ],
                        &result
                            .image
                            .div(result.iteration as f32)
                            .gamma_correct()
                            .to_rgba8(),
                    ));
                ctx.request_repaint();
            }
        })
}

pub(crate) struct PathtracerGui {
    size: Vec2,

    camera: Camera,
    last_update: Option<time::Instant>,

    render: Arc<Mutex<Option<RenderState>>>,
    image: Arc<Mutex<Option<egui::ColorImage>>>,

    texture: Option<egui::TextureHandle>,

    receiver_thread: Option<JoinHandle<()>>,
    worker_thread: Option<JoinHandle<()>>,
    worker_tx: Option<Sender<Pinhole>>,
    worker_rx: Option<Receiver<RenderResult>>,
}

impl PathtracerGui {
    pub(crate) fn new(camera: Camera) -> Self {
        Self {
            size: Vec2::new(256.0, 256.0),
            camera,
            last_update: None,
            render: Arc::new(Mutex::new(None)),
            image: Arc::new(Mutex::new(None)),
            texture: None,
            receiver_thread: None,
            worker_thread: None,
            worker_tx: None,
            worker_rx: None,
        }
    }

    pub(crate) fn run(mut self, pathtracer: Pathtracer) -> Result<(), eframe::Error> {
        eframe::run_native(
            "pathtracer-gui",
            Default::default(),
            Box::new(move |_cc| {
                let pinhole = Pinhole::new(&self.camera, self.size.x as u32, self.size.y as u32);
                let (thread, tx, rx) = spawn_worker(pathtracer, pinhole);
                self.worker_thread = Some(thread);
                self.worker_tx = Some(tx);
                self.worker_rx = Some(rx);
                self.receiver_thread = Some(
                    spawn_receiver_thread(
                        self.worker_rx.take().unwrap(),
                        Arc::clone(&self.render),
                        Arc::clone(&self.image),
                        _cc.egui_ctx.clone(),
                    )
                    .unwrap(),
                );
                Box::new(self)
            }),
        )
    }

    pub fn set_pinhole(&mut self, pinhole: &Pinhole) {
        self.worker_tx
            .iter()
            .for_each(|tx| tx.send(pinhole.clone()).unwrap());
    }

    pub fn stop_worker_thread(&mut self) {
        if let Some(tx) = self.worker_tx.take() {
            drop(tx)
        }
        if let Some(t) = self.worker_thread.take() {
            t.join().unwrap()
        }
    }
}

impl eframe::App for PathtracerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label(
                    self.render
                        .lock()
                        .unwrap()
                        .as_ref()
                        .map_or("rendering...".to_string(), |meta| {
                            format!("{} in {} ms", meta.iteration, meta.duration.as_millis())
                        }),
                );

                let mut resize = false;
                ui.horizontal(|ui| {
                    resize = ui.button("Auto-Size").clicked();
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                let movement_value = |key1, key2| {
                    let (key1, key2) = ui.input(|i| (i.key_down(key1), i.key_down(key2)));
                    if key1 && !key2 {
                        -1.0
                    } else if !key1 && key2 {
                        1.0
                    } else {
                        0.0
                    }
                };
                let translation = Vector3::new(
                    movement_value(egui::Key::A, egui::Key::E),
                    movement_value(egui::Key::Semicolon, egui::Key::Period),
                    movement_value(egui::Key::O, egui::Key::Comma),
                );

                if resize {
                    self.size = ui.available_size();
                    let pinhole =
                        Pinhole::new(&self.camera, self.size.x as u32, self.size.y as u32);
                    self.set_pinhole(&pinhole);
                    ctx.request_repaint_after(time::Duration::from_millis(10));
                } else if translation != Vector3::zeros() {
                    let now = time::Instant::now();
                    if let Some(last_update) = self.last_update {
                        let delta = (now - last_update).as_secs_f32();
                        const TRANSLATION_SPEED: f32 = 2.0;
                        let translation_x = delta * TRANSLATION_SPEED * translation.x;
                        let translation_y = delta * TRANSLATION_SPEED * translation.y;
                        let translation_z = delta * TRANSLATION_SPEED * translation.z;
                        self.camera = self.camera.with_position(
                            self.camera.position
                                + translation_x * *self.camera.right
                                + translation_y * *self.camera.up
                                + translation_z * *self.camera.direction,
                        );
                        let pinhole =
                            Pinhole::new(&self.camera, self.size.x as u32, self.size.y as u32);
                        self.set_pinhole(&pinhole);
                        ctx.request_repaint_after(time::Duration::from_millis(10));
                    }
                    self.last_update = Some(now);
                } else {
                    self.last_update = None;
                }

                if let Some(texture) = &self.texture {
                    ui.image(texture);
                }
            });

            if let Some(image) = self.image.lock().unwrap().take() {
                self.texture = Some(ui.ctx().load_texture("buffer", image, Default::default()));
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.stop_worker_thread();
        self.receiver_thread.take().unwrap().join().unwrap();
    }
}
