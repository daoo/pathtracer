use std::fmt::Display;

use clap::{Parser, ValueEnum};
use geometry::{axis::Axis, triangle::Triangle};
use kdtree::{
    build::build_kdtree, build_median::MedianKdTreeBuilder, build_sah::SahKdTreeBuilder, KdNode,
};
use wavefront::obj;

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
struct Args {
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    #[arg(short = 'j', long, default_value_t = false)]
    json: bool,
    #[arg(short = 'r', long, default_value_t = false)]
    rust: bool,

    #[arg(short = 'm', long, default_value_t = KdTreeMethod::Sah)]
    method: KdTreeMethod,

    #[arg(long, default_value_t = 20)]
    max_depth: u32,
    #[arg(long, default_value_t = 2.0)]
    traverse_cost: f32,
    #[arg(long, default_value_t = 1.0)]
    intersect_cost: f32,
    #[arg(long, default_value_t = 0.8)]
    empty_factor: f32,
}

fn print_pretty(depth: usize, kdtree: &KdNode) {
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
            print_pretty(depth + 1, left);
            print_pretty(depth + 1, right);
        }
    }
}

fn print_triangles_json(triangles: &[Triangle]) {
    print!("[");
    let mut skip_first_separator = true;
    for triangle in triangles {
        if skip_first_separator {
            skip_first_separator = false;
        } else {
            print!(", ");
        }
        print!(
            "[[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]]",
            triangle.v0.x,
            triangle.v0.y,
            triangle.v0.z,
            triangle.v1.x,
            triangle.v1.y,
            triangle.v1.z,
            triangle.v2.x,
            triangle.v2.y,
            triangle.v2.z,
        );
    }
    print!("]");
}

fn print_node_json(kdtree: &KdNode) {
    match kdtree {
        KdNode::Leaf(triangle_indices) => print!("{:?}", triangle_indices),
        KdNode::Node { plane, left, right } => {
            print!(
                "{{\"axis\": \"{:?}\", \"distance\": {}, \"left\": ",
                plane.axis, plane.distance
            );
            print_node_json(left);
            print!(", \"right\": ");
            print_node_json(right);
            print!("}}");
        }
    }
}

fn print_node_rust(kdtree: &KdNode) {
    match kdtree {
        KdNode::Leaf(triangle_indices) if triangle_indices.is_empty() => print!("KdNode::empty()"),
        KdNode::Leaf(triangle_indices) => print!("KdNode::new_leaf(vec!{:?})", triangle_indices),
        KdNode::Node { plane, left, right } => {
            let aap_new = match plane.axis {
                Axis::X => "Aap::new_x",
                Axis::Y => "Aap::new_y",
                Axis::Z => "Aap::new_z",
            };

            print!("KdNode::new_node({}({:?}), ", aap_new, plane.distance);
            print_node_rust(left);
            print!(", ");
            print_node_rust(right);
            print!(")");
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
                v0: obj.index_vertex(&face.p0).into(),
                v1: obj.index_vertex(&face.p1).into(),
                v2: obj.index_vertex(&face.p2).into(),
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
        KdTreeMethod::Median => {
            let builder = MedianKdTreeBuilder { triangles };
            build_kdtree(builder, args.max_depth)
        }
        KdTreeMethod::Sah => {
            let builder = SahKdTreeBuilder {
                triangles,
                traverse_cost: args.traverse_cost,
                intersect_cost: args.intersect_cost,
                empty_factor: args.empty_factor,
            };
            build_kdtree(builder, args.max_depth)
        }
    };
    let t2 = time::Instant::now();
    let duration = t2 - t1;

    let cost = kdtree.cost(args.traverse_cost, args.intersect_cost, args.empty_factor);

    eprintln!("Done in {:.3} with cost {:.3}.", duration, cost);

    if args.json {
        print!("{{\"triangles\": ");
        print_triangles_json(&kdtree.triangles);
        print!(", \"root\": ");
        print_node_json(&kdtree.root);
        println!("}}");
    } else if args.rust {
        println!(
            "let triangles = {:?};",
            kdtree
                .triangles
                .iter()
                .map(Triangle::as_arrays)
                .collect::<Vec<_>>()
        );
        print!("let root = ");
        print_node_rust(&kdtree.root);
        println!(";");
        println!("let tree = KdTree {{ triangles, root }};");
    } else {
        print_pretty(0, &kdtree.root);
    }
}
