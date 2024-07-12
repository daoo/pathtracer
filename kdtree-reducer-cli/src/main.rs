use std::{
    fs::File,
    io::{BufReader, BufWriter},
    time::Instant,
};

use clap::Parser;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

use geometry::{geometry::Geometry, intersection::RayIntersection, ray::Ray, triangle::Triangle};
use kdtree::{build::build_kdtree, build_sah::SahKdTreeBuilder, format::write_tree_json, KdTree};
use wavefront::obj;

fn build_test_tree(geometries: Vec<Geometry>) -> kdtree::KdTree {
    build_kdtree(
        SahKdTreeBuilder {
            traverse_cost: 2.0,
            intersect_cost: 1.0,
            empty_factor: 0.8,
            geometries,
        },
        20,
    )
}

fn verify_removal(ray: &Ray, actual: &(Geometry, RayIntersection), tree: &KdTree) -> bool {
    let intersection = tree.intersect(ray, 0.0..=f32::MAX).unwrap();
    let same_geometry = tree.geometries[intersection.0 as usize] == actual.0;
    let same_intersection = intersection.1 == actual.1;
    same_geometry && same_intersection
}

fn try_removing(
    ray: &Ray,
    actual: &(Geometry, RayIntersection),
    geometries: &[Geometry],
    try_index: usize,
    try_count: usize,
) -> Option<Vec<Geometry>> {
    let mut reduced = Vec::with_capacity(geometries.len() - try_count);
    reduced.extend_from_slice(&geometries[0..try_index]);
    reduced.extend_from_slice(&geometries[try_index + try_count..]);
    let tree = build_test_tree(reduced);
    verify_removal(ray, actual, &tree).then_some(tree.geometries)
}

fn reduce_tree(
    ray: &Ray,
    geometries: Vec<Geometry>,
    expected_intersection: &(usize, RayIntersection),
    actual_intersection: &(usize, RayIntersection),
) -> KdTree {
    let actual_geometry = geometries[actual_intersection.0].clone();
    let actual = (actual_geometry, actual_intersection.1);
    let mut geometries = geometries;
    geometries.swap(0, expected_intersection.0);
    geometries.swap(1, actual_intersection.0);
    geometries[2..].shuffle(&mut SmallRng::from_entropy());
    let mut try_index: usize = 2;
    let mut try_count = geometries.len() - try_index;
    eprintln!(
        "Kept {} with {} geometries left to check.",
        try_index, try_count
    );
    while try_index < geometries.len() {
        let remaining = geometries.len() - try_index;
        try_count = try_count.clamp(1, remaining);
        eprint!("  Trying to remove {: <5}", try_count);
        let time_before = Instant::now();
        let reduced = try_removing(ray, &actual, &geometries, try_index, try_count);
        let duration = Instant::now().duration_since(time_before).as_micros() as f64 / 1000.0;
        if let Some(reduced) = reduced {
            geometries = reduced;
            eprintln!(" Time: {: <8.3} ms. Success!", duration);
            eprintln!(
                "Kept {} with {} geometries left to check.",
                try_index, try_count
            );
        } else if try_count > 1 {
            try_count /= 2;
            eprintln!(" Time: {: <8.3} ms. Fail!", duration);
        } else {
            try_index += 1;
            try_count = remaining;
            eprintln!(" Time: {: <8.3} ms. Fail! Keeping 1 geometry.", duration);
            eprintln!(
                "Kept {} with {} geometries left to check.",
                try_index, try_count
            );
        }
    }
    build_test_tree(geometries)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Wavefront OBJ input path
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    /// Output reduced kd-tree JSON data path
    #[arg(short = 'o', long, required = true)]
    output: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let ray = Ray::new(
        [3.897963, 0.24242611, -4.203691].into(),
        [-13.897963, 9.757574, 14.2036915].into(),
    );
    let expected_intersection = (
        7589,
        RayIntersection {
            t: 0.0004729527,
            u: 0.09395919,
            v: 0.47453666,
        },
    );
    let actual_intersection = (
        5556,
        RayIntersection {
            t: 0.05429069,
            u: 0.2189177,
            v: 0.74337834,
        },
    );

    eprintln!("Loading {}...", args.input.display());
    let obj = obj::obj(&mut BufReader::new(File::open(&args.input).unwrap()));
    eprintln!("  Chunks: {}", obj.chunks.len());
    eprintln!("  Vertices: {}", obj.vertices.len());
    eprintln!("  Normals: {}", obj.normals.len());
    eprintln!("  Texcoords: {}", obj.texcoords.len());

    eprintln!("Collecting geometries...");
    let geometries = obj
        .chunks
        .iter()
        .flat_map(|chunk| {
            chunk.faces.iter().map(|face| {
                if face.points.len() != 3 {
                    panic!(
                        "Only tringular faces supported but found {} vertices.",
                        face.points.len()
                    );
                }
                Triangle {
                    v0: obj.index_vertex(&face.points[0]).into(),
                    v1: obj.index_vertex(&face.points[1]).into(),
                    v2: obj.index_vertex(&face.points[2]).into(),
                }
                .into()
            })
        })
        .collect::<Vec<_>>();
    eprintln!("  Geometries: {}", geometries.len());

    eprintln!("Testing with:");
    eprintln!("  {:?}", &ray);
    eprintln!("  Expected: {:?}", &expected_intersection);
    eprintln!("    Actual: {:?}", &actual_intersection);

    let tree = reduce_tree(
        &ray,
        geometries,
        &expected_intersection,
        &actual_intersection,
    );

    write_tree_json(
        &mut BufWriter::new(File::create(args.output).unwrap()),
        &tree,
    )
    .unwrap();
}
