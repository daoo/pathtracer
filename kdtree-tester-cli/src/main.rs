use clap::Parser;
use geometry::{intersection::RayIntersection, ray::Ray, Geometry};
use kdtree::{
    build::build_kdtree,
    build_sah::{self, SahKdTreeBuilder},
    KdTree,
};
use nalgebra::Vector2;
use pathtracer::{
    camera::Pinhole, material::Material, sampling::uniform_sample_unit_square, scene::Scene,
};
use rand::{rngs::SmallRng, SeedableRng};
use rayon::prelude::*;
use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter, Write},
    ops::RangeInclusive,
    str::{self, FromStr},
};
use wavefront::{mtl, obj};

struct RayBouncer {
    scene: Scene,
    kdtree: KdTree,
    camera: Pinhole,
    bounces: u32,
    size: Size,
}

#[derive(Debug, Clone, Copy)]
struct CheckedIntersection {
    pub ray: Ray,
    pub reference: Option<(u32, RayIntersection)>,
    pub kdtree: Option<(u32, RayIntersection)>,
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

    fn as_bytes(&self, iteration: u16) -> [u8; 50] {
        let mut bytes = [0u8; 50];
        let ray = if let Some(kdtree) = self.kdtree {
            self.ray.extended(kdtree.1.t)
        } else if let Some(reference) = self.reference {
            self.ray.extended(reference.1.t)
        } else {
            self.ray
        };
        let correct_point = self.ray.param(self.reference.unwrap().1.t);
        let actual_point = if let Some(kdtree) = self.kdtree {
            self.ray.param(kdtree.1.t)
        } else {
            [0.0, 0.0, 0.0].into()
        };
        bytes[0..2].copy_from_slice(&iteration.to_le_bytes());
        bytes[2..6].copy_from_slice(&ray.origin.x.to_le_bytes());
        bytes[6..10].copy_from_slice(&ray.origin.y.to_le_bytes());
        bytes[10..14].copy_from_slice(&ray.origin.z.to_le_bytes());
        bytes[14..18].copy_from_slice(&ray.direction.x.to_le_bytes());
        bytes[18..22].copy_from_slice(&ray.direction.y.to_le_bytes());
        bytes[22..26].copy_from_slice(&ray.direction.z.to_le_bytes());
        bytes[26..30].copy_from_slice(&correct_point.x.to_le_bytes());
        bytes[30..34].copy_from_slice(&correct_point.y.to_le_bytes());
        bytes[34..38].copy_from_slice(&correct_point.z.to_le_bytes());
        bytes[38..42].copy_from_slice(&actual_point.x.to_le_bytes());
        bytes[42..46].copy_from_slice(&actual_point.y.to_le_bytes());
        bytes[46..50].copy_from_slice(&actual_point.z.to_le_bytes());
        bytes
    }
}

impl RayBouncer {
    fn reference_ray_intersect(
        &self,
        ray: &Ray,
        t_range: RangeInclusive<f32>,
    ) -> Option<(u32, RayIntersection)> {
        self.kdtree
            .geometries
            .iter()
            .enumerate()
            .filter_map(|(index, geometry)| {
                geometry.intersect_ray(ray).and_then(|intersection| {
                    t_range
                        .contains(&intersection.t)
                        .then_some((index as u32, intersection))
                })
            })
            .min_by(|a, b| f32::total_cmp(&a.1.t, &b.1.t))
    }

    fn checked_ray_intersect(
        &self,
        ray: &Ray,
        t_range: RangeInclusive<f32>,
    ) -> CheckedIntersection {
        let kdtree = self.kdtree.intersect(ray, t_range.clone());
        let reference = self.reference_ray_intersect(ray, t_range);
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

        let intersection = self.checked_ray_intersect(ray, 0.0..=f32::MAX);
        if !intersection.is_valid() {
            return Some(intersection);
        };
        let (triangle_index, intersection) = intersection.reference?;
        let triangle = &self.scene.triangle_data[triangle_index as usize];

        let wi = -ray.direction;
        let n = triangle.normals.lerp(intersection.u, intersection.v);
        let material = triangle.material.as_ref();

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
                let shadow = self.checked_ray_intersect(&shadow_ray, 0.0..=1.0);
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
        let mut rng = SmallRng::seed_from_u64((y * self.size.height + x) as u64);
        let pixel_center = Vector2::new(x as f32, y as f32) + uniform_sample_unit_square(&mut rng);
        let scene_direction = pixel_center.component_div(&self.size.as_vec2());
        let ray = self.camera.ray(scene_direction.x, scene_direction.y);
        self.bounce(&mut rng, &ray, 0)
    }
}

