use std::{
    fs::File,
    io::{BufWriter, Error, Write},
    path::Path,
};

use geometry::ray::Ray;

pub enum RayLoggerWriter {
    None,
    File(BufWriter<File>),
}

impl RayLoggerWriter {
    pub const fn empty() -> Self {
        Self::None
    }

    pub fn create<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        if !cfg!(feature = "ray_logging") {
            return Ok(Self::None);
        }
        let file = File::create(path)?;
        let buf = BufWriter::new(file);
        Ok(Self::File(buf))
    }

    pub const fn with_iteration(&mut self, iteration: u16) -> RayLoggerWithIteration<'_> {
        RayLoggerWithIteration {
            writer: self,
            iteration,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn write(
        &mut self,
        ray: &Ray,
        shadow: u8,
        intersect: u8,
        iteration: u16,
        pixel_x: u16,
        pixel_y: u16,
        bounces: u8,
    ) -> Result<(), Error> {
        if !cfg!(feature = "ray_logging") {
            return Ok(());
        }

        if let Self::File(buf) = self {
            let mut bytes = [0u8; 33];
            bytes[0..2].copy_from_slice(&iteration.to_le_bytes());
            bytes[2..4].copy_from_slice(&pixel_x.to_le_bytes());
            bytes[4..6].copy_from_slice(&pixel_y.to_le_bytes());
            bytes[6..7].copy_from_slice(&bounces.to_le_bytes());
            bytes[7..8].copy_from_slice(&shadow.to_le_bytes());
            bytes[8..9].copy_from_slice(&intersect.to_le_bytes());
            bytes[9..13].copy_from_slice(&ray.origin.x.to_le_bytes());
            bytes[13..17].copy_from_slice(&ray.origin.y.to_le_bytes());
            bytes[17..21].copy_from_slice(&ray.origin.z.to_le_bytes());
            bytes[21..25].copy_from_slice(&ray.direction.x.to_le_bytes());
            bytes[25..29].copy_from_slice(&ray.direction.y.to_le_bytes());
            bytes[29..33].copy_from_slice(&ray.direction.z.to_le_bytes());
            buf.write_all(&bytes)
        } else {
            Ok(())
        }
    }
}

pub struct RayLoggerWithIteration<'a> {
    pub writer: &'a mut RayLoggerWriter,
    pub iteration: u16,
}

impl RayLoggerWithIteration<'_> {
    pub const fn with_pixel(&mut self, x: u16, y: u16) -> RayLoggerWithIterationAndPixel<'_> {
        RayLoggerWithIterationAndPixel {
            writer: self.writer,
            iteration: self.iteration,
            pixel_x: x,
            pixel_y: y,
        }
    }
}

pub struct RayLoggerWithIterationAndPixel<'a> {
    pub writer: &'a mut RayLoggerWriter,
    pub iteration: u16,
    pub pixel_x: u16,
    pub pixel_y: u16,
}

impl RayLoggerWithIterationAndPixel<'_> {
    pub fn log_ray(&mut self, ray: &Ray, bounces: u8, intersect: bool) -> Result<(), Error> {
        let shadow = false;
        self.writer.write(
            ray,
            u8::from(shadow),
            u8::from(intersect),
            self.iteration,
            self.pixel_x,
            self.pixel_y,
            bounces,
        )
    }

    pub fn log_shadow(&mut self, ray: &Ray, bounces: u8, intersect: bool) -> Result<(), Error> {
        let shadow = true;
        self.writer.write(
            ray,
            u8::from(shadow),
            u8::from(intersect),
            self.iteration,
            self.pixel_x,
            self.pixel_y,
            bounces,
        )
    }
}
