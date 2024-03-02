extern crate pathtracer;

use std::env;
use std::fs;
use std::str;
use std::path::Path;
use pathtracer::wavefront::{mtl, obj};

fn main() {
    for arg in env::args().skip(1) {
        let path = Path::new(&arg);
        let bytes = fs::read(path).unwrap();
        let input = str::from_utf8(&bytes).unwrap();
        match path.extension().and_then(|s| s.to_str()) {
            Some("obj") => println!("{:#?}", obj::obj(&input)),
            Some("mtl") => println!("{:#?}", mtl::mtl(&input)),
            _ => panic!("Unexpected file extension for {:?}", path),
        }
    }
}
