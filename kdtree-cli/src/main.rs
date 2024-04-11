use std::fmt::Display;

use clap::{Parser, ValueEnum};
use geometry::{aabb::Aabb, axis::Axis, bound::geometries_bounding_box, triangle::Triangle};
use kdtree::{
    build::build_kdtree, build_median::MedianKdTreeBuilder, build_sah::SahKdTreeBuilder, KdNode,
    KdTree,
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

fn node_cost(
    cost_traverse: f32,
    cost_intersect: f32,
    empty_factor: f32,
    scene_surface_area: f32,
    boundary: Aabb,
    node: &KdNode,
) -> f32 {
    match node {
        KdNode::Leaf(triangle_indices) => {
            cost_intersect * triangle_indices.len() as f32 * boundary.surface_area()
                / scene_surface_area
        }
        KdNode::Node { plane, left, right } => {
            let split_cost = boundary.surface_area() / scene_surface_area;
            let (left_aabb, right_aabb) = boundary.split(plane);
            let left_cost = node_cost(
                cost_traverse,
                cost_intersect,
                empty_factor,
                scene_surface_area,
                left_aabb,
                left,
            );
            let right_cost = node_cost(
                cost_traverse,
                cost_intersect,
                empty_factor,
                scene_surface_area,
                right_aabb,
                right,
            );
            let node_cost = cost_traverse + split_cost + left_cost + right_cost;
            let factor = if left.is_empty() || right.is_empty() {
                empty_factor
            } else {
                1.0
            };
            factor * node_cost
        }
    }
}

pub fn tree_cost(
    kdtree: &KdTree,
    cost_traverse: f32,
    cost_intersect: f32,
    empty_factor: f32,
) -> f32 {
    let bounding_box = geometries_bounding_box(&kdtree.triangles);
    node_cost(
        cost_traverse,
        cost_intersect,
        empty_factor,
        bounding_box.surface_area(),
        bounding_box,
        kdtree.root.as_ref(),
    )
}

fn print_pretty(depth: usize, kdtree: &KdNode) {
    let indent = "  ".repeat(depth);
    match kdtree {
        KdNode::Leaf(triangle_indices) => println!("{indent}Leaf {triangle_indices:?}"),
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
        KdNode::Leaf(triangle_indices) => print!("{triangle_indices:?}"),
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
        KdNode::Leaf(triangle_indices) => print!("KdNode::new_leaf(vec!{triangle_indices:?})"),
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
            });
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

    let cost = tree_cost(
        &kdtree,
        args.traverse_cost,
        args.intersect_cost,
        args.empty_factor,
    );

    eprintln!("Done in {duration:.3} with cost {cost:.3}.");

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
                .into_iter()
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
