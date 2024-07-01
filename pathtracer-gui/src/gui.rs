use std::time;

use egui::Vec2;
use nalgebra::Vector3;
use scene::camera::{Camera, Pinhole};
use tracing::pathtracer::Pathtracer;

use crate::workers::Workers;

struct RenderState {
    iteration: u16,
    duration: time::Duration,
    texture: egui::TextureHandle,
}

pub(crate) struct PathtracerGui {
    size: Vec2,
    render: Option<RenderState>,
    last_update: Option<time::Instant>,
    camera: Camera,
    workers: Workers,
}

impl PathtracerGui {
    pub(crate) fn new(pathtracer: Pathtracer) -> Self {
        let (w, h) = (256, 256);
        let camera = pathtracer.scene.cameras[0].clone();
        let pinhole = Pinhole::new(&camera, w, h);
        Self {
            size: Vec2::new(w as f32, h as f32),
            render: None,
            last_update: None,
            camera: pathtracer.scene.cameras[0].clone(),
            workers: Workers::new(pathtracer, pinhole),
        }
    }

    pub(crate) fn run(self) -> Result<(), eframe::Error> {
        eframe::run_native(
            "pathtracer-gui",
            Default::default(),
            Box::new(move |_cc| Box::new(self)),
        )
    }
}

impl eframe::App for PathtracerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label(
                    self.render
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
                    self.workers.send(&pinhole);
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
                        self.workers.send(&pinhole);
                        ctx.request_repaint_after(time::Duration::from_millis(10));
                    }
                    self.last_update = Some(now);
                } else {
                    self.last_update = None;
                }

                if let Some(render) = &self.render {
                    ui.image(&render.texture);
                }
            });

            while let Some(result) = self.workers.try_recv() {
                let image = egui::ColorImage::from_rgba_unmultiplied(
                    [
                        result.image.width() as usize,
                        result.image.height() as usize,
                    ],
                    &result.image.to_rgba8(),
                );
                let texture = ui.ctx().load_texture("buffer", image, Default::default());
                self.render = Some(RenderState {
                    iteration: result.iteration,
                    duration: result.duration,
                    texture,
                });
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.workers.join();
    }
}
