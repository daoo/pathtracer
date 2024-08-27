use std::ops::{Add, AddAssign, Index, IndexMut};

use glam::{UVec2, Vec3};

#[derive(Clone)]
pub struct ImageBuffer {
    pub size: UVec2,
    pixels: Vec<Vec3>,
}

fn gamma_correct(x: Vec3) -> Vec3 {
    x.powf(1.0 / 2.2).min(Vec3::ONE)
}

impl ImageBuffer {
    #[inline]
    pub fn new(size: UVec2) -> Self {
        Self {
            size,
            pixels: [Vec3::ZERO].repeat((size.x * size.y) as usize),
        }
    }

    #[inline]
    fn index(&self, idx: UVec2) -> usize {
        (self.size.x * idx.y + idx.x) as usize
    }

    pub fn to_rgb8(self, iterations: u16) -> Vec<u8> {
        let iterations_inv = 1.0 / iterations as f32;
        self.pixels
            .into_iter()
            .flat_map(|p| -> [u8; 3] {
                let color = (gamma_correct(p * iterations_inv) * 255.0).round();
                [color.x as u8, color.y as u8, color.z as u8]
            })
            .collect()
    }

    #[inline]
    pub fn into_rgba_iter(self, iterations: u16) -> impl Iterator<Item = [u8; 4]> {
        let iterations_inv = 1.0 / iterations as f32;
        self.pixels.into_iter().map(move |p| -> [u8; 4] {
            let p = (gamma_correct(p * iterations_inv) * 255.0).round();
            [p.x as u8, p.y as u8, p.z as u8, u8::MAX]
        })
    }

    #[inline]
    pub fn coordinates(&self) -> impl Iterator<Item = UVec2> {
        let size_x = self.size.x;
        let size_y = self.size.y;
        (0..size_y).flat_map(move |y| (0..size_x).map(move |x| UVec2::new(x, y)))
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (UVec2, &mut Vec3)> {
        self.coordinates().zip(self.pixels.iter_mut())
    }
}

impl Index<UVec2> for ImageBuffer {
    type Output = Vec3;

    #[inline]
    fn index(&self, index: UVec2) -> &Self::Output {
        &self.pixels[self.index(index)]
    }
}

impl IndexMut<UVec2> for ImageBuffer {
    #[inline]
    fn index_mut(&mut self, index: UVec2) -> &mut Self::Output {
        let index = self.index(index);
        &mut self.pixels[index]
    }
}

impl Add for ImageBuffer {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(self.size.x == rhs.size.x && self.size.y == rhs.size.y);
        Self {
            size: self.size,
            pixels: self
                .pixels
                .into_iter()
                .zip(rhs.pixels)
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>(),
        }
    }
}

impl AddAssign for ImageBuffer {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        debug_assert!(self.size.x == rhs.size.x && self.size.y == rhs.size.y);
        self.pixels
            .iter_mut()
            .zip(rhs.pixels.into_iter())
            .for_each(|(a, b)| *a += b);
    }
}
