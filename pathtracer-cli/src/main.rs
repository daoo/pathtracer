use clap::Parser;
use pathtracer::{camera::Pinhole, image_buffer::ImageBuffer, raylogger::RayLogger, scene::Scene};
use rand::{rngs::SmallRng, SeedableRng};
use rayon::prelude::*;
use std::{fs, io::Write, str, sync::mpsc};
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
    max_bounces: u32,
    #[arg(short = 'n', long, default_value_t = 16)]
    iterations: u32,
    #[arg(short, long, default_value_t = 1)]
    threads: u32,
}

struct Work<'a> {
    width: u32,
    height: u32,
    max_bounces: u32,
    scene: &'a Scene,
    pinhole: &'a Pinhole,
}

fn create_ray_logger<'a>(thread: u32) -> RayLogger<'a> {
    if cfg!(feature = "ray_logging") {
        let path = format!("/tmp/raylog{}.bin", thread);
        RayLogger::create(path).unwrap()
    } else {
        RayLogger::None()
    }
}

fn worker_thread(
    thread: u32,
    work: &Work,
    iterations: u32,
    tx: &mpsc::Sender<time::Duration>,
) -> ImageBuffer {
    let mut rng = SmallRng::from_entropy();
    let mut buffer = ImageBuffer::new(work.width, work.height);
    let mut ray_logger = create_ray_logger(thread);
    for iteration in 0..iterations {
        let t1 = time::Instant::now();
        let mut ray_logger = ray_logger.with_meta(&[iteration as u16]);
        pathtracer::pathtracer::render(
            &mut ray_logger,
            work.max_bounces,
            work.scene,
            work.pinhole,
            &mut buffer,
            &mut rng,
        );
        let t2 = time::Instant::now();
        let duration = t2 - t1;
        tx.send(duration).unwrap();
    }
    buffer
}

fn printer_thread(threads: u32, iterations: u32, rx: mpsc::Receiver<time::Duration>) {
    let mut total = 0.0;
    let mut total_squared = 0.0;
    let mut completed = 0;
    loop {
        let duration = rx.recv().unwrap();
        if duration.is_zero() {
            println!();
            println!(
                "Total time: {:.2}",
                time::Duration::seconds_f64(total) / (threads as f64)
            );
            return;
        }
        let seconds = duration.as_seconds_f64();
        total += seconds;
        total_squared += seconds * seconds;
        completed += 1;
        if completed % threads == 0 {
            let mean = total / completed as f64;
            let sdev = ((total_squared / completed as f64) - mean * mean).sqrt();
            let eta = ((iterations - completed) as f64 * mean) / (threads as f64);
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
    let scene = Scene::from_wavefront(&obj, &mtl);
    println!("Triangles: {}", scene.triangle_normals.len());

    let pinhole = Pinhole::new(&scene.cameras[0], args.width as f32 / args.height as f32);

    println!(
        "Rendering {} px x {} px image with {} iteration(s)...",
        args.width, args.height, args.iterations
    );

    let work = Work {
        width: args.width,
        height: args.height,
        max_bounces: args.max_bounces,
        scene: &scene,
        pinhole: &pinhole,
    };
    let iterations_per_thread = args.iterations / args.threads;
    let (tx, rx) = mpsc::channel();
    let buffers = (0..args.threads)
        .into_par_iter()
        .map(|i| worker_thread(i, &work, iterations_per_thread, &tx));
    let printer = std::thread::spawn(move || printer_thread(args.threads, args.iterations, rx));
    let buffer = buffers.reduce_with(|a, b| a.add(&b)).unwrap();
    tx.send(time::Duration::ZERO).unwrap();
    printer.join().unwrap();

    println!("Writing {}...", args.output.display());
    buffer
        .div(args.iterations as f32)
        .gamma_correct()
        .save_png(&args.output)
        .unwrap();
}
