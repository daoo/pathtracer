use pathtracer::geometry::triangle::*;
use pathtracer::kdtree::*;
use pathtracer::kdtree::build::*;
use pathtracer::wavefront::*;
use std::env;
use std::fs;
use std::path::Path;
use std::str;

fn print_triangles(all_triangles: &[Triangle], triangles: &[Triangle]) -> Vec<usize> {
    triangles
        .iter()
        .map(|t1| all_triangles
             .iter()
             .position(|t2| t1 == t2)
             .unwrap())
        .collect()
}

fn print(depth: usize, all_triangles: &[Triangle], kdtree: &KdNode) {
    let indent = "  ".repeat(depth);
    match kdtree {
        KdNode::Leaf(triangles) => println!("{}Leaf: {:?}", indent, print_triangles(all_triangles, triangles)),
        KdNode::Node{ plane, left, right } => {
            println!("{}Split: {:?}", "  ".repeat(depth), plane);
            print(depth + 1, all_triangles, left);
            print(depth + 1, all_triangles, right);
        },
    }
}

fn main() {
    for arg in env::args().skip(1) {
        let path = Path::new(&arg);
        let bytes = fs::read(path).unwrap();
        let input = str::from_utf8(&bytes).unwrap();
        let obj = obj::obj(&input);
        let mut triangles: Vec<Triangle> = Vec::new();
        for chunk in &obj.chunks {
            for face in &chunk.faces {
                triangles.push(
                    Triangle{
                        v0: obj.index_vertex(&face.p0),
                        v1: obj.index_vertex(&face.p1),
                        v2: obj.index_vertex(&face.p2),
                    }
                )
            }
        }

        let kdtree = build_kdtree_median(5, &triangles);

        print(0, &triangles, &kdtree.root);
    }
}
