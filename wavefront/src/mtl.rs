use nom::{
    bytes::complete::tag, character::complete::multispace0, combinator::rest,
    number::complete::float, sequence::Tuple, IResult,
};
use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub struct Material {
    pub name: String,
    pub diffuse_map: String,
    pub diffuse_reflection: [f32; 3],
    pub specular_reflection: [f32; 3],
    pub emittance: [f32; 3],
    pub transparency: f32,
    pub reflection_0_degrees: f32,
    pub reflection_90_degrees: f32,
    pub index_of_refraction: f32,
}

impl Material {
    pub fn new(name: String) -> Material {
        Material {
            name,
            diffuse_map: String::new(),
            diffuse_reflection: [0.7, 0.7, 0.7],
            specular_reflection: [1.0, 1.0, 1.0],
            emittance: [0.0, 0.0, 0.0],
            transparency: 0.0,
            reflection_0_degrees: 0.0,
            reflection_90_degrees: 0.0,
            index_of_refraction: 1.0,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Light {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub radius: f32,
    pub intensity: f32,
}

#[derive(Debug, Default, PartialEq)]
pub struct Camera {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub up: [f32; 3],
    pub fov: f32,
}

#[derive(Debug, PartialEq)]
pub struct Mtl {
    pub materials: Vec<Material>,
    pub lights: Vec<Light>,
    pub cameras: Vec<Camera>,
}

fn tagged<'a, O>(
    name: &str,
    data: impl Fn(&'a str) -> IResult<&'a str, O>,
    input: &'a str,
) -> IResult<&'a str, O> {
    let (input, _) = tag(name)(input)?;
    let (input, _) = multispace0(input)?;
    data(input)
}

fn vec3(input: &str) -> IResult<&str, [f32; 3]> {
    let (input, (x, _, y, _, z)) = (float, multispace0, float, multispace0, float).parse(input)?;
    Ok((input, [x, y, z]))
}

pub fn mtl<R>(input: &mut R) -> Mtl
where
    R: BufRead,
{
    let mut materials: Vec<Material> = Vec::new();
    let mut lights: Vec<Light> = Vec::new();
    let mut cameras: Vec<Camera> = Vec::new();

    let mut line = String::new();
    while input.read_line(&mut line).unwrap() > 0 {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            line.clear();
            continue;
        }

        if let Ok((_, _)) = tagged("newlight", rest, trimmed) {
            lights.push(Light::default());
        } else if let Ok((_, x)) = tagged("lightposition", vec3, trimmed) {
            lights.last_mut().unwrap().position = x;
        } else if let Ok((_, x)) = tagged("lightcolor", vec3, trimmed) {
            lights.last_mut().unwrap().color = x;
        } else if let Ok((_, x)) = tagged("lightradius", float, trimmed) {
            lights.last_mut().unwrap().radius = x;
        } else if let Ok((_, x)) = tagged("lightintensity", float, trimmed) {
            lights.last_mut().unwrap().intensity = x;
        } else if let Ok((_, _)) = tagged("newcamera", rest, trimmed) {
            cameras.push(Camera::default());
        } else if let Ok((_, x)) = tagged("cameraposition", vec3, trimmed) {
            cameras.last_mut().unwrap().position = x;
        } else if let Ok((_, x)) = tagged("cameratarget", vec3, trimmed) {
            cameras.last_mut().unwrap().target = x;
        } else if let Ok((_, x)) = tagged("cameraup", vec3, trimmed) {
            cameras.last_mut().unwrap().up = x;
        } else if let Ok((_, x)) = tagged("camerafov", float, trimmed) {
            cameras.last_mut().unwrap().fov = x;
        } else if let Ok((_, name)) = tagged("newmtl", rest, trimmed) {
            materials.push(Material::new(name.to_string()));
        } else if let Ok((_, _)) = tagged("illum", float, trimmed) {
            // TODO: not supported
        } else if let Ok((_, _)) = tagged("Ka", float, trimmed) {
            // TODO: not supported
        } else if let Ok((_, x)) = tagged("Kd", vec3, trimmed) {
            materials.last_mut().unwrap().diffuse_reflection = x;
        } else if let Ok((_, x)) = tagged("map_Kd", rest, trimmed) {
            materials.last_mut().unwrap().diffuse_map = x.to_string();
        } else if let Ok((_, x)) = tagged("Ks", vec3, trimmed) {
            materials.last_mut().unwrap().specular_reflection = x;
        } else if let Ok((_, _)) = tagged("Ns", float, trimmed) {
            // TODO: not supported
        } else if let Ok((_, _)) = tagged("Ke", float, trimmed) {
            // TODO: not supported
        } else if let Ok((_, x)) = tagged("reflat0deg", float, trimmed) {
            materials.last_mut().unwrap().reflection_0_degrees = x;
        } else if let Ok((_, x)) = tagged("reflat90deg", float, trimmed) {
            materials.last_mut().unwrap().reflection_90_degrees = x;
        } else if let Ok((_, x)) = tagged("Ni", float, trimmed) {
            materials.last_mut().unwrap().index_of_refraction = x;
        } else if let Ok((_, x)) = tagged("d", float, trimmed) {
            materials.last_mut().unwrap().transparency = 1.0 - x;
        } else if let Ok((_, x)) = tagged("Tr", float, trimmed) {
            materials.last_mut().unwrap().transparency = x;
        } else if let Ok((_, _)) = tagged("specularroughness", float, trimmed) {
            // TODO: not supported
        } else {
            panic!("Unexpected line: \"{line}\"");
        }
        line.clear();
    }

    Mtl {
        materials,
        lights,
        cameras,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3() {
        assert_eq!(vec3("0 0 0"), Ok(("", [0., 0., 0.])));
        assert_eq!(vec3("1 2 3"), Ok(("", [1., 2., 3.])));
        assert_eq!(vec3("1. 2. 3."), Ok(("", [1., 2., 3.])));
        assert_eq!(vec3("-1. -2. -3."), Ok(("", [-1., -2., -3.])));
    }

    fn mtl_test(str: &str) -> Mtl {
        mtl(&mut str.as_bytes())
    }

    #[test]
    fn test_light() {
        assert_eq!(mtl_test("newlight l1").lights.len(), 1);
        assert_eq!(
            mtl_test("newlight l1\nlightposition 1. 2. 3.").lights[0].position,
            [1., 2., 3.]
        );
        assert_eq!(
            mtl_test("newlight l1\nlightcolor 1. 2. 3.").lights[0].color,
            [1., 2., 3.]
        );
        assert_eq!(mtl_test("newlight l1\nlightradius 1.").lights[0].radius, 1.);
        assert_eq!(
            mtl_test("newlight l1\nlightintensity 1.").lights[0].intensity,
            1.
        );
    }

    #[test]
    fn test_camera() {
        assert_eq!(mtl_test("newcamera c1").cameras.len(), 1);
        assert_eq!(
            mtl_test("newcamera c1\ncameraposition 1. 2. 3.").cameras[0].position,
            [1., 2., 3.]
        );
        assert_eq!(
            mtl_test("newcamera c1\ncameratarget 1. 2. 3.").cameras[0].target,
            [1., 2., 3.]
        );
        assert_eq!(
            mtl_test("newcamera c1\ncameraup 1. 2. 3.").cameras[0].up,
            [1., 2., 3.]
        );
        assert_eq!(mtl_test("newcamera c1\ncamerafov 1.").cameras[0].fov, 1.);
    }

    #[test]
    fn test_mtl() {
        assert_eq!(mtl_test("newmtl m1").materials.len(), 1);
        assert_eq!(mtl_test("newmtl m1").materials[0].name, "m1");
        assert_eq!(
            mtl_test("newmtl m1\nKd 1. 2. 3.").materials[0].diffuse_reflection,
            [1., 2., 3.]
        );
        assert_eq!(
            mtl_test("newmtl m1\nmap_Kd file.png").materials[0].diffuse_map,
            "file.png"
        );
        assert_eq!(
            mtl_test("newmtl m1\nKs 1. 2. 3.").materials[0].specular_reflection,
            [1., 2., 3.]
        );
        assert_eq!(
            mtl_test("newmtl m1\nreflat0deg 0.5").materials[0].reflection_0_degrees,
            0.5
        );
        assert_eq!(
            mtl_test("newmtl m1\nreflat90deg 0.5").materials[0].reflection_90_degrees,
            0.5
        );
        assert_eq!(
            mtl_test("newmtl m1\nNi 0.5").materials[0].index_of_refraction,
            0.5
        );
        assert_eq!(mtl_test("newmtl m1\nd 1.0").materials[0].transparency, 0.0);
        assert_eq!(mtl_test("newmtl m1\nTr 0.5").materials[0].transparency, 0.5);
        assert_eq!(
            mtl_test("newmtl m1\nspecularroughness 1.").materials.len(),
            1
        );
    }

    #[test]
    fn test_comment() {
        assert_eq!(mtl_test("# comment\nnewmtl m1").materials.len(), 1);
    }
}
