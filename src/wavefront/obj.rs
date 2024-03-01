use nalgebra::Vector2;
use nalgebra::Vector3;
use nom::IResult;
use nom::bytes::complete::{tag, is_not};
use nom::character::complete::{char, i32, multispace0};
use nom::combinator::opt;
use nom::number::complete::float;
use nom::sequence::Tuple;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Point {
    v: i32,
    t: i32,
    n: i32,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Face {
    p1: Point,
    p2: Point,
    p3: Point,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Chunk {
    polygons: Vec<Face>,
    material: String,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Obj {
  mtl_lib: PathBuf,

  vertices: Vec<Vector3<f32>>,
  normals: Vec<Vector3<f32>>,
  texcoords: Vec<Vector2<f32>>,
  chunks: Vec<Chunk>,
}

impl Obj {
    fn index_vertex(&self, i: i32) -> Vector3<f32> {
        index_wavefront_vec(&self.vertices, i)
    }

    fn index_normal(&self, i: i32) -> Vector3<f32> {
        index_wavefront_vec(&self.normals, i)
    }

    fn index_texcoord(&self, i: i32) -> Vector2<f32> {
        index_wavefront_vec(&self.texcoords, i)
    }
}

fn index_wavefront_vec<T: Default + Clone>(v: &[T], i: i32) -> T {
    if i == 0 {
        Default::default()
    } else if i < 0 {
        v[((v.len() as i32) + i) as usize].clone()
    } else {
        v[i as usize].clone()
    }
}

fn tagged<'a, O>(name: &str, data: impl Fn(&'a str) -> IResult<&'a str, O>, input: &'a str) -> IResult<&'a str, O> {
    let (input, _) = tag(name)(input)?;
    let (input, _) = multispace0(input)?;
    data(input)
}

fn vec2(input: &str) -> IResult<&str, Vector2<f32>> {
    let (input, (x, _, y)) = (float, multispace0, float).parse(input)?;
    Ok((input, Vector2::new(x, y)))
}

fn vec3(input: &str) -> IResult<&str, Vector3<f32>> {
    let (input, (x, _, y, _, z)) = (float, multispace0, float, multispace0, float).parse(input)?;
    Ok((input, Vector3::new(x, y, z)))
}

fn vn(input: &str) -> IResult<&str, Vector3<f32>> { tagged("vn", vec3, input) }
fn vt(input: &str) -> IResult<&str, Vector2<f32>> { tagged("vt", vec2, input) }
fn v(input: &str) -> IResult<&str, Vector3<f32>> { tagged("v", vec3, input) }

fn i32_or_zero(input: &str) -> IResult<&str, i32> {
    let (input, n) = opt(i32)(input)?;
    Ok((input, n.unwrap_or(0)))
}

fn point(input: &str) -> IResult<&str, Point> {
    let (input, (v, _, t, _, n)) = (i32, char('/'), i32_or_zero, char('/'), i32).parse(input)?;
    Ok((input, Point { v, t, n }))
}

fn f(input: &str) -> IResult<&str, Face> {
    let (input, _) = tag("f")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, (p1, _, p2, _, p3)) = (point, multispace0, point, multispace0, point).parse(input)?;
    Ok((input, Face { p1, p2, p3 }))
}

fn usemtl(input: &str) -> IResult<&str, &str> {
    tagged("usemtl", is_not("\r\n"), input)
}

fn mtllib(input: &str) -> IResult<&str, &Path> {
    let (input, s) = tagged("mtllib", is_not("\r\n"), input)?;
    Ok((input, Path::new(s)))
}

pub fn obj(input: &str) -> Obj {
    let mut mtl_lib = Path::new("");
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut vertices: Vec<Vector3<f32>> = Vec::new();
    let mut normals: Vec<Vector3<f32>> = Vec::new();
    let mut texcoords: Vec<Vector2<f32>> = Vec::new();

    for line in input.lines() {
        match mtllib(line) {
            Ok((_, x)) => mtl_lib = x, _ => ()
        }
        match usemtl(line) {
            Ok((_, x)) => chunks.push(Chunk{material: x.to_string(), polygons: Vec::new()}), _ => ()
        }
        match v(line) {
            Ok((_, x)) => vertices.push(x), _ => ()
        }
        match vn(line) {
            Ok((_, x)) => normals.push(x), _ => ()
        }
        match vt(line) {
            Ok((_, x)) => texcoords.push(x), _ => ()
        }
        match f(line) {
            Ok((_, x)) => chunks.last_mut().unwrap().polygons.push(x), _ => ()
        }
    }

    Obj {
        mtl_lib: mtl_lib.to_path_buf(),
        vertices,
        normals,
        texcoords,
        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2() {
        assert_eq!(vec2("0 0"), Ok(("", Vector2::new(0.0, 0.0))));
        assert_eq!(vec2("1 2"), Ok(("", Vector2::new(1.0, 2.0))));
        assert_eq!(vec2("1.0 2.0"), Ok(("", Vector2::new(1.0, 2.0))));
        assert_eq!(vec2("-1.0 -2.0"), Ok(("", Vector2::new(-1.0, -2.0))));
    }

    #[test]
    fn test_vec3() {
        assert_eq!(vec3("0 0 0"), Ok(("", Vector3::new(0.0, 0.0, 0.0))));
        assert_eq!(vec3("1 2 3"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
        assert_eq!(vec3("1.0 2.0 3.0"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
        assert_eq!(vec3("-1.0 -2.0 -3.0"), Ok(("", Vector3::new(-1.0, -2.0, -3.0))));
    }

    #[test]
    fn test_v() {
        assert_eq!(v("v 1 2 3"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
    }

    #[test]
    fn test_vt() {
        assert_eq!(vt("vt 1 2"), Ok(("", Vector2::new(1.0, 2.0))));
    }

    #[test]
    fn test_vn() {
        assert_eq!(vn("vn 1 2 3"), Ok(("", Vector3::new(1.0, 2.0, 3.0))));
    }

    #[test]
    fn test_i32_or_zero() {
        assert_eq!(i32_or_zero("1"), Ok(("", 1)));
        assert_eq!(i32_or_zero("-1"), Ok(("", -1)));
        assert_eq!(i32_or_zero(""), Ok(("", 0)));
    }

    #[test]
    fn test_point() {
        assert_eq!(point("1/2/3"), Ok(("", Point{v:1, t:2, n:3})));
        assert_eq!(point("1//3"), Ok(("", Point{v:1, t:0, n:3})));
    }

    #[test]
    fn test_f() {
        assert_eq!(f("f 1/2/3 4/5/6 7/8/9"), Ok(("", Face{
            p1: Point{v:1, t:2, n:3},
            p2: Point{v:4, t:5, n:6},
            p3: Point{v:7, t:8, n:9}
        })));
    }

    #[test]
    fn test_usemtl() {
        assert_eq!(usemtl("usemtl test"), Ok(("", "test")));
    }

    #[test]
    fn test_mtllib() {
        assert_eq!(mtllib("mtllib test.mtl"), Ok(("", Path::new("test.mtl"))));
    }
}
