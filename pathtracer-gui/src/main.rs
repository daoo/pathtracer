use clap::Parser;
use eframe::egui;
use egui::Vec2;
use kdtree::{build::build_kdtree, build_sah::SahKdTreeBuilder};
use nalgebra::Vector3;
use scene::Scene;
use std::{str, sync::Arc};
use tracing::pathtracer::Pathtracer;
use workers::{RenderMeta, Workers};

mod workers;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    #[arg(long, default_value_t = 20)]
    max_depth: u32,
    #[arg(long, default_value_t = 2.0)]
    traverse_cost: f32,
    #[arg(long, default_value_t = 1.0)]
    intersect_cost: f32,
    #[arg(long, default_value_t = 0.8)]
    empty_factor: f32,
}

struct PathtracerGui {
    size: Vec2,
    last_meta: Option<RenderMeta>,
    texture: Option<egui::TextureHandle>,
    workers: Workers,
}

impl PathtracerGui {
    fn new(workers: Workers) -> Self {
        Self {
            size: Vec2::new(512.0, 512.0),
            last_meta: None,
            texture: None,
            workers,
        }
    }
}

impl eframe::App for PathtracerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label(self.last_meta.map_or("rendering...".to_string(), |meta| {
                    format!("{} in {} ms", meta.iteration, meta.duration.as_millis())
                }));

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
                    self.workers
                        .send(self.size.x as u32, self.size.y as u32, translation);
                }

                if let Some(texture) = &self.texture {
                    ui.image(texture);
                }
            });

            if let Some(result) = self.workers.try_recv() {
                self.texture = Some(ui.ctx().load_texture(
                    "buffer",
                    result.image,
                    Default::default(),
                ));
                self.last_meta = Some(result.meta);
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.workers.join();
    }
}

fn main() {
    let args = Args::parse();
    let scene = Scene::read_obj_file_with_print_logging(&args.input);

    println!("Building kdtree...");
    let kdtree = build_kdtree(
        SahKdTreeBuilder {
            traverse_cost: args.traverse_cost,
            intersect_cost: args.intersect_cost,
            empty_factor: args.empty_factor,
            geometries: scene
                .triangle_data
                .iter()
                .map(|t| t.triangle.into())
                .collect(),
        },
        args.max_depth,
    );

    let pathtracer = Arc::new(Pathtracer {
        max_bounces: 16,
        scene,
        kdtree,
    });
    let gui = PathtracerGui::new(Workers::new(pathtracer, 512, 512));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([512.0, 512.0]),
        ..Default::default()
    };
    eframe::run_native(
        "pathtracer-cli",
        options,
        Box::new(move |_cc| Box::new(gui)),
    )
    .unwrap();
}
