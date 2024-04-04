use clap::Parser;
use pathtracer::{camera::Pinhole, image_buffer::ImageBuffer, raylogger::RayLogger, scene::Scene};
use rand::{rngs::SmallRng, SeedableRng};
use std::{
    fs,
    io::Write,
    str,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};
use time::{Duration, Instant};
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

struct Work {
    max_bounces: u16,
    width: u32,
    height: u32,
    pinhole: Pinhole,
    scene: Arc<Scene>,
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
    work: &Arc<Work>,
    iterations: u32,
    tx: &Sender<Duration>,
) -> ImageBuffer {
    let mut rng = SmallRng::from_entropy();
    let mut buffer = ImageBuffer::new(work.width, work.height);
    let mut ray_logger = create_ray_logger(thread);
    for iteration in 0..iterations {
        let t1 = Instant::now();
        pathtracer::pathtracer::render(
            iteration as u16,
            &mut ray_logger,
            work.max_bounces,
            &work.scene,
            &work.pinhole,
            &mut buffer,
            &mut rng,
        );
        let t2 = Instant::now();
        let duration = t2 - t1;
        tx.send(duration).unwrap();
    }
    buffer
}

fn new_worker(
    thread: u32,
    work: Arc<Work>,
    iterations: u32,
    tx: Sender<Duration>,
) -> JoinHandle<ImageBuffer> {
    thread::spawn(move || worker_thread(thread, &work, iterations, &tx))
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
            print!(
                "{}{}[{}/{}] mean: {:.2}, sdev: {:.2}, eta: {:.2}",
                termion::clear::CurrentLine,
                termion::cursor::Left(u16::MAX),
                completed,
                iterations,
                Duration::seconds_f64(mean),
                Duration::seconds_f64(sdev),
                Duration::seconds_f64(eta)
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
    let mtl_path = args.input.parent().unwrap().join(&obj.mtl_lib);
    println!("Loading {}...", mtl_path.display());
    let mtl = mtl::mtl(str::from_utf8(&fs::read(mtl_path).unwrap()).unwrap());
    println!("Building scene...");
    let scene = Scene::from_wavefront(
        &obj,
        &mtl,
        args.max_depth,
        args.traverse_cost,
        args.intersect_cost,
        args.empty_factor,
    );
    println!("Triangles: {}", scene.triangle_normals.len());

    let pinhole = Pinhole::new(&scene.cameras[0], args.width as f32 / args.height as f32);

    let total_iterations = args.threads * args.iterations_per_thread;
    println!(
        "Rendering {} px x {} px image with {} thread(s) and {} total iteration(s)...",
        args.width, args.height, args.threads, total_iterations,
    );

    let t1 = Instant::now();

    let work = Arc::new(Work {
        max_bounces: args.max_bounces,
        width: args.width,
        height: args.height,
        pinhole,
        scene: Arc::new(scene),
    });
    let (tx, rx) = mpsc::channel();
    let threads = (0..args.threads)
        .map(|i| new_worker(i, work.clone(), args.iterations_per_thread, tx.clone()))
        .collect::<Vec<_>>();
    let printer = thread::spawn(move || printer_thread(args.threads, total_iterations, &rx));
    let buffer = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .reduce(|a, b| a.add(&b))
        .unwrap();
    drop(tx);
    printer.join().unwrap();

    let t2 = Instant::now();
    println!("Total time: {:.2}", t2 - t1);

    println!("Writing {}...", args.output.display());
    buffer
        .div(total_iterations as f32)
        .gamma_correct()
        .save_png(&args.output)
        .unwrap();
}
