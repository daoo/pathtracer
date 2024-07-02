use std::{
    ops::{Add, Div},
    path::Path,
};

use nalgebra::{DMatrix, Vector2, Vector3};

#[derive(Clone)]
pub struct ImageBuffer(DMatrix<Vector3<f32>>);

fn gamma_correct(x: f32) -> f32 {
    const GAMMA_POWER: f32 = 1.0 / 2.2;
    1.0f32.min(x.powf(GAMMA_POWER))
}

impl ImageBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        ImageBuffer(DMatrix::from_element(
            height as usize,
            width as usize,
            Vector3::zeros(),
        ))
    }

    pub fn width(&self) -> u32 {
        self.0.ncols() as u32
    }

    pub fn height(&self) -> u32 {
        self.0.nrows() as u32
    }

    pub fn add_mut(&mut self, rhs: &Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(a, b)| *a += *b);
    }

    pub fn add_pixel_mut(&mut self, x: u32, y: u32, p: Vector3<f32>) {
        self.0[(y as usize, x as usize)] += p;
    }

    pub fn add_at(mut self, at: Vector2<u32>, rhs: &Self) -> Self {
        for y in 0..rhs.0.nrows() {
            for x in 0..rhs.0.ncols() {
                self.0[(at.y as usize + y, at.x as usize + x)] += rhs.0[(y, x)];
            }
        }
        self
    }

    pub fn gamma_correct(self) -> Self {
        ImageBuffer(self.0.map(|p| p.map(gamma_correct)))
    }

    pub fn to_rgb8(self) -> Vec<u8> {
        self.0
            .transpose()
            .into_iter()
            .flat_map(|p| {
                [
                    (p.x * 255.0).round() as u8,
                    (p.y * 255.0).round() as u8,
                    (p.z * 255.0).round() as u8,
                ]
            })
            .collect()
    }

    pub fn to_rgba8(self) -> Vec<u8> {
        self.0
            .transpose()
            .into_iter()
            .flat_map(|p| {
                [
                    (p.x * 255.0).round() as u8,
                    (p.y * 255.0).round() as u8,
                    (p.z * 255.0).round() as u8,
                    u8::MAX,
                ]
            })
            .collect()
    }

    pub fn save_png(self, path: &Path) -> Result<(), image::ImageError> {
        image::RgbImage::from_raw(self.width(), self.height(), self.to_rgb8())
            .unwrap()
            .save_with_format(path, image::ImageFormat::Png)
    }
}

impl Add for ImageBuffer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + &rhs.0)
    }
}

impl Div<f32> for ImageBuffer {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0.map(|p| p / rhs))
    }
}
