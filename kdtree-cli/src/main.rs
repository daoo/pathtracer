use clap::Parser;
use geometry::{
    aabb::Aabb, bound::geometries_bounding_box, geometric::Geometric, triangle::Triangle,
};
use kdtree::{
    build::build_kdtree,
    build_sah::{self, SahKdTreeBuilder},
    format::{write_tree_json, write_tree_pretty, write_tree_rust},
    KdNode, KdTree,
};
use std::{
    fs::File,
    io::{self, BufReader},
    time::Instant,
};
use time::Duration;
use wavefront::obj;

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Wavefront OBJ input path
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    /// Output JSON to standard output
    #[arg(short = 'j', long, default_value_t = false)]
    json: bool,
    /// Output Rust to standard output
    #[arg(short = 'r', long, default_value_t = false)]
    rust: bool,

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
        write_tree_json(&mut io::stdout().lock(), &kdtree).unwrap();
    } else if args.rust {
        write_tree_rust(&mut io::stdout().lock(), &kdtree).unwrap();
    } else {
        write_tree_pretty(&mut io::stdout().lock(), &kdtree).unwrap();
    }
}
