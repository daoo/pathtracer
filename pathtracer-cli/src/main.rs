use clap::Parser;
use kdtree::{
    build::build_kdtree,
    build_sah::{self, SahKdTreeBuilder},
};
use pathtracer::{
    camera::Pinhole, image_buffer::ImageBuffer, pathtracer::Pathtracer, raylogger::RayLogger,
    scene::Scene,
};
use rand::{rngs::SmallRng, SeedableRng};
use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Write},
    str::FromStr,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};
use wavefront::{mtl, obj};

#[derive(Clone, Copy, Debug)]
struct Size {
    width: u32,
    height: u32,
}

impl Size {
    fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }

    fn aspect_ratio(self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

impl FromStr for Size {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s.find('x').expect("Could not parse");
        Ok(Size {
            width: s[0..pos].parse()?,
            height: s[pos + 1..].parse()?,
        })
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
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
    max_bounces: u16,
    /// Iterations to execute per thread
    #[arg(short = 'n', long, default_value_t = 4)]
    iterations_per_thread: u32,
    /// Number of threads
    #[arg(short, long, default_value_t = 1)]
    threads: u32,

    /// Maximum kd-tree depth
    #[arg(long, default_value_t = build_sah::MAX_DEPTH)]
    max_depth: u32,
    /// SAH kd-tree traverse cost
    #[arg(long, default_value_t = build_sah::TRAVERSE_COST)]
    traverse_cost: f32,
    /// SAH kd-tree intersect cost
    #[arg(long, default_value_t = build_sah::INTERSECT_COST)]
    intersect_cost: f32,
    /// SAH kd-tree empty factor
    #[arg(long, default_value_t = build_sah::EMPTY_FACTOR)]
    empty_factor: f32,
}

fn create_ray_logger(thread: u32) -> RayLogger {
    if cfg!(feature = "ray_logging") {
        let path = format!("./tmp/raylog{thread}.bin");
        RayLogger::create(path).unwrap()
    } else {
        RayLogger::None()
    }
}

fn worker_thread(
    thread: u32,
    pathtracer: &Pathtracer,
    size: Size,
    iterations: u32,
    tx: &Sender<Duration>,
) -> ImageBuffer {
    let mut rng = SmallRng::from_entropy();
    let mut buffer = ImageBuffer::new(size.width, size.height);
    let mut ray_logger = create_ray_logger(thread);
    for iteration in 0..iterations {
        let t1 = Instant::now();
        pathtracer.render(iteration as u16, &mut ray_logger, &mut buffer, &mut rng);
        let t2 = Instant::now();
        let duration = t2 - t1;
        tx.send(duration).unwrap();
    }
    buffer
}

fn printer_thread(threads: u32, iterations: u32, rx: &Receiver<Duration>) {
    let mut total = 0.0;
    let mut total_squared = 0.0;
    let mut completed = 0;
    loop {
        if let Ok(duration) = rx.recv() {
            let seconds = duration.as_secs_f64();
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

fn main() {
    let args = Args::parse();

    println!("Loading {}...", args.input.display());
    let obj = obj::obj(&mut BufReader::new(File::open(&args.input).unwrap()));
    println!("  Chunks: {}", obj.chunks.len());
    println!("  Vertices: {}", obj.vertices.len());
    println!("  Normals: {}", obj.normals.len());
    println!("  Texcoords: {}", obj.texcoords.len());

    let mtl_path = args.input.parent().unwrap().join(&obj.mtl_lib);
    println!("Loading {}...", mtl_path.display());
    let mtl = mtl::mtl(&mut BufReader::new(File::open(mtl_path).unwrap()));
    println!("  Materials: {}", mtl.materials.len());
    println!("  Lights: {}", mtl.lights.len());
    println!("  Cameras: {}", mtl.cameras.len());

    println!("Collecting scene...");
    let scene = Scene::from_wavefront(&obj, &mtl);
    println!("  Triangles: {}", scene.triangle_data.len());

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

    let total_iterations = args.threads * args.iterations_per_thread;
    println!(
        "Rendering {} px image with {} thread(s) and {} total iteration(s)...",
        args.size, args.threads, total_iterations,
    );
    let camera = Pinhole::new(&scene.cameras[0], args.size.aspect_ratio());
    let pathtracer = Pathtracer {
        max_bounces: args.max_bounces,
        scene,
        kdtree,
        camera,
    };

    thread::scope(|s| {
        let start_time = Instant::now();
        let (tx, rx) = mpsc::channel();
        let threads = (0..args.threads)
            .map(|i| {
                let tx = tx.clone();
                let pathtracer = &pathtracer;
                s.spawn(move || {
                    worker_thread(i, pathtracer, args.size, args.iterations_per_thread, &tx)
                })
            })
            .collect::<Vec<_>>();
        let printer = s.spawn(move || printer_thread(args.threads, total_iterations, &rx));

        let buffers = threads.into_iter().map(|t| t.join().unwrap());
        let buffer = buffers.reduce(|a, b| a.add(&b)).unwrap();
        drop(tx);
        printer.join().unwrap();

        let duration = Instant::now().duration_since(start_time);
        println!(
            "Total time: {:.2}",
            time::Duration::new(duration.as_secs() as i64, duration.subsec_nanos() as i32)
        );

        println!("Writing {}...", args.output.display());
        buffer
            .div(total_iterations as f32)
            .gamma_correct()
            .save_png(&args.output)
            .unwrap();
    });
}
