use clap::Parser;
use image::{ImageFormat, RgbImage};
use kdtree::{build::build_kdtree, build_sah, build_sah::SahKdTreeBuilder};
use scene::{camera::Pinhole, Scene};
use std::{fmt::Display, str::FromStr};
use tracing::{image_buffer::ImageBuffer, raytracer::Raytracer};

#[derive(Clone, Copy, Debug)]
struct Size {
    width: u32,
    height: u32,
}

impl Size {
    fn new(width: u32, height: u32) -> Self {
        Size { width, height }
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

    println!("Rendering...");
    let camera = Pinhole::new(scene.cameras[0].clone(), args.size.width, args.size.height);
    let raytracer = Raytracer {
        scene,
        kdtree,
        camera,
    };
    let mut buffer = ImageBuffer::new(args.size.width, args.size.height);
    raytracer.render(&mut buffer);

    println!("Writing {}...", args.output.display());
    RgbImage::from_raw(buffer.width, buffer.height, buffer.to_rgb8(1))
        .unwrap()
        .save_with_format(&args.output, ImageFormat::Png)
        .unwrap();
}
