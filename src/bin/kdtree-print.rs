use std::fmt::Display;

use clap::{Parser, ValueEnum};
use pathtracer::{
    geometry::triangle::Triangle,
    kdtree::{build_sah::build_kdtree_sah, KdNode, build_naive::build_kdtree_median},
    wavefront::obj,
};

#[derive(Clone, Debug, ValueEnum)]
enum KdTreeMethod {
    Median,
    Sah,
}

impl Display for KdTreeMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KdTreeMethod::Median => write!(f, "median"),
            KdTreeMethod::Sah => write!(f, "sah"),
        }
    }
}

#[derive(Clone, Debug, Parser)]
#[command(about = "test", long_about = None)]
struct Args {
    #[arg(short = 'i', long, required = true, help = "test")]
    input: std::path::PathBuf,

    #[arg(short = 'm', long, default_value_t = KdTreeMethod::Median)]
    method: KdTreeMethod,

    #[arg(short = 'd', long, default_value_t = 3)]
    max_depth: u32,

    #[arg(short = 't', long, default_value_t = 0.1)]
    traverse_cost: f32,

    #[arg(short = 'c', long, default_value_t = 0.3)]
    intersect_cost: f32,
}

fn print(depth: usize, kdtree: &KdNode) {
    let indent = "  ".repeat(depth);
    match kdtree {
        KdNode::Leaf(triangle_indices) => println!("{}Leaf {:?}", indent, triangle_indices),
        KdNode::Node { plane, left, right } => {
            println!(
                "{}Split {:?} {}",
                "  ".repeat(depth),
                plane.axis,
                plane.distance
            );
            print(depth + 1, left);
            print(depth + 1, right);
        }
    }
}

fn main() {
    let args = Args::parse();
    eprintln!("Reading {:?}...", &args.input);
    let bytes = std::fs::read(args.input).unwrap();
    let input = std::str::from_utf8(&bytes).unwrap();
    let obj = obj::obj(input);
    let mut triangles: Vec<Triangle> = Vec::new();
    for chunk in &obj.chunks {
        for face in &chunk.faces {
            triangles.push(Triangle {
                v0: obj.index_vertex(&face.p0),
                v1: obj.index_vertex(&face.p1),
                v2: obj.index_vertex(&face.p2),
            })
        }
    }

    eprintln!(
        "Building {} kdtree for {} triangle(s) with max depth {}...",
        args.method,
        triangles.len(),
        args.max_depth
    );

    let t1 = time::Instant::now();
    let kdtree = match args.method {
        KdTreeMethod::Median => build_kdtree_median(args.max_depth, triangles),
        KdTreeMethod::Sah => build_kdtree_sah(args.max_depth, triangles),
    };
    let t2 = time::Instant::now();
    let duration = t2 - t1;

    let cost = kdtree.cost(args.traverse_cost, args.intersect_cost);

    eprintln!("Done in {:.3} with cost {:.3}.", duration, cost);

    print(0, &kdtree.root);
}
