use clap::Parser;
use kdtree::{build::build_kdtree, build_sah::SahKdTreeBuilder};
use pathtracer::{
    camera::Pinhole, image_buffer::ImageBuffer, pathtracer::Pathtracer, raylogger::RayLogger,
    scene::Scene,
};
use rand::{rngs::SmallRng, SeedableRng};
use std::{
    fs,
    io::Write,
    str,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};
use wavefront::{mtl, obj};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    #[arg(short, long, required = true)]
    output: std::path::PathBuf,
    #[arg(short, long, default_value_t = 512)]
    width: u32,
    #[arg(short, long, default_value_t = 512)]
    height: u32,
    #[arg(short, long, default_value_t = 10)]
    max_bounces: u16,
    #[arg(short = 'n', long, default_value_t = 4)]
    iterations_per_thread: u32,
    #[arg(short, long, default_value_t = 1)]
    threads: u32,

    #[arg(long, default_value_t = 20)]
    max_depth: u32,
    #[arg(long, default_value_t = 2.0)]
    traverse_cost: f32,
    #[arg(long, default_value_t = 1.0)]
    intersect_cost: f32,
    #[arg(long, default_value_t = 0.8)]
    empty_factor: f32,
}

fn create_ray_logger(thread: u32) -> RayLogger {
    if cfg!(feature = "ray_logging") {
        let path = format!("/tmp/raylog{thread}.bin");
        RayLogger::create(path).unwrap()
    } else {
        RayLogger::None()
    }
}

fn worker_thread(
    thread: u32,
    pathtracer: &Pathtracer,
    width: u32,
    height: u32,
    iterations: u32,
    tx: &Sender<Duration>,
) -> ImageBuffer {
    let mut rng = SmallRng::from_entropy();
    let mut buffer = ImageBuffer::new(width, height);
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
            print!(
                "{}{}[{}/{}] mean: {:.2}, sdev: {:.2}, eta: {:.2}",
                termion::clear::CurrentLine,
                termion::cursor::Left(u16::MAX),
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
    let obj = obj::obj(str::from_utf8(&fs::read(&args.input).unwrap()).unwrap());
    println!("  Chunks: {}", obj.chunks.len());
    println!("  Vertices: {}", obj.vertices.len());
    println!("  Normals: {}", obj.normals.len());
    println!("  Texcoords: {}", obj.texcoords.len());

    let mtl_path = args.input.parent().unwrap().join(&obj.mtl_lib);
    println!("Loading {}...", mtl_path.display());
    let mtl = mtl::mtl(str::from_utf8(&fs::read(mtl_path).unwrap()).unwrap());
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
        "Rendering {} px x {} px image with {} thread(s) and {} total iteration(s)...",
        args.width, args.height, args.threads, total_iterations,
    );
    let camera = Pinhole::new(&scene.cameras[0], args.width as f32 / args.height as f32);
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
                let i = i.clone();
                let pathtracer = &pathtracer;
                s.spawn(move || {
                    worker_thread(
                        i,
                        &pathtracer,
                        args.width,
                        args.height,
                        args.iterations_per_thread,
                        &tx,
                    )
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
