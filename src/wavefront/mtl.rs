use nalgebra::Vector3;
use nom::IResult;
use nom::bytes::complete::{tag, is_not};
use nom::character::complete::multispace0;
use nom::number::complete::float;
use nom::sequence::Tuple;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct Material {
  name: String,
  diffuse_map: String,
  diffuse_reflection: Vector3<f32>,
  specular_reflection: Vector3<f32>,
  emittance: Vector3<f32>,
  transparency: f32,
  reflection_0_degrees: f32,
  reflection_90_degrees: f32,
  index_of_refraction: f32,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct Light {
  position: Vector3<f32>,
  color: Vector3<f32>,
  radius: f32,
  intensity: f32,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct Camera {
  position: Vector3<f32>,
  target: Vector3<f32>,
  up: Vector3<f32>,
  fov: f32,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Mtl {
  materials: Vec<Material>,
  lights: Vec<Light>,
  cameras: Vec<Camera>,
}

fn tagged<'a, O>(name: &str, data: impl Fn(&'a str) -> IResult<&'a str, O>, input: &'a str) -> IResult<&'a str, O> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(name)(input)?;
    let (input, _) = multispace0(input)?;
    data(input)
}

fn vec3(input: &str) -> IResult<&str, Vector3<f32>> {
    let (input, (x, _, y, _, z)) = (float, multispace0, float, multispace0, float).parse(input)?;
    Ok((input, Vector3::new(x, y, z)))
}

fn newlight(input: &str) -> IResult<&str, &str> { tagged("newlight", is_not("\r\n"), input) }
fn lightposition(input: &str) -> IResult<&str, Vector3<f32>> { tagged("lightposition", vec3, input) }
fn lightcolor(input: &str) -> IResult<&str, Vector3<f32>> { tagged("lightcolor", vec3, input) }
fn lightradius(input: &str) -> IResult<&str, f32> { tagged("lightradius", float, input) }
fn lightintensity(input: &str) -> IResult<&str, f32> { tagged("lightintensity", float, input) }

fn newcamera(input: &str) -> IResult<&str, &str> { tagged("newcamera", is_not("\r\n"), input) }
fn cameraposition(input: &str) -> IResult<&str, Vector3<f32>> { tagged("cameraposition", vec3, input) }
fn cameratarget(input: &str) -> IResult<&str, Vector3<f32>> { tagged("cameratarget", vec3, input) }
fn cameraup(input: &str) -> IResult<&str, Vector3<f32>> { tagged("cameraup", vec3, input) }
fn camerafov(input: &str) -> IResult<&str, f32> { tagged("camerafov", float, input) }

fn newmtl(input: &str) -> IResult<&str, &str> { tagged("newmtl", is_not("\r\n"), input) }
fn kd(input: &str) -> IResult<&str, Vector3<f32>> { tagged("kd", vec3, input) }
fn ks(input: &str) -> IResult<&str, Vector3<f32>> { tagged("ks", vec3, input) }
fn reflat0deg(input: &str) -> IResult<&str, f32> { tagged("reflat0deg", float, input) }
fn reflat90deg(input: &str) -> IResult<&str, f32> { tagged("reflat90deg", float, input) }
fn indexofrefraction(input: &str) -> IResult<&str, f32> { tagged("indexofrefraction", float, input) }
fn transparency(input: &str) -> IResult<&str, f32> { tagged("transparency", float, input) }

