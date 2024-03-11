use nalgebra::{DMatrix, Vector3};
use std::{
    ops::{Add, Index, IndexMut},
    path::Path,
};

pub struct ImageBuffer(DMatrix<Vector3<f32>>);

fn gamma_correct(x: f32) -> f32 {
    const GAMMA_POWER: f32 = 1.0 / 2.2;
    1.0f32.min(x.powf(GAMMA_POWER))
}

fn to_rgb_u8(vector: &Vector3<f32>) -> image::Rgb<u8> {
    image::Rgb([
        (gamma_correct(vector.x) * 255.0) as u8,
        (gamma_correct(vector.y) * 255.0) as u8,
        (gamma_correct(vector.z) * 255.0) as u8,
    ])
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        ImageBuffer(DMatrix::zeros(height, width))
    }

    pub fn ncols(&self) -> usize {
        self.0.ncols()
    }
    pub fn nrows(&self) -> usize {
        self.0.nrows()
    }

    pub fn div(&self, value: f32) -> Self {
        ImageBuffer(self.0.map(|e| e / value))
    }

    pub fn save_with_format(
        &self,
        path: &Path,
        format: image::ImageFormat,
    ) -> Result<(), image::ImageError> {
        let mut image = image::RgbImage::new(self.0.ncols() as u32, self.0.nrows() as u32);
        for y in 0..self.0.nrows() {
            for x in 0..self.0.ncols() {
                image[(x as u32, y as u32)] = to_rgb_u8(&self.0[(y, x)]);
            }
        }
        image.save_with_format(path, format)
    }
}

impl Add for ImageBuffer {
    type Output = ImageBuffer;

    fn add(self, rhs: Self) -> Self::Output {
        ImageBuffer(self.0 + rhs.0)
    }
}

impl Index<(usize, usize)> for ImageBuffer {
    type Output = Vector3<f32>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[(index.1, index.0)]
    }
}

impl IndexMut<(usize, usize)> for ImageBuffer {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[(index.1, index.0)]
    }
}
