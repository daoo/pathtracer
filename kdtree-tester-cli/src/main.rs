use clap::Parser;
use geometry::{
    intersect::{intersect_triangle_ray, TriangleRayIntersection},
    ray::Ray,
};
use nalgebra::Vector2;
use pathtracer::{camera::Pinhole, sampling::uniform_sample_unit_square, scene::Scene};
use rand::{rngs::SmallRng, SeedableRng};
use rayon::prelude::*;
use std::{fs, str};
use wavefront::{mtl, obj};

struct RayBouncer<'a> {
    bounces: u32,
    scene: &'a Scene,
    width: u32,
    height: u32,
    camera: &'a Pinhole,
}

#[derive(Debug, Clone, Copy)]
struct CheckedIntersection {
    pub ray: Ray,
    pub reference: Option<(usize, TriangleRayIntersection)>,
    pub kdtree: Option<(usize, TriangleRayIntersection)>,
}

impl CheckedIntersection {
    pub fn is_valid(&self) -> bool {
        const T_TOLERANCE: f32 = 0.000001;
        const UV_TOLERANCE: f32 = 0.00001;
        match (self.reference, self.kdtree) {
            (None, None) => true,
            (Some((t1, i1)), Some((t2, i2))) => {
                t1 == t2
                    && (i1.t - i2.t).abs() < T_TOLERANCE
                    && (i1.u - i2.u).abs() < UV_TOLERANCE
                    && (i1.v - i2.v).abs() < UV_TOLERANCE
            }
            _ => false,
        }
    }
}

impl RayBouncer<'_> {
    fn reference_ray_intersect(
        &self,
        ray: &Ray,
        tmin: f32,
        tmax: f32,
    ) -> Option<(usize, TriangleRayIntersection)> {
        let t_range = tmin..=tmax;
        self.scene
            .kdtree
            .triangles
            .iter()
            .enumerate()
            .filter_map(|(index, triangle)| {
                intersect_triangle_ray(triangle, ray).and_then(|intersection| {
                    t_range
                        .contains(&intersection.t)
                        .then_some((index, intersection))
                })
            })
            .min_by(|a, b| f32::total_cmp(&a.1.t, &b.1.t))
    }

    fn checked_ray_intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> CheckedIntersection {
        let kdtree = self.scene.intersect(ray, tmin, tmax);
        let reference = self.reference_ray_intersect(ray, tmin, tmax);
        CheckedIntersection {
            ray: *ray,
            reference,
            kdtree,
        }
    }

    fn bounce(
        &self,
        rng: &mut SmallRng,
        ray: &Ray,
        accumulated_bounces: u32,
    ) -> Option<CheckedIntersection> {
        if accumulated_bounces >= self.bounces {
            return None;
        }

        let intersection = self.checked_ray_intersect(ray, 0.0, f32::MAX);
        if !intersection.is_valid() {
            return Some(intersection);
        };
        let (triangle_index, intersection) = intersection.reference?;

        let wi = -ray.direction;
        let n = self.scene.triangle_normals[triangle_index].lerp(intersection.u, intersection.v);
        let material = &self.scene.triangle_materials[triangle_index];

        // TODO: How to chose offset?
        let offset = 0.00001 * n.into_inner();
        let point = ray.param(intersection.t);
        let point_above = point + offset;
        let point_below = point - offset;

        let incoming_fails = self
            .scene
            .lights
            .iter()
            .filter_map(|light| {
                let shadow_ray = Ray::between(&point_above, &light.sample(rng));
                let shadow = self.checked_ray_intersect(&shadow_ray, 0.0, 1.0);
                (!shadow.is_valid()).then_some(shadow)
            })
            .collect::<Vec<_>>();
        if let Some(checked) = incoming_fails.first() {
            return Some(*checked);
        }

        let sample = material.sample(&wi, &n, rng);
        let next_ray = Ray {
            origin: if sample.wo.dot(&n) >= 0.0 {
                point_above
            } else {
                point_below
            },
            direction: *sample.wo,
        };

        self.bounce(rng, &next_ray, accumulated_bounces + 1)
    }

    fn bounce_pixel(&self, pixel: (u32, u32)) -> Option<CheckedIntersection> {
        let (x, y) = pixel;
        let mut rng = SmallRng::seed_from_u64((y * self.height + x) as u64);
        let pixel_center = Vector2::new(x as f32, y as f32) + uniform_sample_unit_square(&mut rng);
        let scene_direction =
            pixel_center.component_div(&Vector2::new(self.width as f32, self.height as f32));
        let ray = self.camera.ray(scene_direction.x, scene_direction.y);
        self.bounce(&mut rng, &ray, 0)
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    #[arg(short, long, default_value_t = 512)]
    width: u32,
    #[arg(short, long, default_value_t = 512)]
    height: u32,
    #[arg(short, long, default_value_t = 10)]
    bounces: u32,
    #[arg(short = 'n', long, default_value_t = 4)]
    iterations: u32,

    #[arg(long, default_value_t = 20)]
    max_depth: u32,
    #[arg(long, default_value_t = 2.0)]
    traverse_cost: f32,
    #[arg(long, default_value_t = 1.0)]
    intersect_cost: f32,
    #[arg(long, default_value_t = 0.8)]
    empty_factor: f32,
}

fn main() {
    let args = Args::parse();

    println!("Loading {}...", args.input.display());
    let obj = obj::obj(str::from_utf8(&fs::read(&args.input).unwrap()).unwrap());
    let mtl_path = args.input.parent().unwrap().join(&obj.mtl_lib);
    println!("Loading {}...", mtl_path.display());
    let mtl = mtl::mtl(str::from_utf8(&fs::read(mtl_path).unwrap()).unwrap());
    println!("Building scene...");
    let scene = Scene::from_wavefront(
        &obj,
        &mtl,
        args.max_depth,
        args.traverse_cost,
        args.intersect_cost,
        args.empty_factor,
    );
    println!("Triangles: {}", scene.triangle_normals.len());

    let pinhole = Pinhole::new(&scene.cameras[0], args.width as f32 / args.height as f32);

    let bouncer = RayBouncer {
        bounces: args.bounces,
        scene: &scene,
        width: args.width,
        height: args.height,
        camera: &pinhole,
    };

    println!(
        "Testing up to {} rays...",
        args.width * args.height * args.bounces
    );

    let xs = 0..args.width;
    let ys = 0..args.height;
    let pixels = ys
        .flat_map(|y| xs.clone().map(move |x| (x, y)))
        .collect::<Vec<_>>();
    let fails = pixels
        .into_par_iter()
        .flat_map(|pixel| bouncer.bounce_pixel(pixel))
        .collect::<Vec<_>>();

    println!("{} fails", fails.len());
    for fail in fails {
        eprintln!("{:?}", fail.ray);
        eprintln!("  reference: {:?}", fail.reference);
        eprintln!("     kdtree: {:?}", fail.kdtree);
    }
}
