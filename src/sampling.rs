use nalgebra::{vector, Vector2, Vector3};
use rand::rngs::SmallRng;
use rand::Rng;

// fn uniform_sample_unit_square(rng: &mut SmallRng) -> Vector2<f32> {
//   vector![rng.gen(), rng.gen()]
// }

pub fn uniform_sample_unit_sphere(rng: &mut SmallRng) -> Vector3<f32> {
    let z = rng.gen_range(-1.0..1.0);
    let a = rng.gen_range(0.0..std::f32::consts::TAU);
    let r = (1.0f32 - z * z).sqrt();
    let x = r * a.cos();
    let y = r * a.sin();
    vector![x, y, z]
}

// fn uniform_sample_hemisphere(rng: &mut SmallRng) -> Vector3<f32> {
//   let r = uniform_sample_unit_square(rng);

//   let a = 2.0 * (r.y * (1.0 - r.y)).sqrt();
//   let b = std::f32::consts::TAU * r.x;
//   vector![
//       a * b.cos(),
//       a * b.sin(),
//       (1.0 - 2.0 * r.y).abs()
//   ]
// }

fn concentric_sample_unit_disk(rng: &mut SmallRng) -> Vector2<f32> {
    let x = rng.gen_range(-1.0..1.0);
    let y = rng.gen_range(-1.0..1.0);
    if x == 0.0 && y == 0.0 {
        return Vector2::zeros();
    }

    let (r, theta) = match (x, y) {
        (x, y) if x >= -y && x > y => (x, y / x),
        (x, y) if x >= -y => (y, 2.0 - x / y),
        (x, y) if x <= y => (-x, 4.0 + y / x),
        (x, y) => (-y, 6.0 - x / y),
    };

    let theta = theta * std::f32::consts::FRAC_PI_4;
    vector![r * theta.cos(), r * theta.sin()]
}

pub fn cosine_sample_hemisphere(rng: &mut SmallRng) -> Vector3<f32> {
    let ret = concentric_sample_unit_disk(rng);
    let z = (0.0f32.max(1.0 - ret.x * ret.x - ret.y * ret.y)).sqrt();
    vector![ret.x, ret.y, z]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    // #[test]
    // fn test_uniform_sample_unit_square() {
    //     let mut rng = SmallRng::from_entropy();
    //     for _ in 0..1000 {
    //         let point = uniform_sample_unit_square(&mut rng);
    //         assert!(point >= vector![0.0, 0.0]);
    //         assert!(point <= vector![1.0, 1.0]);
    //     }
    // }

    #[test]
    fn test_uniform_sample_unit_sphere() {
        let mut rng = SmallRng::from_entropy();
        for _ in 0..1000 {
            let point = uniform_sample_unit_sphere(&mut rng);
            let error = (point.norm_squared() - 1.0).abs();
            assert!(error <= 1e-6, "{}", error);
        }
    }

    // #[test]
    // fn test_uniform_sample_hemisphere() {
    //     let mut rng = SmallRng::from_entropy();
    //     for _ in 0..1000 {
    //         let point = uniform_sample_hemisphere(&mut rng);
    //         let error = (point.norm_squared() - 1.0).abs();
    //         // TODO: Check which hemisphere
    //         assert!(error <= 1e-6, "{}", error);
    //     }
    // }

    #[test]
    fn test_concentric_sample_disk() {
        let mut rng = SmallRng::from_entropy();
        for _ in 0..1000 {
            let point = concentric_sample_unit_disk(&mut rng);
            assert!(point.norm_squared() <= 1.0, "{}", point.norm_squared());
        }
    }

    #[test]
    fn cosine_cosine_sample_hemisphere() {
        let mut rng = SmallRng::from_entropy();
        for _ in 0..1000 {
            let point = cosine_sample_hemisphere(&mut rng);
            let error = (point.norm_squared() - 1.0).abs();
            // TODO: Check which hemisphere
            assert!(error <= 1e-6, "{}", error);
        }
    }
}