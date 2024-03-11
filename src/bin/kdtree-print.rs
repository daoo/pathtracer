use clap::Parser;
use pathtracer::{
    geometry::triangle::Triangle,
    kdtree::{build_naive::build_kdtree_median, KdNode},
    wavefront::obj,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, required = true)]
    input: std::path::PathBuf,
    #[arg(short, long, default_value_t = 3)]
    max_depth: u32,
}

fn print(depth: usize, kdtree: &KdNode) {
    let indent = "  ".repeat(depth);
    match kdtree {
        KdNode::Leaf(triangle_indices) => println!("{}Leaf: {:?}", indent, triangle_indices),
        KdNode::Node { plane, left, right } => {
            println!("{}Split: {:?}", "  ".repeat(depth), plane);
            print(depth + 1, left);
            print(depth + 1, right);
        }
    }
}

fn main() {
    let args = Args::parse();
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

    let kdtree = build_kdtree_median(args.max_depth, triangles);
    print(0, &kdtree.root);
}
