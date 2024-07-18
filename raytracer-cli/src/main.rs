use clap::Parser;
use glam::UVec2;
use image::{ImageFormat, RgbImage};
use kdtree::{build::build_kdtree, sah::SahCost, MAX_DEPTH};
use scene::{camera::Pinhole, Scene};
use std::{fmt::Display, str::FromStr};
use tracing::{image_buffer::ImageBuffer, raytracer::Raytracer};

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
struct Args {
    /// Wavefront OBJ input path
    #[arg(short, long, required = true)]
    input: std::path::PathBuf,

    /// PNG output path
    #[arg(short, long, required = true)]
    output: std::path::PathBuf,
    /// Image size in pixels
    #[arg(short, long, default_value_t = Size::new(512, 512))]
    size: Size,

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

    println!("Rendering...");
    let camera = Pinhole::new(scene.cameras[0].clone(), args.size.as_uvec2());
    let raytracer = Raytracer {
        scene,
        kdtree,
        camera,
    };
    let mut buffer = ImageBuffer::new(args.size.as_uvec2());
    raytracer.render(&mut buffer);

    println!("Writing {}...", args.output.display());
    RgbImage::from_raw(buffer.size.x, buffer.size.y, buffer.to_rgb8(1))
        .unwrap()
        .save_with_format(&args.output, ImageFormat::Png)
        .unwrap();
}
