use std::{
    fs::File,
    io::{Error, Write},
    path::Path,
};

use geometry::ray::Ray;

pub enum RayLogger {
    None(),
    File(File),
}

impl RayLogger {
    pub fn empty() -> RayLogger {
        RayLogger::None()
    }

    pub fn create<P>(path: P) -> Result<RayLogger, Error>
    where
        P: AsRef<Path>,
    {
        if !cfg!(feature = "ray_logging") {
            return Ok(RayLogger::None());
        }
        let file = File::create(path)?;
        Ok(RayLogger::File(file))
    }

    fn log_internal(
        file: &mut File,
        ray: &Ray,
        infinite: bool,
        iteration: u16,
        pixel: (u16, u16),
    ) -> Result<(), Error> {
        let mut bytes: [u8; 31] = Default::default();
        bytes[0..2].copy_from_slice(&iteration.to_le_bytes());
        bytes[2..4].copy_from_slice(&pixel.0.to_le_bytes());
        bytes[4..6].copy_from_slice(&pixel.1.to_le_bytes());
        bytes[6..7].copy_from_slice(&[infinite as u8]);
        bytes[7..11].copy_from_slice(&ray.origin.x.to_le_bytes());
        bytes[11..15].copy_from_slice(&ray.origin.y.to_le_bytes());
        bytes[15..19].copy_from_slice(&ray.origin.z.to_le_bytes());
        bytes[19..23].copy_from_slice(&ray.direction.x.to_le_bytes());
        bytes[23..27].copy_from_slice(&ray.direction.y.to_le_bytes());
        bytes[27..31].copy_from_slice(&ray.direction.z.to_le_bytes());
        file.write_all(&bytes)
    }

    pub fn log_infinite(
        &mut self,
        ray: &Ray,
        iteration: u16,
        pixel: (u16, u16),
    ) -> Result<(), Error> {
        if !cfg!(feature = "ray_logging") {
            return Ok(());
        }
        match self {
            RayLogger::None() => Ok(()),
            RayLogger::File(file) => RayLogger::log_internal(file, ray, true, iteration, pixel),
        }
    }

    pub fn log_finite(
        &mut self,
        ray: &Ray,
        iteration: u16,
        pixel: (u16, u16),
    ) -> Result<(), Error> {
        if !cfg!(feature = "ray_logging") {
            return Ok(());
        }
        match self {
            RayLogger::None() => Ok(()),
            RayLogger::File(file) => RayLogger::log_internal(file, ray, false, iteration, pixel),
        }
    }
}
