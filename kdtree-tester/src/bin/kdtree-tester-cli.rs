use clap::Parser;
use glam::UVec2;
use kdtree::{build::build_kdtree, sah::SahCost, MAX_DEPTH};
use kdtree_tester::ray_bouncer::RayBouncer;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use scene::{camera::Pinhole, Scene};
use std::{
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
    str::{self, FromStr},
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
    /// Wavefront OBJ input path
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    /// Output ray fail binary data path
    #[arg(short = 'o', long)]
    output: Option<std::path::PathBuf>,
    /// Image size in pixels
    #[arg(short, long, default_value_t = Size::new(512, 512))]
    size: Size,
    /// Max number of bounces to test
    #[arg(short, long, default_value_t = 10)]
    bounces: u32,

    /// Maximum kd-tree depth
    #[arg(long, default_value_t = MAX_DEPTH as u32)]
    max_depth: u32,
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

fn main() {
    let args = Args::parse();
    let scene = Scene::read_obj_file_with_print_logging(&args.input);

    println!("Building kdtree...");
    let kdtree = build_kdtree(
        scene.collect_geometries_as_vec(),
        args.max_depth,
        &SahCost {
            traverse_cost: args.traverse_cost,
            intersect_cost: args.intersect_cost,
            empty_factor: args.empty_factor,
        },
    );

    println!(
        "Testing up to {} rays...",
        args.size.x * args.size.y * args.bounces
    );
    let camera = Pinhole::new(scene.cameras[0].clone(), args.size.as_uvec2());
    let bouncer = RayBouncer {
        scene,
        kdtree,
        camera,
        size: args.size.as_uvec2(),
        bounces: args.bounces,
    };

    let xs = 0..args.size.x;
    let ys = 0..args.size.y;
    let pixels = ys
        .flat_map(|y| xs.clone().map(move |x| (x, y)))
        .collect::<Vec<_>>();
    let pixel_count = pixels.len();
    let fails = pixels
        .into_par_iter()
        .enumerate()
        .filter_map(|(i, pixel)| {
            let result = bouncer.bounce_pixel(pixel);
            if let Some(fail) = &result {
                eprintln!(
                    "Fail on pixel {} x {} ({} / {})",
                    pixel.0, pixel.1, i, pixel_count
                );
                eprintln!("  {:?}", fail.ray);
                eprintln!("  Expected: {:?}", fail.reference);
                eprintln!("    Actual: {:?}", fail.kdtree);
            }
            result
        })
        .collect::<Vec<_>>();
    println!("Found {} fails", fails.len());

    if let Some(path) = args.output {
        println!("Writing failed rays to {:?}...", path);
        let mut logger = BufWriter::new(File::create(path).unwrap());
        fails.iter().enumerate().for_each(|(i, fail)| {
            logger.write_all(&fail.as_bytes(i as u16)).unwrap();
        });
    }
}
