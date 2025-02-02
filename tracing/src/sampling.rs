use glam::{Vec2, Vec3};
use rand::{rngs::SmallRng, Rng};

#[inline]
pub fn uniform_sample_unit_square(rng: &mut SmallRng) -> Vec2 {
    Vec2::new(rng.random(), rng.random())
}

pub fn uniform_sample_unit_sphere(rng: &mut SmallRng) -> Vec3 {
    let z = rng.random_range(-1.0..1.0);
    let a = rng.random_range(0.0..std::f32::consts::TAU);
    let r = (1.0f32 - z * z).sqrt();
    let (a_sin, a_cos) = a.sin_cos();
    let x = r * a_cos;
    let y = r * a_sin;
    Vec3::new(x, y, z)
}

// fn uniform_sample_hemisphere(rng: &mut SmallRng) -> Vec3 {
//     let r = uniform_sample_unit_square(rng);

//     let a = 2.0 * (r.y * (1.0 - r.y)).sqrt();
//     let b = std::f32::consts::TAU * r.x;
//     Vec3::new(a * b.cos(), a * b.sin(), (1.0 - 2.0 * r.y).abs())
// }

fn concentric_sample_unit_disk(rng: &mut SmallRng) -> Vec2 {
    let x = rng.random_range(-1.0f32..=1.0f32);
    let y = rng.random_range(-1.0f32..=1.0f32);
    if x == 0.0 && y == 0.0 {
        return Vec2::ZERO;
    }

    let (r, theta) = match (x, y) {
        (x, y) if x >= -y && x > y => (x, y / x),
        (x, y) if x >= -y => (y, 2.0 - x / y),
        (x, y) if x <= y => (-x, 4.0 + y / x),
        (x, y) => (-y, 6.0 - x / y),
    };

    r * Vec2::from((theta * std::f32::consts::FRAC_PI_4).sin_cos())
}

pub fn cosine_sample_hemisphere(rng: &mut SmallRng) -> Vec3 {
    let ret = concentric_sample_unit_disk(rng);
    let z = (0.0f32.max(1.0 - ret.x * ret.x - ret.y * ret.y)).sqrt();
    Vec3::new(ret.x, ret.y, z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_uniform_sample_unit_square() {
        let mut rng = SmallRng::from_os_rng();
        for _ in 0..1000 {
            let point = uniform_sample_unit_square(&mut rng);
            assert!(point.cmpge(Vec2::new(0.0, 0.0)).all());
            assert!(point.cmple(Vec2::new(1.0, 1.0)).all());
        }
    }

    #[test]
    fn test_uniform_sample_unit_sphere() {
        let mut rng = SmallRng::from_os_rng();
        for _ in 0..1000 {
            let point = uniform_sample_unit_sphere(&mut rng);
            let error = point.length();
            assert!((0.9999999..=1.0000001).contains(&error), "{}", error);
        }
    }

    // #[test]
    // fn test_uniform_sample_hemisphere() {
    //     let mut rng = SmallRng::from_os_rng();
    //     for _ in 0..1000 {
    //         let point = uniform_sample_hemisphere(&mut rng);
    //         let error = (point.norm_squared() - 1.0).abs();
    //         // TODO: Check which hemisphere
    //         assert!(error <= 1e-6, "{}", error);
    //     }
    // }

    #[test]
    fn test_concentric_sample_disk() {
        let mut rng = SmallRng::from_os_rng();
        for _ in 0..1000 {
            let point = concentric_sample_unit_disk(&mut rng);
            assert!(point.length_squared() <= 1.0, "{}", point.length_squared());
        }
    }

    #[test]
    fn cosine_cosine_sample_hemisphere() {
        let mut rng = SmallRng::from_os_rng();
        for _ in 0..1000 {
            let point = cosine_sample_hemisphere(&mut rng);
            let error = (point.length_squared() - 1.0).abs();
            assert!(point.z >= 0.0 && point.z <= 1.0);
            assert!(error <= 1e-6, "{}", error);
        }
    }
}
