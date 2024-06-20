use std::{env, fs::File, io::BufReader, path::Path};
use wavefront::{mtl, obj};

fn main() {
    for arg in env::args().skip(1) {
        let path = Path::new(&arg);
        let file = File::open(path).unwrap();
        let mut buf = BufReader::new(file);
        match path.extension().and_then(|s| s.to_str()) {
            Some("obj") => println!("{:#?}", obj::obj(&mut buf)),
            Some("mtl") => println!("{:#?}", mtl::mtl(&mut buf)),
            _ => panic!("Unexpected file extension for {path:?}"),
        }
    }
}
