use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
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
    let same_geometry = tree.geometries[intersection.index as usize] == actual.0;
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
    let tree = build_test_tree(reduced);
    verify_removal(ray, actual, &tree).then_some(tree.geometries)
}

fn reduce_tree(seed: u64, intersection: &CheckedIntersection, geometries: Vec<Geometry>) -> KdTree {
    let actual_geometry = geometries[intersection.kdtree.0].clone();
    let actual = (actual_geometry, intersection.kdtree.1);
    let mut geometries = geometries;
    geometries.swap(0, intersection.reference.0);
    geometries.swap(1, intersection.kdtree.0);
    geometries[2..].shuffle(&mut SmallRng::seed_from_u64(seed));
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

    /// Output ray fail binary data path
    #[arg(short = 'f', long)]
    fail: Option<std::path::PathBuf>,

    /// Seed for random generator used to shuffle input geometry
    #[arg(short = 's', long, required = true)]
    seed: u64,
}

#[derive(Debug, Clone, Copy)]
struct CheckedIntersection {
    pub ray: Ray,
    pub reference: (usize, RayIntersection),
    pub kdtree: (usize, RayIntersection),
}

impl CheckedIntersection {
    fn as_bytes(&self, iteration: u16) -> [u8; 50] {
        let mut bytes = [0u8; 50];
        let ray = self.ray.extended(self.kdtree.1.t);
        let correct_point = self.ray.param(self.reference.1.t);
        let actual_point = self.ray.param(self.kdtree.1.t);
        bytes[0..2].copy_from_slice(&iteration.to_le_bytes());
        bytes[2..6].copy_from_slice(&ray.origin.x.to_le_bytes());
        bytes[6..10].copy_from_slice(&ray.origin.y.to_le_bytes());
        bytes[10..14].copy_from_slice(&ray.origin.z.to_le_bytes());
        bytes[14..18].copy_from_slice(&ray.direction.x.to_le_bytes());
        bytes[18..22].copy_from_slice(&ray.direction.y.to_le_bytes());
        bytes[22..26].copy_from_slice(&ray.direction.z.to_le_bytes());
        bytes[26..30].copy_from_slice(&correct_point.x.to_le_bytes());
        bytes[30..34].copy_from_slice(&correct_point.y.to_le_bytes());
        bytes[34..38].copy_from_slice(&correct_point.z.to_le_bytes());
        bytes[38..42].copy_from_slice(&actual_point.x.to_le_bytes());
        bytes[42..46].copy_from_slice(&actual_point.y.to_le_bytes());
        bytes[46..50].copy_from_slice(&actual_point.z.to_le_bytes());
        bytes
    }
}

fn main() {
    let args = Args::parse();

    let intersection = CheckedIntersection {
        ray: Ray::new(
            [3.897963, 0.24242611, -4.203691].into(),
            [-13.897963, 9.757574, 14.2036915].into(),
        ),
        reference: (
            7589,
            RayIntersection {
                t: 0.0004729527,
                u: 0.09395919,
                v: 0.47453666,
            },
        ),
        kdtree: (
            5556,
            RayIntersection {
                t: 0.05429069,
                u: 0.2189177,
                v: 0.74337834,
            },
        ),
    };
    eprintln!("Testing with:");
    eprintln!("  {:?}", &intersection.ray);
    eprintln!("  Expected: {:?}", &intersection.reference);
    eprintln!("    Actual: {:?}", &intersection.kdtree);
    if let Some(path) = args.fail {
        eprintln!("Writing test ray to {:?}...", path);
        let file = File::create(path).unwrap();
        let mut buf = BufWriter::new(file);
        buf.write_all(&intersection.as_bytes(1)).unwrap();
    }

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

    let tree = reduce_tree(args.seed, &intersection, geometries);

    write_tree_json(
        &mut BufWriter::new(File::create(args.output).unwrap()),
        &tree,
    )
    .unwrap();
}
