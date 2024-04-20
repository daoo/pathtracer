use clap::Parser;
use geometry::{
    aabb::Aabb, axis::Axis, bound::geometries_bounding_box, geometric::Geometric,
    triangle::Triangle,
};
use kdtree::{
    build::build_kdtree,
    build_sah::{self, SahKdTreeBuilder},
    KdNode, KdTree,
};
use std::{fs::File, io::BufReader, time::Instant};
use time::Duration;
use wavefront::obj;

#[derive(Clone, Debug, Parser)]
struct Args {
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    #[arg(short = 'j', long, default_value_t = false)]
    json: bool,
    #[arg(short = 'r', long, default_value_t = false)]
    rust: bool,

    #[arg(long, default_value_t = build_sah::MAX_DEPTH)]
    max_depth: u32,
    #[arg(long, default_value_t = build_sah::TRAVERSE_COST)]
    traverse_cost: f32,
    #[arg(long, default_value_t = build_sah::INTERSECT_COST)]
    intersect_cost: f32,
    #[arg(long, default_value_t = build_sah::EMPTY_FACTOR)]
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
        KdNode::Leaf(indices) => {
            cost_intersect * indices.len() as f32 * boundary.surface_area() / scene_surface_area
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
    let bounding_box = geometries_bounding_box(&kdtree.geometries);
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
        KdNode::Leaf(indices) => println!("{indent}Leaf {indices:?}"),
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

fn print_triangle_array(geometries: &[Geometric]) {
    print!(
        "{:?}",
        geometries
            .iter()
            .map(|t| match t {
                Geometric::Triangle(t) => t.as_arrays(),
                Geometric::AxiallyAlignedTriangle(t) => t.as_arrays(),
            })
            .collect::<Vec<_>>()
    );
}

fn print_node_json(kdtree: &KdNode) {
    match kdtree {
        KdNode::Leaf(indices) => print!("{indices:?}"),
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
        KdNode::Leaf(indices) if indices.is_empty() => print!("KdNode::empty()"),
        KdNode::Leaf(indices) => print!("KdNode::new_leaf(vec!{indices:?})"),
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
    let obj = obj::obj(&mut BufReader::new(File::open(args.input).unwrap()));
    let triangles = obj
        .chunks
        .iter()
        .flat_map(|chunk| {
            chunk.faces.iter().map(|face| {
                Triangle {
                    v0: obj.index_vertex(&face.p0).into(),
                    v1: obj.index_vertex(&face.p1).into(),
                    v2: obj.index_vertex(&face.p2).into(),
                }
                .into()
            })
        })
        .collect::<Vec<Geometric>>();
    eprintln!("  Triangles: {}", triangles.len());

    eprintln!("Building kdtree...");
    eprintln!("  Max depth: {:?}", args.max_depth);
    eprintln!("  Traverse cost: {:?}", args.traverse_cost);
    eprintln!("  Intersect cost: {:?}", args.intersect_cost);
    eprintln!("  Empty factor: {:?}", args.empty_factor);

    let start_time = Instant::now();
    let builder = SahKdTreeBuilder {
        geometries: triangles,
        traverse_cost: args.traverse_cost,
        intersect_cost: args.intersect_cost,
        empty_factor: args.empty_factor,
    };
    let kdtree = build_kdtree(builder, args.max_depth);
    let duration = Instant::now().duration_since(start_time);

    let cost = tree_cost(
        &kdtree,
        args.traverse_cost,
        args.intersect_cost,
        args.empty_factor,
    );

    eprintln!(
        "Done in {:.3} with cost {cost:.3}.",
        Duration::new(duration.as_secs() as i64, duration.as_nanos() as i32)
    );

    if args.json {
        print!("{{\"triangles\": ");
        print_triangle_array(&kdtree.geometries);
        print!(", \"root\": ");
        print_node_json(&kdtree.root);
        println!("}}");
    } else if args.rust {
        print!("let triangles = ");
        print_triangle_array(&kdtree.geometries);
        print!(";\nlet root = ");
        print_node_rust(&kdtree.root);
        println!(";");
        println!("let tree = KdTree {{ triangles, root }};");
    } else {
        print_pretty(0, &kdtree.root);
    }
}
