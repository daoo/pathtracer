pub mod wavefront;

use std::env;
use std::fs;
use std::str;
use std::path::Path;
use wavefront::{mtl, obj};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let bytes = fs::read(path).unwrap();
    let input = str::from_utf8(&bytes).unwrap();
    match path.extension().and_then(|s| s.to_str()) {
        Some("obj") => println!("{:?}", obj::obj(&input)),
        Some("mtl") => println!("{:?}", mtl::mtl(&input)),
        _ => panic!("Unexpected file extension for {:?}", path),
    }
}
