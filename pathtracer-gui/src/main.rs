use clap::Parser;
use eframe::egui;
use kdtree::{build::build_kdtree, build_sah::SahKdTreeBuilder};
use rand::{rngs::SmallRng, SeedableRng};
use scene::{camera::Pinhole, Scene};
use std::{
    str,
    sync::{
        mpsc::{self, TryRecvError},
        Arc,
    },
    thread::JoinHandle,
    time,
};
use tracing::{image_buffer::ImageBuffer, pathtracer::Pathtracer, raylogger::RayLogger};

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

#[derive(Debug, Clone)]
struct RenderSettings {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Copy)]
struct RenderMeta {
    iteration: u16,
    duration: time::Duration,
}

struct RenderResult {
    meta: RenderMeta,
    image: egui::ImageData,
}

fn worker_loop(
    start_settings: RenderSettings,
    pathtracer: Arc<Pathtracer>,
    rx: mpsc::Receiver<RenderSettings>,
    tx: mpsc::Sender<RenderResult>,
) {
    let mut rng = SmallRng::from_entropy();
    let mut settings = start_settings;
    let mut buffer = ImageBuffer::new(settings.width, settings.height);
    let mut pinhole = Pinhole::new(&pathtracer.scene.cameras[0], buffer.aspect_ratio());
    let mut iteration = 0;
    loop {
        match rx.try_recv() {
            Ok(new_settings) => {
                eprintln!("Resetting buffer {new_settings:?}");
                settings = new_settings;
                buffer = ImageBuffer::new(settings.width, settings.height);
                pinhole = Pinhole::new(&pathtracer.scene.cameras[0], buffer.aspect_ratio());
                iteration = 0;
            }
            Err(TryRecvError::Empty) => (),
            Err(_) => return,
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

fn new_worker(
    start_settings: RenderSettings,
    pathtracer: Arc<Pathtracer>,
    rx: mpsc::Receiver<RenderSettings>,
    tx: mpsc::Sender<RenderResult>,
) -> JoinHandle<()> {
    std::thread::Builder::new()
        .name("Pathtracer Thread".to_string())
        .spawn(move || worker_loop(start_settings, pathtracer, rx, tx))
        .unwrap()
}

struct PathtracerGui {
    settings: RenderSettings,
    last_meta: Option<RenderMeta>,
    texture: Option<egui::TextureHandle>,
    start_work_tx: mpsc::Sender<RenderSettings>,
    render_result_rc: mpsc::Receiver<RenderResult>,
}

impl PathtracerGui {
    fn new(
        settings: RenderSettings,
        start_work_tx: mpsc::Sender<RenderSettings>,
        render_result_rc: mpsc::Receiver<RenderResult>,
    ) -> Self {
        Self {
            settings,
            last_meta: None,
            texture: None,
            start_work_tx,
            render_result_rc,
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

                if resize {
                    let new_size = [ui.available_size().x as u32, ui.available_size().y as u32];
                    self.settings.width = new_size[0];
                    self.settings.height = new_size[1];
                    self.start_work_tx.send(self.settings.clone()).unwrap();
                }

                if let Some(texture) = &self.texture {
                    ui.image(texture);
                }
            });

            if let Ok(result) = self.render_result_rc.try_recv() {
                self.texture = Some(ui.ctx().load_texture(
                    "buffer",
                    result.image,
                    Default::default(),
                ));
                self.last_meta = Some(result.meta);
            }
        });
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

    let settings = RenderSettings {
        width: 512,
        height: 512,
    };
    let pathtracer = Arc::new(Pathtracer {
        max_bounces: 16,
        scene,
        kdtree,
    });
    let (start_work_tx, start_work_rc) = mpsc::channel::<RenderSettings>();
    let (render_result_tx, render_result_rc) = mpsc::channel::<RenderResult>();
    let thread = new_worker(
        settings.clone(),
        pathtracer,
        start_work_rc,
        render_result_tx,
    );
    let gui = Box::new(PathtracerGui::new(
        settings,
        start_work_tx.clone(),
        render_result_rc,
    ));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([512.0, 512.0]),
        ..Default::default()
    };
    eframe::run_native("pathtracer-cli", options, Box::new(move |_cc| gui)).unwrap();

    drop(start_work_tx);
    thread.join().unwrap();
}
