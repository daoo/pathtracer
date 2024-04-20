use clap::Parser;
use kdtree::build_sah;
use kdtree::{build::build_kdtree, build_sah::SahKdTreeBuilder};
use pathtracer::raytracer::Raytracer;
use pathtracer::{camera::Pinhole, image_buffer::ImageBuffer, scene::Scene};
use std::fs::File;
use std::io::BufReader;
use wavefront::{mtl, obj};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, required = true)]
    input: std::path::PathBuf,

    #[arg(short, long, required = true)]
    output: std::path::PathBuf,
    #[arg(short, long, default_value_t = 512)]
    width: u32,
    #[arg(short, long, default_value_t = 512)]
    height: u32,

    #[arg(long, default_value_t = build_sah::MAX_DEPTH)]
    max_depth: u32,
    #[arg(long, default_value_t = build_sah::TRAVERSE_COST)]
    traverse_cost: f32,
    #[arg(long, default_value_t = build_sah::INTERSECT_COST)]
    intersect_cost: f32,
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
    let camera = Pinhole::new(&scene.cameras[0], args.width as f32 / args.height as f32);
    let raytracer = Raytracer {
        scene,
        kdtree,
        camera,
    };
    let mut buffer = ImageBuffer::new(args.width, args.height);
    raytracer.render(&mut buffer);

    println!("Writing {}...", args.output.display());
    buffer.gamma_correct().save_png(&args.output).unwrap();
}
