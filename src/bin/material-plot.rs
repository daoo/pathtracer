use std::sync::Arc;

use nalgebra::vector;
use pathtracer::material::*;
use plotters::prelude::*;
use rand::{rngs::SmallRng, SeedableRng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("/tmp/test.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Material wo", ("sans-serif", 50).into_font())
        .build_cartesian_3d(-1.0..1.0, -1.0..1.0, -1.0..1.0)?;

    let diffuse_reflective = DiffuseReflectiveMaterial {
        reflectance: vector![1., 1., 1.],
    };
    let specular_reflective = SpecularReflectiveMaterial {
        reflectance: vector![1., 1., 1.],
    };
    let specular_refractive = SpecularRefractiveMaterial {
        index_of_refraction: 1.55,
    };
    let fresnel_blend = FresnelBlendMaterial {
        refraction: Arc::new(specular_refractive.clone()),
        reflection: Arc::new(diffuse_reflective.clone()),
        r0: 0.0,
    };
    let fresnel_blend = BlendMaterial {
        first: Arc::new(specular_refractive),
        second: Arc::new(diffuse_reflective),
        factor: 1.0,
    };
    let material = fresnel_blend;
    let wi = vector![-0.5, 0.5, 0.0].normalize();
    let n = vector![0.0, 1.0, 0.0];
    let mut rng = SmallRng::from_entropy();

    let data = (0..1000).map(|_| {
        let sample = material.sample(&wi, &n, &mut rng);
        (sample.wo.x as f64, sample.wo.y as f64, sample.wo.z as f64)
    });

    chart
        .draw_series(PointSeries::<_, _, Circle<_, _>, _>::new(data, 2.0, &RED))
        .unwrap();

    chart.configure_axes().draw()?;

    Ok(())
}
