extern crate pathtracer;

use pathtracer::geometry::triangle::*;
use pathtracer::kdtree::build::*;
use pathtracer::wavefront::*;
use std::env;
use std::fs;
use std::path::Path;
use std::str;

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
                        uv0: obj.index_texcoord(&face.p0),
                        uv1: obj.index_texcoord(&face.p1),
                        uv2: obj.index_texcoord(&face.p2),
                        n0: obj.index_normal(&face.p0),
                        n1: obj.index_normal(&face.p1),
                        n2: obj.index_normal(&face.p2),
                    }
                )
            }
        }

        let kdtree = build_kdtree_center(3, &triangles);

        println!("{:#?}", kdtree);
    }
}
