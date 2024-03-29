use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::number::complete::float;
use nom::sequence::Tuple;
use nom::IResult;

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
    let (input, _) = tag_no_case(name)(input)?;
    let (input, _) = multispace0(input)?;
    data(input)
}

fn vec3(input: &str) -> IResult<&str, [f32; 3]> {
    let (input, (x, _, y, _, z)) = (float, multispace0, float, multispace0, float).parse(input)?;
    Ok((input, [x, y, z]))
}

pub fn mtl(input: &str) -> Mtl {
    let mut materials: Vec<Material> = Vec::new();
    let mut lights: Vec<Light> = Vec::new();
    let mut cameras: Vec<Camera> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Ok((_, _)) = tagged("newlight", rest, line) {
            lights.push(Light::default());
        } else if let Ok((_, x)) = tagged("lightposition", vec3, line) {
            lights.last_mut().unwrap().position = x;
        } else if let Ok((_, x)) = tagged("lightcolor", vec3, line) {
            lights.last_mut().unwrap().color = x;
        } else if let Ok((_, x)) = tagged("lightradius", float, line) {
            lights.last_mut().unwrap().radius = x;
        } else if let Ok((_, x)) = tagged("lightintensity", float, line) {
            lights.last_mut().unwrap().intensity = x;
        } else if let Ok((_, _)) = tagged("newcamera", rest, line) {
            cameras.push(Camera::default());
        } else if let Ok((_, x)) = tagged("cameraposition", vec3, line) {
            cameras.last_mut().unwrap().position = x;
        } else if let Ok((_, x)) = tagged("cameratarget", vec3, line) {
            cameras.last_mut().unwrap().target = x;
        } else if let Ok((_, x)) = tagged("cameraup", vec3, line) {
            cameras.last_mut().unwrap().up = x;
        } else if let Ok((_, x)) = tagged("camerafov", float, line) {
            cameras.last_mut().unwrap().fov = x;
        } else if let Ok((_, name)) = tagged("newmtl", rest, line) {
            materials.push(Material::new(name.to_string()));
        } else if let Ok((_, x)) = tagged("kd", vec3, line) {
            materials.last_mut().unwrap().diffuse_reflection = x;
        } else if let Ok((_, x)) = tagged("ks", vec3, line) {
            materials.last_mut().unwrap().specular_reflection = x;
        } else if let Ok((_, x)) = tagged("reflat0deg", float, line) {
            materials.last_mut().unwrap().reflection_0_degrees = x;
        } else if let Ok((_, x)) = tagged("reflat90deg", float, line) {
            materials.last_mut().unwrap().reflection_90_degrees = x;
        } else if let Ok((_, x)) = tagged("indexofrefraction", float, line) {
            materials.last_mut().unwrap().index_of_refraction = x;
        } else if let Ok((_, x)) = tagged("transparency", float, line) {
            materials.last_mut().unwrap().transparency = x;
        } else if let Ok((_, _)) = tagged("specularroughness", float, line) {
            // TODO: not supported
        } else if let Ok((_, _)) = tagged("map_kd", rest, line) {
            // TODO: not supported
        } else {
            panic!("Unexpected line: \"{line}\"");
        }
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

    #[test]
    fn test_light() {
        assert_eq!(mtl("newlight l1").lights.len(), 1);
        assert_eq!(
            mtl("newlight l1\nlightposition 1. 2. 3.").lights[0].position,
            [1., 2., 3.]
        );
        assert_eq!(
            mtl("newlight l1\nlightcolor 1. 2. 3.").lights[0].color,
            [1., 2., 3.]
        );
        assert_eq!(mtl("newlight l1\nlightradius 1.").lights[0].radius, 1.);
        assert_eq!(
            mtl("newlight l1\nlightintensity 1.").lights[0].intensity,
            1.
        );
    }

    #[test]
    fn test_camera() {
        assert_eq!(mtl("newcamera c1").cameras.len(), 1);
        assert_eq!(
            mtl("newcamera c1\ncameraposition 1. 2. 3.").cameras[0].position,
            [1., 2., 3.]
        );
        assert_eq!(
            mtl("newcamera c1\ncameratarget 1. 2. 3.").cameras[0].target,
            [1., 2., 3.]
        );
        assert_eq!(
            mtl("newcamera c1\ncameraup 1. 2. 3.").cameras[0].up,
            [1., 2., 3.]
        );
        assert_eq!(mtl("newcamera c1\ncamerafov 1.").cameras[0].fov, 1.);
    }

    #[test]
    fn test_mtl() {
        assert_eq!(mtl("newmtl m1").materials.len(), 1);
        assert_eq!(mtl("newmtl m1").materials[0].name, "m1");
        assert_eq!(
            mtl("newmtl m1\nkd 1. 2. 3.").materials[0].diffuse_reflection,
            [1., 2., 3.]
        );
        assert_eq!(
            mtl("newmtl m1\nks 1. 2. 3.").materials[0].specular_reflection,
            [1., 2., 3.]
        );
        assert_eq!(
            mtl("newmtl m1\nreflat0deg 1.").materials[0].reflection_0_degrees,
            1.
        );
        assert_eq!(
            mtl("newmtl m1\nreflat90deg 1.").materials[0].reflection_90_degrees,
            1.
        );
        assert_eq!(
            mtl("newmtl m1\nindexofrefraction 1.").materials[0].index_of_refraction,
            1.
        );
        assert_eq!(
            mtl("newmtl m1\ntransparency 1.").materials[0].transparency,
            1.
        );
        assert_eq!(mtl("newmtl m1\nspecularroughness 1.").materials.len(), 1);
        assert_eq!(mtl("newmtl m1\nmap_kd todo").materials.len(), 1);
    }
}
