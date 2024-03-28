use std::{
    fs::File,
    io::{Error, Write},
    path::Path,
};

use geometry::ray::Ray;

pub enum RayLogger<'a> {
    None(),
    File(File),
    WithMeta(&'a mut File, Vec<u8>),
}

impl RayLogger<'_> {
    pub fn empty() -> RayLogger<'static> {
        RayLogger::None()
    }

    pub fn create<P>(path: P) -> Result<RayLogger<'static>, Error>
    where
        P: AsRef<Path>,
    {
        if !cfg!(feature = "ray_logging") {
            return Ok(RayLogger::None());
        }
        let file = File::create(path)?;
        Ok(RayLogger::File(file))
    }

    pub fn with_meta(&mut self, values: &[u16]) -> RayLogger {
        if !cfg!(feature = "ray_logging") {
            return RayLogger::None();
        }
        let bytes: Vec<_> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
        match self {
            RayLogger::None() => RayLogger::None(),
            RayLogger::File(file) => RayLogger::WithMeta(file, bytes),
            RayLogger::WithMeta(file, meta) => {
                RayLogger::WithMeta(file, [meta.clone(), bytes].concat())
            }
        }
    }

    fn log_internal(file: &mut File, ray: &Ray, meta: &[u8]) -> Result<(), Error> {
        file.write_all(
            &[
                meta,
                &ray.origin.x.to_le_bytes(),
                &ray.origin.y.to_le_bytes(),
                &ray.origin.z.to_le_bytes(),
                &ray.direction.x.to_le_bytes(),
                &ray.direction.y.to_le_bytes(),
                &ray.direction.z.to_le_bytes(),
            ]
            .concat(),
        )
    }

    pub fn log(&mut self, ray: &Ray) -> Result<(), Error> {
        if !cfg!(feature = "ray_logging") {
            return Ok(());
        }
        match self {
            RayLogger::None() => Ok(()),
            RayLogger::File(file) => RayLogger::log_internal(file, ray, &[]),
            RayLogger::WithMeta(file, meta) => RayLogger::log_internal(file, ray, &meta),
        }
    }
}
