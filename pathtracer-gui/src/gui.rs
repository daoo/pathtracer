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
use tracing::{image_buffer::ImageBuffer, pathtracer::Pathtracer};

use crate::workers::{spawn_worker, RenderResult};

struct RenderState {
    iteration: u16,
    duration: time::Duration,
}

fn convert(iterations: u16, buffer: ImageBuffer) -> egui::ColorImage {
    let size = [buffer.width as usize, buffer.height as usize];
    let pixels = buffer
        .into_rgba_iter(iterations)
        .map(|p| egui::Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
        .collect();
    egui::ColorImage { size, pixels }
}

fn receiver_thread(
    rx: Receiver<RenderResult>,
    render_ptr: Arc<Mutex<Option<RenderState>>>,
    image_ptr: Arc<Mutex<Option<egui::ColorImage>>>,
    ctx: egui::Context,
) {
    while let Ok(result) = rx.recv() {
        let image = convert(result.iteration, result.image);
        render_ptr.lock().unwrap().replace(RenderState {
            iteration: result.iteration,
            duration: result.duration,
        });
        image_ptr.lock().unwrap().replace(image);
        ctx.request_repaint();
    }
}

fn spawn_receiver(
    rx: Receiver<RenderResult>,
    render_ptr: Arc<Mutex<Option<RenderState>>>,
    image_ptr: Arc<Mutex<Option<egui::ColorImage>>>,
    ctx: egui::Context,
) -> Result<JoinHandle<()>, std::io::Error> {
    std::thread::Builder::new()
        .name("GUI receiver".to_string())
        .spawn(move || receiver_thread(rx, render_ptr, image_ptr, ctx))
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
                let pinhole =
                    Pinhole::new(self.camera.clone(), self.size.x as u32, self.size.y as u32);
                let (thread, tx, rx) = spawn_worker(pathtracer, pinhole);
                self.worker_thread = Some(thread);
                self.worker_tx = Some(tx);
                self.worker_rx = Some(rx);
                self.receiver_thread = Some(
                    spawn_receiver(
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

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
        self.update_pinhole();
    }

    pub fn set_camera_position(&mut self, camera_position: Vector3<f32>) {
        self.camera = self.camera.with_position(camera_position);
        self.update_pinhole();
    }

    pub fn update_pinhole(&mut self) {
        let pinhole = Pinhole::new(self.camera.clone(), self.size.x as u32, self.size.y as u32);
        if let Some(tx) = &self.worker_tx {
            tx.send(pinhole).unwrap()
        }
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
            if let Some(image) = self.image.lock().unwrap().take() {
                self.texture = Some(ui.ctx().load_texture("buffer", image, Default::default()));
            }

            let movement_value =
                |key1, key2| match ui.input(|i| (i.key_down(key1), i.key_down(key2))) {
                    (true, false) => -1.0,
                    (false, true) => 1.0,
                    _ => 0.0,
                };
            let translation = Vector3::new(
                movement_value(egui::Key::A, egui::Key::E),
                movement_value(egui::Key::Semicolon, egui::Key::Period),
                movement_value(egui::Key::O, egui::Key::Comma),
            );

            if translation != Vector3::zeros() {
                let now = time::Instant::now();
                if let Some(last_update) = self.last_update {
                    let delta = (now - last_update).as_secs_f32();
                    const TRANSLATION_SPEED: f32 = 2.0;
                    let translation_x = delta * TRANSLATION_SPEED * translation.x;
                    let translation_y = delta * TRANSLATION_SPEED * translation.y;
                    let translation_z = delta * TRANSLATION_SPEED * translation.z;
                    self.set_camera_position(
                        self.camera.position
                            + translation_x * *self.camera.right
                            + translation_y * *self.camera.up
                            + translation_z * *self.camera.direction,
                    );
                }
                self.last_update = Some(now);
            } else {
                self.last_update = None;
            }

            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    let mut resize = false;
                    ui.vertical(|ui| {
                        ui.label("Camera");
                        ui.label(format!(
                            "  Postion: {:.2} {:.2} {:.2}",
                            self.camera.position.x, self.camera.position.y, self.camera.position.z
                        ));
                        ui.label(format!(
                            "  Direction: {:.2} {:.2} {:.2}",
                            self.camera.direction.x,
                            self.camera.direction.y,
                            self.camera.direction.z
                        ));
                        ui.label(format!(
                            "Size: {} x {} pixels",
                            self.size.x as u32, self.size.y as u32
                        ));
                        if let Some(render) = self.render.lock().unwrap().as_ref() {
                            ui.label(format!("Iteration: {}", render.iteration));
                            ui.label(format!(
                                "Render Time: {} ms/iteration",
                                render.duration.as_millis()
                            ));
                        }

                        ui.horizontal(|ui| {
                            resize = ui.button("Auto-Size").clicked();
                            if ui.button("Exit").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                    });

                    if resize {
                        self.set_size(ui.available_size());
                    }
                    if let Some(texture) = &self.texture {
                        ui.add(egui::Image::from_texture(texture).fit_to_exact_size(self.size));
                    }
                },
            );
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.stop_worker_thread();
        self.receiver_thread.take().unwrap().join().unwrap();
    }
}
