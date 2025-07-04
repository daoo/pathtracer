use clap::Parser;
use geometry::geometry::from_wavefront;
use glam::{UVec2, Vec3};
use image::ImageFormat;
use kdtree::{build::build_kdtree, sah::SahCost};
use std::{
    fmt::Display,
    io::Write,
    str::FromStr,
    sync::mpsc::{self, Receiver},
    thread,
};
use time::Duration;
use tracing::{
    camera::Pinhole, light::Light, material::Material, pathtracer::Pathtracer,
    worker::render_parallel_iterations,
};
use wavefront::read_obj_and_mtl_with_print_logging;

#[derive(Clone, Copy, Debug)]
struct Size {
    x: u32,
    y: u32,
}

impl Size {
    fn new(x: u32, y: u32) -> Self {
        Size { x, y }
    }

    fn as_uvec2(self) -> UVec2 {
        UVec2::new(self.x, self.y)
    }
}

impl FromStr for Size {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s.find('x').expect("Could not parse");
        Ok(Size {
            x: s[0..pos].parse()?,
            y: s[pos + 1..].parse()?,
        })
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Wavefront OBJ input path
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    /// PNG output path
    #[arg(short, long, required = true)]
    output: std::path::PathBuf,
    /// Image size in pixels
    #[arg(short, long, default_value_t = Size::new(512, 512))]
    size: Size,
    /// Max number of bounces
    #[arg(short, long, default_value_t = 10)]
    max_bounces: u8,
    /// Iterations to execute per thread
    #[arg(short = 'n', long, default_value_t = 4)]
    iterations_per_thread: u32,
    /// Number of threads
    #[arg(short, long, default_value_t = 1)]
    threads: u32,

    /// SAH kd-tree traverse cost
    #[arg(long, default_value_t = SahCost::default().traverse_cost)]
    traverse_cost: f32,
    /// SAH kd-tree intersect cost
    #[arg(long, default_value_t = SahCost::default().intersect_cost)]
    intersect_cost: f32,
    /// SAH kd-tree empty factor
    #[arg(long, default_value_t = SahCost::default().empty_factor)]
    empty_factor: f32,
}

fn printer_thread(threads: u32, iterations: u32, rx: &Receiver<Duration>) {
    let mut total = 0.0;
    let mut total_squared = 0.0;
    let mut completed = 0;
    loop {
        if let Ok(duration) = rx.recv() {
            let seconds = duration.as_seconds_f64();
            total += seconds;
            total_squared += seconds * seconds;
            completed += 1;

            let mean = total / f64::from(completed);
            let sdev = ((total_squared / f64::from(completed)) - mean * mean).sqrt();
            let eta = (f64::from(iterations - completed) * mean) / f64::from(threads);
            // ANSI escape codes
            const CSI_ERASE_IN_LINE: &str = "\x1B[1K";
            const CSI_CURSOR_HORIZONTAL_ABSOLUTE: &str = "\x1B[1G";
            print!(
                "{}{}[{}/{}] mean: {:.2}, sdev: {:.2}, eta: {:.2}",
                CSI_ERASE_IN_LINE,
                CSI_CURSOR_HORIZONTAL_ABSOLUTE,
                completed,
                iterations,
                time::Duration::seconds_f64(mean),
                time::Duration::seconds_f64(sdev),
                time::Duration::seconds_f64(eta)
            );
            std::io::stdout().flush().unwrap();
        } else {
            println!();
            return;
        }
    }
}

fn main() {
    let args = Args::parse();
    let (obj, mtl, mtl_path) = read_obj_and_mtl_with_print_logging(&args.input).unwrap();
    let (geometries, properties) = from_wavefront(&obj, &mtl);

    println!("Building kdtree...");
    let accelerator = build_kdtree(
        &geometries,
        &SahCost {
            traverse_cost: args.traverse_cost,
            intersect_cost: args.intersect_cost,
            empty_factor: args.empty_factor,
        },
    );

    let total_iterations = args.threads * args.iterations_per_thread;
    println!(
        "Rendering {} px image with {} thread(s) and {} total iteration(s)...",
        args.size, args.threads, total_iterations,
    );
    let camera = Pinhole::new(mtl.cameras[0].clone().into(), args.size.as_uvec2());
    let image_directory = mtl_path.parent().unwrap();
    let materials = mtl
        .materials
        .iter()
        .map(|m| Material::load_from_mtl(image_directory, m))
        .collect();
    let lights = mtl.lights.iter().map(Light::from).collect();
    let pathtracer = Pathtracer {
        max_bounces: args.max_bounces,
        geometries,
        properties,
        materials,
        lights,
        environment: Vec3::new(0.8, 0.8, 0.8),
        accelerator,
    };

    thread::scope(|s| {
        let (tx, rx) = mpsc::channel();
        let printer = s.spawn(move || printer_thread(args.threads, total_iterations, &rx));
        let (duration, image) = render_parallel_iterations(
            &pathtracer,
            camera,
            args.size.as_uvec2(),
            args.threads,
            args.iterations_per_thread,
            tx,
        );
        printer.join().unwrap();
        println!("Total time: {duration:.2}");

        println!("Writing {}...", args.output.display());
        image
            .save_with_format(&args.output, ImageFormat::Png)
            .unwrap();
    });
}
