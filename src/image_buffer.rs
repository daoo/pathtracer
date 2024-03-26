use std::{ops::Add, path::Path};

pub struct ImageBuffer(image::ImageBuffer<image::Rgb<f32>, Vec<f32>>);

fn gamma_correct(x: &f32) -> f32 {
    const GAMMA_POWER: f32 = 1.0 / 2.2;
    1.0f32.min(x.powf(GAMMA_POWER))
}

impl ImageBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        ImageBuffer(image::ImageBuffer::new(height, width))
    }

    pub fn ncols(&self) -> u32 {
        self.0.width()
    }
    pub fn nrows(&self) -> u32 {
        self.0.height()
    }

    pub fn div(&self, value: f32) -> Self {
        ImageBuffer(
            image::ImageBuffer::from_vec(
                self.0.width(),
                self.0.height(),
                self.0.iter().map(|e| e / value).collect(),
            )
            .unwrap(),
        )
    }

    pub fn gamma_correct(&self) -> Self {
        ImageBuffer(
            image::ImageBuffer::from_vec(
                self.0.width(),
                self.0.height(),
                self.0.iter().map(gamma_correct).collect(),
            )
            .unwrap(),
        )
    }

    pub fn save_png(self, path: &Path) -> Result<(), image::ImageError> {
        image::DynamicImage::ImageRgb32F(self.0)
            .into_rgb8()
            .save_with_format(path, image::ImageFormat::Png)
    }

    pub fn add_mut(&mut self, x: u32, y: u32, value: [f32; 3]) {
        self.0[(x, y)][0] += value[0];
        self.0[(x, y)][1] += value[1];
        self.0[(x, y)][2] += value[2];
    }
}

impl Add for ImageBuffer {
    type Output = ImageBuffer;

    fn add(self, rhs: Self) -> Self::Output {
        ImageBuffer(
            image::ImageBuffer::from_vec(
                self.0.width(),
                self.0.height(),
                self.0
                    .iter()
                    .zip(rhs.0.iter())
                    .map(|(a, b)| a + b)
                    .collect(),
            )
            .unwrap(),
        )
    }
}
