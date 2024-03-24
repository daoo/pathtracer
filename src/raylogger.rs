use std::{
    fs::File,
    io::{Error, Write},
    path::Path,
};

use geometry::ray::Ray;

pub struct RayLogger {
    file: File,
}

pub struct RayLoggerWithMeta<'a> {
    file: &'a File,
    meta: Vec<u8>,
}

impl RayLogger {
    pub fn create(path: &Path) -> Result<RayLogger, Error> {
        let file = File::create(path)?;
        Ok(RayLogger { file })
    }

    pub fn to_meta(&self) -> RayLoggerWithMeta {
        RayLoggerWithMeta {
            file: &self.file,
            meta: vec![],
        }
    }
}

impl RayLoggerWithMeta<'_> {
    pub fn with_meta<'a>(&'a self, values: &[u16]) -> RayLoggerWithMeta<'a> {
        let bytes: Vec<_> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
        RayLoggerWithMeta {
            file: self.file,
            meta: [self.meta.clone(), bytes].concat(),
        }
    }

    pub fn log(&mut self, ray: &Ray) -> Result<(), Error> {
        self.file.write_all(
            &[
                self.meta.as_slice(),
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
}
