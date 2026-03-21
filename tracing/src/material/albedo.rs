use glam::{Vec2, Vec3};
use image::Rgb32FImage;

#[derive(Clone, Debug)]
pub enum AlbedoSource {
    Color(Vec3),
    Texture(Rgb32FImage),
}

fn wrap01(x: f32) -> f32 {
    let y = x - x.floor();
    if y == 1.0 { 0.0 } else { y }
}

impl AlbedoSource {
    pub const ZERO: Self = Self::Color(Vec3::ZERO);

    pub fn get(&self, uv: Vec2) -> Vec3 {
        match self {
            AlbedoSource::Color(albedo) => *albedo,
            AlbedoSource::Texture(texture) => {
                let px = (texture.width() as f32 * wrap01(uv.x)).floor();
                let py = (texture.height() as f32 * wrap01(uv.y)).floor();
                Vec3::from(texture.get_pixel(px as u32, py as u32).0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use image::Rgb;

    #[test]
    fn texture_repeat() {
        let texture = Rgb32FImage::from_fn(2, 2, |x, y| Rgb([x as f32 / 2.0, y as f32 / 2.0, 0.0]));
        let albedo_source = AlbedoSource::Texture(texture);
        let uv = Vec2::new(0.2, 0.3);

        let actual1 = albedo_source.get(uv);
        let actual2 = albedo_source.get(uv + Vec2::ONE);

        assert_ulps_eq!(actual1, actual2);
    }
}
