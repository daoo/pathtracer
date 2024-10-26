use std::{
    fs::File,
    io::{BufReader, Error},
    path::{Path, PathBuf},
};

pub mod mtl;
pub mod obj;

pub fn read_obj_and_mtl_with_print_logging(
    path: &Path,
) -> Result<(obj::Obj, mtl::Mtl, PathBuf), Error> {
    println!("Loading {}...", path.display());
    let obj = obj::obj(&mut BufReader::new(File::open(path)?));
    println!("  Chunks: {}", obj.chunks.len());
    println!("  Vertices: {}", obj.vertices.len());
    println!("  Normals: {}", obj.normals.len());
    println!("  Texcoords: {}", obj.texcoords.len());

    let mtl_path = path
        .parent()
        .map_or(obj.mtl_lib.clone(), |p| p.join(&obj.mtl_lib));
    println!("Loading {}...", mtl_path.display());
    let mtl = mtl::mtl(&mut BufReader::new(File::open(&mtl_path)?));
    println!("  Materials: {}", mtl.materials.len());
    println!("  Lights: {}", mtl.lights.len());
    println!("  Cameras: {}", mtl.cameras.len());

    Ok((obj, mtl, mtl_path))
}