pub fn mtl(input: &str) -> Mtl {
  let mut materials: Vec<Material> = Vec::new();
  let mut lights: Vec<Light> = Vec::new();
  let mut cameras: Vec<Camera> = Vec::new();

  for line in input.lines() {
      match newlight(line) {
          Ok((_, _)) => lights.push(Default::default()), _ => ()
      }
      match lightposition(line) {
          Ok((_, x)) => lights.last_mut().unwrap().position = x, _ => ()
      }
      match lightcolor(line) {
          Ok((_, x)) => lights.last_mut().unwrap().color = x, _ => ()
      }
      match lightradius(line) {
          Ok((_, x)) => lights.last_mut().unwrap().radius = x, _ => ()
      }
      match lightintensity(line) {
          Ok((_, x)) => lights.last_mut().unwrap().intensity = x, _ => ()
      }

      match newcamera(line) {
          Ok((_, _)) => cameras.push(Default::default()), _ => ()
      }
      match cameraposition(line) {
          Ok((_, x)) => cameras.last_mut().unwrap().position = x, _ => ()
      }
      match cameratarget(line) {
          Ok((_, x)) => cameras.last_mut().unwrap().target = x, _ => ()
      }
      match cameraup(line) {
          Ok((_, x)) => cameras.last_mut().unwrap().up = x, _ => ()
      }
      match camerafov(line) {
          Ok((_, x)) => cameras.last_mut().unwrap().fov = x, _ => ()
      }

      match newmtl(line) {
          Ok((_, name)) => {
              materials.push(Default::default());
              materials.last_mut().unwrap().name = name.to_string()
          },
          _ => ()
      }
      match kd(line) {
          Ok((_, x)) => materials.last_mut().unwrap().diffuse_reflection = x, _ => ()
      }
      match ks(line) {
          Ok((_, x)) => materials.last_mut().unwrap().specular_reflection = x, _ => ()
      }
      match reflat0deg(line) {
          Ok((_, x)) => materials.last_mut().unwrap().reflection_0_degrees = x, _ => ()
      }
      match reflat90deg(line) {
          Ok((_, x)) => materials.last_mut().unwrap().reflection_90_degrees = x, _ => ()
      }
      match indexofrefraction(line) {
          Ok((_, x)) => materials.last_mut().unwrap().index_of_refraction = x, _ => ()
      }
      match transparency(line) {
          Ok((_, x)) => materials.last_mut().unwrap().transparency = x, _ => ()
      }
  }

  Mtl{materials, lights, cameras}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3() {
        assert_eq!(vec3("0 0 0"), Ok(("", Vector3::new(0.0, 0.0, 0.0))));
        assert_eq!(vec3("1 2 3"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
        assert_eq!(vec3("1.0 2.0 3.0"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
        assert_eq!(vec3("-1.0 -2.0 -3.0"), Ok(("", Vector3::new(-1.0, -2.0, -3.0))));
    }

    #[test]
    fn test_light() {
        assert_eq!(newlight("newlight light01"), Ok(("", "light01")));
        assert_eq!(lightposition("lightposition 1.0 2.0 3.0"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
        assert_eq!(lightcolor("lightcolor 1.0 2.0 3.0"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
        assert_eq!(lightradius("lightradius 1.0"), Ok(("", 1.0)));
        assert_eq!(lightintensity("lightintensity 1.0"), Ok(("", 1.0)));
    }

    #[test]
    fn test_camera() {
        assert_eq!(newcamera("newcamera camera01"), Ok(("", "camera01")));
        assert_eq!(cameraposition("cameraposition 1.0 2.0 3.0"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
        assert_eq!(cameratarget("cameratarget 1.0 2.0 3.0"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
        assert_eq!(cameraup("cameraup 1.0 2.0 3.0"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
        assert_eq!(camerafov("camerafov 1.0"), Ok(("", 1.0)));
    }

    #[test]
    fn test_mtl() {
        assert_eq!(newmtl("newmtl white"), Ok(("", "white")));
        assert_eq!(kd("kd 1.0 2.0 3.0"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
        assert_eq!(ks("ks 1.0 2.0 3.0"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
        assert_eq!(reflat0deg("reflat0deg 1.0"), Ok(("", 1.0)));
        assert_eq!(reflat90deg("reflat90deg 1.0"), Ok(("", 1.0)));
        assert_eq!(indexofrefraction("indexofrefraction 1.0"), Ok(("", 1.0)));
        assert_eq!(transparency("transparency 1.0"), Ok(("", 1.0)));
    }
}
