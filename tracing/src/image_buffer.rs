use std::ops::{Add, AddAssign, Index, IndexMut};

use glam::{UVec2, Vec3};

#[derive(Clone)]
pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec3>,
}

fn gamma_correct(x: Vec3) -> Vec3 {
    x.powf(1.0 / 2.2).min(Vec3::ONE)
}

impl ImageBuffer {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: [Vec3::ZERO].repeat((width * height) as usize),
        }
    }

    #[inline]
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    #[inline]
    fn index(&self, idx: UVec2) -> usize {
        (self.width * idx.y + idx.x) as usize
    }

    #[inline]
    pub fn add_at(mut self, at: UVec2, rhs: &Self) -> Self {
        debug_assert!(at.x + rhs.width <= self.width && at.y + rhs.height <= self.height);
        for y in 0..rhs.height {
            for x in 0..rhs.width {
                self[UVec2::new(at.x + x, at.y + y)] += rhs[UVec2::new(x, y)];
            }
        }
        self
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
        debug_assert!(self.width == rhs.width && self.height == rhs.height);
        Self {
            width: self.width,
            height: self.height,
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
        debug_assert!(self.width == rhs.width && self.height == rhs.height);
        self.pixels
            .iter_mut()
            .zip(rhs.pixels.iter())
            .for_each(|(a, b)| *a += *b);
    }
}
