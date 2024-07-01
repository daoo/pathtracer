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
    camera: Camera,
    workers: Workers,
}

impl PathtracerGui {
    pub(crate) fn new(pathtracer: Pathtracer) -> Self {
        let (w, h) = (512, 512);
        let camera = pathtracer.scene.cameras[0].clone();
        let pinhole = Pinhole::new(&camera, w, h);
        Self {
            size: Vec2::new(w as f32, h as f32),
            render: None,
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

                let mut translation = Vector3::zeros();
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    translation.y = 0.1;
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    translation.y = -0.1;
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                    translation.x = -0.1;
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                    translation.x = 0.1;
                }

                if resize {
                    self.size = ui.available_size();
                }

                if resize || translation != Vector3::zeros() {
                    self.camera = self.camera.translate(&translation);
                    let pinhole =
                        Pinhole::new(&self.camera, self.size.x as u32, self.size.y as u32);
                    self.workers.send(&pinhole);
                }

                if let Some(render) = &self.render {
                    ui.image(&render.texture);
                }
            });

            if let Some(result) = self.workers.try_recv() {
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
