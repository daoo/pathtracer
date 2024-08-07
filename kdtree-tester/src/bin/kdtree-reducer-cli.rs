use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    time::Instant,
};

use clap::Parser;
use kdtree_tester::checked_intersection::CheckedIntersection;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

use geometry::{geometry::Geometry, intersection::RayIntersection, ray::Ray, triangle::Triangle};
use kdtree::{
    build::build_kdtree, format::write_tree_json, intersection::KdIntersection, sah::SahCost,
    KdNode, MAX_DEPTH,
};
use wavefront::obj;

fn build_test_tree(geometries: &[Geometry]) -> KdNode {
    build_kdtree(geometries, MAX_DEPTH as u32, &SahCost::default())
}

fn verify_removal(
    geometries: &[Geometry],
    ray: &Ray,
    actual: &(Geometry, RayIntersection),
    tree: &KdNode,
) -> bool {
    let intersection = tree.intersect(geometries, ray, 0.0..=f32::MAX).unwrap();
    let same_geometry = geometries[intersection.index as usize] == actual.0;
    let same_intersection = intersection.intersection == actual.1;
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
    let tree = build_test_tree(&reduced);
    verify_removal(&reduced, ray, actual, &tree).then_some(reduced)
}

fn reduce_tree(
    seed: u64,
    intersection: CheckedIntersection,
    geometries: Vec<Geometry>,
) -> (Vec<Geometry>, KdNode) {
    let actual = (
        geometries[intersection.kdtree.as_ref().unwrap().index as usize].clone(),
        intersection.kdtree.as_ref().unwrap().intersection.clone(),
    );
    let mut geometries = geometries;
    geometries.swap(0, intersection.reference.unwrap().index as usize);
    geometries.swap(1, intersection.kdtree.unwrap().index as usize);
    geometries[2..].shuffle(&mut SmallRng::seed_from_u64(seed));
    let mut try_index: usize = 2;
    let mut try_count = geometries.len() - try_index;
    eprintln!(
        "Kept {} with {} geometries left to check.",
        try_index, try_count
    );
    while try_index < geometries.len() {
        try_count = try_count.clamp(1, geometries.len() - try_index);
        eprint!("  Trying to remove {: <5}", try_count);
        let time_before = Instant::now();
        let reduced = try_removing(
            &intersection.ray,
            &actual,
            &geometries,
            try_index,
            try_count,
        );
        let duration = Instant::now().duration_since(time_before).as_micros() as f64 / 1000.0;
        if let Some(reduced) = reduced {
            geometries = reduced;
            try_count = geometries.len() - try_index;
            eprintln!(" Time: {: <8.3} ms. Success!", duration);
            eprintln!(
                "Kept {} with {} geometries left to check.",
                try_index, try_count,
            );
        } else if try_count > 1 {
            try_count /= 2;
            eprintln!(" Time: {: <8.3} ms. Fail!", duration);
        } else {
            try_index += 1;
            try_count = geometries.len() - try_index;
            eprintln!(" Time: {: <8.3} ms. Fail! Keeping 1 geometry.", duration);
            eprintln!(
                "Kept {} with {} geometries left to check.",
                try_index, try_count
            );
        }
    }
    let tree = build_test_tree(&geometries);
    (geometries, tree)
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

    /// Output ray fail binary data path
    #[arg(short = 'f', long)]
    fail: Option<std::path::PathBuf>,

    /// Seed for random generator used to shuffle input geometry
    #[arg(short = 's', long, required = true)]
    seed: u64,
}

fn main() {
    let args = Args::parse();

    let intersection = CheckedIntersection {
        ray: Ray::new(
            [3.897963, 0.24242611, -4.203691].into(),
            [-13.897963, 9.757574, 14.2036915].into(),
        ),
        reference: Some(KdIntersection::new(
            7589,
            RayIntersection {
                t: 0.0004729527,
                u: 0.09395919,
                v: 0.47453666,
            },
        )),
        kdtree: Some(KdIntersection::new(
            5556,
            RayIntersection {
                t: 0.05429069,
                u: 0.2189177,
                v: 0.74337834,
            },
        )),
    };
    eprintln!("Seed: {}", args.seed);
    eprintln!("Testing with failed intersection:");
    eprintln!("  {:?}", &intersection.ray);
    eprintln!("  Expected: {:?}", &intersection.reference);
    eprintln!("    Actual: {:?}", &intersection.kdtree);

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

    if let Some(path) = args.fail {
        eprintln!("Writing test ray to {:?}...", path);
        let file = File::create(path).unwrap();
        let mut buf = BufWriter::new(file);
        buf.write_all(&intersection.as_bytes(1)).unwrap();
    }

    eprintln!("Reducing tree...");
    let (geometries, tree) = reduce_tree(args.seed, intersection, geometries);

    eprintln!("Writing reduced tree to {:?}...", args.output);
    write_tree_json(
        &mut BufWriter::new(File::create(args.output).unwrap()),
        &geometries,
        &tree,
    )
    .unwrap();
}
