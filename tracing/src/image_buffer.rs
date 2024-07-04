use std::ops::{Add, AddAssign, Index, IndexMut};

use nalgebra::{Vector2, Vector3};

#[derive(Clone)]
pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vector3<f32>>,
}

fn gamma_correct(x: Vector3<f32>) -> Vector3<f32> {
    const GAMMA_POWER: f32 = 1.0 / 2.2;
    x.zip_map(&Vector3::from_element(GAMMA_POWER), |b, e| b.powf(e))
        .inf(&Vector3::from_element(1.0))
}

impl ImageBuffer {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: [Vector3::zeros()].repeat((width * height) as usize),
        }
    }

    #[inline]
    fn index(&self, idx: Vector2<u32>) -> usize {
        (self.width * idx.y + idx.x) as usize
    }

    #[inline]
    pub fn add_at(mut self, at: Vector2<u32>, rhs: &Self) -> Self {
        debug_assert!(at.x + rhs.width <= self.width && at.y + rhs.height <= self.height);
        for y in 0..rhs.height {
            for x in 0..rhs.width {
                self[Vector2::new(at.x + x, at.y + y)] += rhs[Vector2::new(x, y)];
            }
        }
        self
    }

    pub fn to_rgb8(self, iterations: u16) -> Vec<u8> {
        let iterations_inv = 1.0 / iterations as f32;
        self.pixels
            .into_iter()
            .flat_map(|p| -> [u8; 3] {
                (gamma_correct(p * iterations_inv) * 255.0)
                    .map(|c| c.round() as u8)
                    .into()
            })
            .collect()
    }

    #[inline]
    pub fn into_rgba_iter(self, iterations: u16) -> impl Iterator<Item = [u8; 4]> {
        let iterations_inv = 1.0 / iterations as f32;
        self.pixels.into_iter().map(move |p| -> [u8; 4] {
            let p = (gamma_correct(p * iterations_inv) * 255.0).map(|c| c.round() as u8);
            [p.x, p.y, p.z, u8::MAX]
        })
    }
}

impl Index<Vector2<u32>> for ImageBuffer {
    type Output = Vector3<f32>;

    #[inline]
    fn index(&self, index: Vector2<u32>) -> &Self::Output {
        &self.pixels[self.index(index)]
    }
}

impl IndexMut<Vector2<u32>> for ImageBuffer {
    #[inline]
    fn index_mut(&mut self, index: Vector2<u32>) -> &mut Self::Output {
        let index = self.index(index);
        &mut self.pixels[index]
    }
}

impl Add for ImageBuffer {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(self.width == rhs.width && self.height == rhs.height);
        Self {
            width: self.width,
            height: self.height,
            pixels: self
                .pixels
                .into_iter()
                .zip(rhs.pixels.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>(),
        }
    }
}

impl AddAssign for ImageBuffer {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        debug_assert!(self.width == rhs.width && self.height == rhs.height);
        self.pixels
            .iter_mut()
            .zip(rhs.pixels.iter())
            .for_each(|(a, b)| *a += *b);
    }
}