#[derive(Clone, Copy, Debug)]
struct Size {
    width: u32,
    height: u32,
}

impl Size {
    fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }

    fn aspect_ratio(self) -> f32 {
        self.width as f32 / self.height as f32
    }

    fn as_vec2(self) -> Vector2<f32> {
        Vector2::new(self.width as f32, self.height as f32)
    }
}

impl FromStr for Size {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s.find('x').expect("Could not parse");
        Ok(Size {
            width: s[0..pos].parse()?,
            height: s[pos + 1..].parse()?,
        })
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Wavefront OBJ input path
    #[arg(short = 'i', long, required = true)]
    input: std::path::PathBuf,

    /// Output ray fail binary data path
    #[arg(short = 'o', long)]
    output: Option<std::path::PathBuf>,
    /// Image size in pixels
    #[arg(short, long, default_value_t = Size::new(512, 512))]
    size: Size,
    /// Max number of bounces to test
    #[arg(short, long, default_value_t = 10)]
    bounces: u32,

    /// Maximum kd-tree depth
    #[arg(long, default_value_t = build_sah::MAX_DEPTH)]
    max_depth: u32,
    /// SAH kd-tree traverse cost
    #[arg(long, default_value_t = build_sah::TRAVERSE_COST)]
    traverse_cost: f32,
    /// SAH kd-tree intersect cost
    #[arg(long, default_value_t = build_sah::INTERSECT_COST)]
    intersect_cost: f32,
    /// SAH kd-tree empty factor
    #[arg(long, default_value_t = build_sah::EMPTY_FACTOR)]
    empty_factor: f32,
}

fn main() {
    let args = Args::parse();

    println!("Loading {}...", args.input.display());
    let obj = obj::obj(&mut BufReader::new(File::open(&args.input).unwrap()));
    println!("  Chunks: {}", obj.chunks.len());
    println!("  Vertices: {}", obj.vertices.len());
    println!("  Normals: {}", obj.normals.len());
    println!("  Texcoords: {}", obj.texcoords.len());

    let mtl_path = args.input.parent().unwrap().join(&obj.mtl_lib);
    println!("Loading {}...", mtl_path.display());
    let mtl = mtl::mtl(&mut BufReader::new(File::open(mtl_path).unwrap()));
    println!("  Materials: {}", mtl.materials.len());
    println!("  Lights: {}", mtl.lights.len());
    println!("  Cameras: {}", mtl.cameras.len());

    println!("Collecting scene...");
    let scene = Scene::from_wavefront(&obj, &mtl);
    println!("  Triangles: {}", scene.triangle_data.len());

    println!("Building kdtree...");
    let kdtree = build_kdtree(
        SahKdTreeBuilder {
            traverse_cost: args.traverse_cost,
            intersect_cost: args.intersect_cost,
            empty_factor: args.empty_factor,
            geometries: scene
                .triangle_data
                .iter()
                .map(|t| t.triangle.into())
                .collect(),
        },
        args.max_depth,
    );

    println!(
        "Testing up to {} rays...",
        args.size.width * args.size.height * args.bounces
    );
    let camera = Pinhole::new(&scene.cameras[0], args.size.aspect_ratio());
    let bouncer = RayBouncer {
        scene,
        kdtree,
        camera,
        size: args.size,
        bounces: args.bounces,
    };

    let xs = 0..args.size.width;
    let ys = 0..args.size.height;
    let pixels = ys
        .flat_map(|y| xs.clone().map(move |x| (x, y)))
        .collect::<Vec<_>>();
    let fails = pixels
        .into_par_iter()
        .filter_map(|pixel| bouncer.bounce_pixel(pixel))
        .map(|fail| {
            eprintln!("{:?}", fail.ray);
            eprintln!("  reference: {:?}", fail.reference);
            eprintln!("     kdtree: {:?}", fail.kdtree);
            fail
        })
        .collect::<Vec<_>>();
    println!("Found {} fails", fails.len());

    if let Some(path) = args.output {
        println!("Writing failed rays to {:?}...", path);
        let mut logger = BufWriter::new(File::create(path).unwrap());
        fails.iter().enumerate().for_each(|(i, fail)| {
            logger.write_all(&fail.as_bytes(i as u16)).unwrap();
        });
    }
}
