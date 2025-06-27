use clap::Parser;
use geometry::{aabb::Aabb, bound::geometries_bounding_box, shape::Shape, triangle::Triangle};
use kdtree::{
    KdNode,
    build::build_kdtree,
    format::{write_node_pretty, write_tree_json, write_tree_rust},
    sah::SahCost,
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

fn node_cost(
    cost_traverse: f32,
    cost_intersect: f32,
    empty_factor: f32,
    scene_surface_area: f32,
    boundary: &Aabb,
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
                &left_aabb,
                left,
            );
            let right_cost = node_cost(
                cost_traverse,
                cost_intersect,
                empty_factor,
                scene_surface_area,
                &right_aabb,
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

fn tree_cost(
    geometries: &[Shape],
    node: &KdNode,
    cost_traverse: f32,
    cost_intersect: f32,
    empty_factor: f32,
) -> f32 {
    let bounding_box = geometries_bounding_box(geometries);
    node_cost(
        cost_traverse,
        cost_intersect,
        empty_factor,
        bounding_box.surface_area(),
        &bounding_box,
        node,
    )
}

struct Statistics {
    min: usize,
    max: usize,
    total: usize,
    median: f32,
    mean: f32,
}

impl Statistics {
    fn compute(mut vec: Vec<usize>) -> Self {
        vec.sort_unstable();
        let median = if vec.len() == 1 {
            vec[0] as f32
        } else if vec.len() % 2 == 0 {
            vec[vec.len() / 2] as f32
        } else {
            ((vec[vec.len() / 2] + vec[vec.len() / 2 + 1]) as f32) / 2.0
        };
        let mean = vec.iter().map(|x| *x as f32).sum::<f32>() / vec.len() as f32;
        Self {
            min: *vec.iter().min().unwrap(),
            max: *vec.iter().max().unwrap(),
            total: vec.iter().sum(),
            median,
            mean,
        }
    }
}

struct KdTreeStatistics {
    geometries: usize,
    node_count: usize,
    leaf_count: usize,
    leaf_depth: Statistics,
    leaf_geometries: Statistics,
}

fn statistics(geometries: &[Shape], tree: &KdNode) -> KdTreeStatistics {
    let geometries = geometries.len();
    let node_count = tree.iter_nodes().map(|_| 1).sum();
    let leaf_count = tree.iter_leafs().map(|_| 1).sum();
    let leaf_depth = Statistics::compute(tree.iter_leafs().map(|(depth, _)| depth).collect());
    let leaf_geometries = Statistics::compute(
        tree.iter_leafs()
            .map(|(_, indices)| indices.len())
            .collect(),
    );
    KdTreeStatistics {
        geometries,
        node_count,
        leaf_count,
        leaf_depth,
        leaf_geometries,
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    eprintln!("Reading {:?}...", &args.input);
    let obj = obj::obj(&mut BufReader::new(File::open(args.input)?))?;
    let geometries = obj
        .chunks
        .iter()
        .flat_map(|chunk| {
            chunk.faces.iter().map(|face| {
                assert!(
                    face.points.len() == 3,
                    "Only tringular faces supported but found {} vertices.",
                    face.points.len()
                );
                Triangle {
                    v0: obj.index_vertex(&face.points[0]).into(),
                    v1: obj.index_vertex(&face.points[1]).into(),
                    v2: obj.index_vertex(&face.points[2]).into(),
                }
                .into()
            })
        })
        .collect::<Vec<Shape>>();
    eprintln!("  Geometries: {}", geometries.len());

    eprintln!("Building kdtree...");
    eprintln!("  Traverse cost: {:?}", args.traverse_cost);
    eprintln!("  Intersect cost: {:?}", args.intersect_cost);
    eprintln!("  Empty factor: {:?}", args.empty_factor);

    let start_time = Instant::now();
    let cost = SahCost {
        traverse_cost: args.traverse_cost,
        intersect_cost: args.intersect_cost,
        empty_factor: args.empty_factor,
    };
    let kdtree = build_kdtree(&geometries, &cost);
    let duration = Instant::now().duration_since(start_time);
    let duration = Duration::try_from(duration).unwrap();
    eprintln!("Done in {duration:.3}...");

    let cost = tree_cost(
        &geometries,
        &kdtree,
        args.traverse_cost,
        args.intersect_cost,
        args.empty_factor,
    );
    let stats = statistics(&geometries, &kdtree);
    eprintln!("Tree statistics:");
    eprintln!("  SAH cost: {cost:.3}");
    eprintln!("  Geometries: {}", stats.geometries);
    eprintln!("  Node count: {}", stats.node_count);
    eprintln!("  Leaf count: {}", stats.leaf_count);
    eprintln!("  Leaf depth:");
    eprintln!("    Min: {}", stats.leaf_depth.min);
    eprintln!("    Max: {}", stats.leaf_depth.max);
    eprintln!("    Mean: {}", stats.leaf_depth.mean);
    eprintln!("    Median: {}", stats.leaf_depth.median);
    eprintln!("  Leaf geometry:");
    eprintln!("    Total: {}", stats.leaf_geometries.total);
    eprintln!("    Min: {}", stats.leaf_geometries.min);
    eprintln!("    Max: {}", stats.leaf_geometries.max);
    eprintln!("    Mean: {}", stats.leaf_geometries.mean);
    eprintln!("    Median: {}", stats.leaf_geometries.median);

    if args.json {
        write_tree_json(&mut io::stdout().lock(), &geometries, &kdtree)?;
    } else if args.rust {
        write_tree_rust(&mut io::stdout().lock(), &geometries, &kdtree)?;
    } else {
        write_node_pretty(&mut io::stdout().lock(), &kdtree)?;
    }

    Ok(())
}
