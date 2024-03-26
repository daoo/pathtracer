use std::path::Path;

pub struct ImageBuffer(image::ImageBuffer<image::Rgb<f32>, Vec<f32>>);

fn gamma_correct(x: f32) -> f32 {
    const GAMMA_POWER: f32 = 1.0 / 2.2;
    1.0f32.min(x.powf(GAMMA_POWER))
}

impl ImageBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        ImageBuffer(image::ImageBuffer::new(width, height))
    }

    pub fn size(&self) -> [u32; 2] {
        [self.0.width(), self.0.height()]
    }

    pub fn width(&self) -> u32 {
        self.0.width()
    }

    pub fn height(&self) -> u32 {
        self.0.height()
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.0.width() as f32 / self.0.height() as f32
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

    pub fn add(&self, rhs: &Self) -> Self {
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

    pub fn add_mut(&mut self, rhs: &Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(a, b)| *a += *b);
    }

    pub fn add_pixel_mut(&mut self, x: u32, y: u32, value: [f32; 3]) {
        self.0[(x, y)][0] += value[0];
        self.0[(x, y)][1] += value[1];
        self.0[(x, y)][2] += value[2];
    }

    pub fn gamma_correct(&self) -> Self {
        ImageBuffer(
            image::ImageBuffer::from_vec(
                self.0.width(),
                self.0.height(),
                self.0.iter().map(|x| gamma_correct(*x)).collect(),
            )
            .unwrap(),
        )
    }

    pub fn to_rgba8(self) -> Vec<u8> {
        image::DynamicImage::ImageRgb32F(self.0).to_rgba8().to_vec()
    }

    pub fn save_png(self, path: &Path) -> Result<(), image::ImageError> {
        image::DynamicImage::ImageRgb32F(self.0)
            .into_rgb8()
            .save_with_format(path, image::ImageFormat::Png)
    }
}
