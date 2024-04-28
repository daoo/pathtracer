use std::{
    fs::File,
    io::{BufWriter, Error, Write},
    path::Path,
};

use geometry::ray::Ray;

pub enum RayLogger {
    None(),
    File(BufWriter<File>),
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
        let buf = BufWriter::new(file);
        Ok(RayLogger::File(buf))
    }

    fn log_internal(
        buf: &mut BufWriter<File>,
        ray: &Ray,
        infinite: bool,
        iteration: u16,
        pixel: (u16, u16),
        bounces: u8,
    ) -> Result<(), Error> {
        let mut bytes = [0u8; 32];
        bytes[0..2].copy_from_slice(&iteration.to_le_bytes());
        bytes[2..4].copy_from_slice(&pixel.0.to_le_bytes());
        bytes[4..6].copy_from_slice(&pixel.1.to_le_bytes());
        bytes[6..7].copy_from_slice(&bounces.to_le_bytes());
        bytes[7..8].copy_from_slice(&(infinite as u8).to_le_bytes());
        bytes[8..12].copy_from_slice(&ray.origin.x.to_le_bytes());
        bytes[12..16].copy_from_slice(&ray.origin.y.to_le_bytes());
        bytes[16..20].copy_from_slice(&ray.origin.z.to_le_bytes());
        bytes[20..24].copy_from_slice(&ray.direction.x.to_le_bytes());
        bytes[24..28].copy_from_slice(&ray.direction.y.to_le_bytes());
        bytes[28..32].copy_from_slice(&ray.direction.z.to_le_bytes());
        buf.write_all(&bytes)
    }

    pub fn log_infinite(
        &mut self,
        ray: &Ray,
        iteration: u16,
        pixel: (u16, u16),
        bounces: u8,
    ) -> Result<(), Error> {
        if !cfg!(feature = "ray_logging") {
            return Ok(());
        }
        match self {
            RayLogger::None() => Ok(()),
            RayLogger::File(buf) => {
                RayLogger::log_internal(buf, ray, true, iteration, pixel, bounces)
            }
        }
    }

    pub fn log_finite(
        &mut self,
        ray: &Ray,
        iteration: u16,
        pixel: (u16, u16),
        bounces: u8,
    ) -> Result<(), Error> {
        if !cfg!(feature = "ray_logging") {
            return Ok(());
        }
        match self {
            RayLogger::None() => Ok(()),
            RayLogger::File(buf) => {
                RayLogger::log_internal(buf, ray, false, iteration, pixel, bounces)
            }
        }
    }
}
