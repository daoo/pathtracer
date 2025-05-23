use clap::Parser;
use geometry::{
    geometry::{GeometryIntersection, GeometryProperties, intersect_closest_geometry},
    shape::Shape,
    sphere::Sphere,
};
use glam::{UVec2, Vec3};
use image::ImageFormat;
use kdtree::IntersectionAccelerator;
use std::{
    fmt::Display,
    io::Write,
    str::FromStr,
    sync::mpsc::{self, Receiver},
    thread,
};
use time::Duration;
use tracing::{
    camera::{Camera, Pinhole},
    light::{DirectionalLight, Light},
    material::Material,
    pathtracer::Pathtracer,
    worker::render_parallel_iterations,
};

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

            let mean = total / completed as f64;
            let sdev = ((total_squared / completed as f64) - mean * mean).sqrt();
            let eta = ((iterations - completed) as f64 * mean) / threads as f64;
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

struct NoAccelerator {}

impl IntersectionAccelerator for NoAccelerator {
    fn intersect(
        &self,
        geometries: &[Shape],
        ray: &geometry::ray::Ray,
        t_range: std::ops::RangeInclusive<f32>,
    ) -> Option<GeometryIntersection> {
        let indices = 0u32..geometries.len() as u32;
        intersect_closest_geometry(geometries, indices, ray, t_range)
    }
}

fn main() {
    let args = Args::parse();

    let total_iterations = args.threads * args.iterations_per_thread;
    println!(
        "Rendering {} px image with {} thread(s) and {} total iteration(s)...",
        args.size, args.threads, total_iterations,
    );
    let camera = Camera::new(
        [-10.0, 0.0, 0.0].into(),
        [0.0, 0.0, 0.0].into(),
        [0.0, 0.0, 1.0].into(),
        20.0,
    );
    let pinhole = Pinhole::new(camera, args.size.as_uvec2());
    let spheres = [
        Sphere::new(Vec3::new(0.0, -1.0, -1.0), 0.45),
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.45),
        Sphere::new(Vec3::new(0.0, 1.0, -1.0), 0.45),
        Sphere::new(Vec3::new(0.0, -1.0, 0.0), 0.45),
        Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.45),
        Sphere::new(Vec3::new(0.0, 1.0, 0.0), 0.45),
        Sphere::new(Vec3::new(0.0, -1.0, 1.0), 0.45),
        Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.45),
        Sphere::new(Vec3::new(0.0, 1.0, 1.0), 0.45),
    ];
    let properties = spheres
        .iter()
        .enumerate()
        .map(|(i, s)| GeometryProperties::Sphere {
            material: i,
            radius: s.radius,
        })
        .collect();
    let material = |t| Material {
        diffuse_reflectance: [1.0, 0.0, 0.0].into(),
        diffuse_texture_reflectance: None,
        specular_reflectance: [1.0, t, 0.1].into(),
        index_of_refraction: 0.0,
        reflection_0_degrees: 1.0,
        reflection_90_degrees: 1.0,
        transparency: 0.0,
    };
    let materials = (0..spheres.len())
        .map(|i| material(i as f32 * 1.0 / (spheres.len() - 1) as f32))
        .collect();
    //let lights = [PointLight {
    //    center: Vec3::new(-10.0, -5.0, 1.0) * 100.0,
    //    intensity: Vec3::ONE * 100.0 * 100.0f32.powi(2),
    //}];
    let lights = [DirectionalLight {
        direction: Vec3::new(-1.0, -1.0, 1.0),
        intensity: Vec3::ONE,
    }];
    let accelerator = NoAccelerator {};
    let pathtracer = Pathtracer {
        max_bounces: args.max_bounces,
        geometries: spheres.map(Shape::from).to_vec(),
        properties,
        materials,
        lights: lights.map(Light::from).to_vec(),
        environment: Vec3::new(0.8, 0.8, 0.8),
        accelerator,
    };

    thread::scope(|s| {
        let (tx, rx) = mpsc::channel();
        let printer = s.spawn(move || printer_thread(args.threads, total_iterations, &rx));
        let (duration, image) = render_parallel_iterations(
            &pathtracer,
            pinhole,
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
