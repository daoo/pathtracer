use clap::Parser;
use kdtree::build_sah;
use kdtree::{build::build_kdtree, build_sah::SahKdTreeBuilder};
use pathtracer::raytracer::Raytracer;
use pathtracer::{camera::Pinhole, image_buffer::ImageBuffer, scene::Scene};
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
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

    println!("Rendering...");
    let camera = Pinhole::new(&scene.cameras[0], args.size.aspect_ratio());
    let raytracer = Raytracer {
        scene,
        kdtree,
        camera,
    };
    let mut buffer = ImageBuffer::new(args.size.width, args.size.height);
    raytracer.render(&mut buffer);

    println!("Writing {}...", args.output.display());
    buffer.gamma_correct().save_png(&args.output).unwrap();
}
