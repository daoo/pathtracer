use nalgebra::Vector2;
use nalgebra::Vector3;
use nom::IResult;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{char, i32, multispace0};
use nom::combinator::{opt, rest};
use nom::number::complete::float;
use nom::sequence::Tuple;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Point {
    pub v: i32,
    pub t: i32,
    pub n: i32,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Face {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Chunk {
    pub faces: Vec<Face>,
    pub material: String,
}

impl Chunk {
    pub fn new(material: String) -> Chunk {
        Chunk { faces: Vec::new(), material }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Obj {
  pub mtl_lib: PathBuf,

  pub vertices: Vec<Vector3<f32>>,
  pub normals: Vec<Vector3<f32>>,
  pub texcoords: Vec<Vector2<f32>>,
  pub chunks: Vec<Chunk>,
}

impl Obj {
    pub fn index_vertex(&self, point: &Point) -> Vector3<f32> {
        index_wavefront_vec(&self.vertices, point.v)
    }

    pub fn index_texcoord(&self, point: &Point) -> Vector2<f32> {
        index_wavefront_vec(&self.texcoords, point.t)
    }

    pub fn index_normal(&self, point: &Point) -> Vector3<f32> {
        index_wavefront_vec(&self.normals, point.n)
    }
}

fn index_wavefront_vec<T: Default + Clone>(v: &[T], i: i32) -> T {
    if i == 0 {
        Default::default()
    } else if i < 0 {
        v[((v.len() as i32) + i) as usize].clone()
    } else {
        v[(i - 1) as usize].clone()
    }
}

fn tagged<'a, O>(name: &str, data: impl Fn(&'a str) -> IResult<&'a str, O>, input: &'a str) -> IResult<&'a str, O> {
    let (input, _) = tag_no_case(name)(input)?;
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

fn i32_or_zero(input: &str) -> IResult<&str, i32> {
    opt(i32)(input).map(|(input, n)| (input, n.unwrap_or(0)))
}

fn point(input: &str) -> IResult<&str, Point> {
    (i32, char('/'), i32_or_zero, char('/'), i32)
        .parse(input)
        .map(|(input, (v, _, t, _, n))| (input, Point { v, t, n }))
}

fn triangle(input: &str) -> IResult<&str, Face> {
    (point, multispace0, point, multispace0, point)
        .parse(input)
        .map(|(input, (p0, _, p1, _, p2))| (input, Face{ p0, p1, p2 }))
}

pub fn obj(input: &str) -> Obj {
    let mut mtl_lib = Path::new("");
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut vertices: Vec<Vector3<f32>> = Vec::new();
    let mut normals: Vec<Vector3<f32>> = Vec::new();
    let mut texcoords: Vec<Vector2<f32>> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("#") {
            continue
        }

        if let Ok((_, x)) = tagged("mtllib", rest, line) {
            mtl_lib = Path::new(x)
        } else if let Ok((_, x)) = tagged("usemtl", rest, line) {
            chunks.push(Chunk::new(x.to_string()))
        } else if let Ok((_, x)) = tagged("v", vec3, line) {
            vertices.push(x)
        } else if let Ok((_, x)) = tagged("vn", vec3, line) {
            normals.push(x)
        } else if let Ok((_, x)) = tagged("vt", vec2, line) {
            texcoords.push(x)
        } else if let Ok((_, x)) = tagged("f", triangle, line) {
            chunks.last_mut().unwrap().faces.push(x)
        } else if let Ok((_, _)) = tagged("o", rest, line) {
            // TODO: not supported
        } else if let Ok((_, _)) = tagged("s", rest, line) {
            // TODO: not supported
        } else {
            panic!("Unexpected line: \"{}\"", line)
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
    fn test_i32_or_zero() {
        assert_eq!(i32_or_zero("1"), Ok(("", 1)));
        assert_eq!(i32_or_zero("-1"), Ok(("", -1)));
        assert_eq!(i32_or_zero(""), Ok(("", 0)));
    }

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
    fn test_point() {
        assert_eq!(point("1/2/3"), Ok(("", Point{v:1, t:2, n:3})));
        assert_eq!(point("1//3"), Ok(("", Point{v:1, t:0, n:3})));
    }

    #[test]
    fn test_data() {
        assert_eq!(obj("v 1 2 3").vertices, [Vector3::new(1.0, 2.0, 3.0)]);
        assert_eq!(obj("vt 1 2").texcoords, [Vector2::new(1.0, 2.0)]);
        assert_eq!(obj("vn 1 2 3").normals, [Vector3::new(1.0, 2.0, 3.0)]);
    }

    #[test]
    fn test_faces() {
        assert_eq!(obj("usemtl m1\nf 1/2/3 4/5/6 7/8/9").chunks, [Chunk{
            faces: vec![Face{
                p0: Point{v:1, t:2, n:3},
                p1: Point{v:4, t:5, n:6},
                p2: Point{v:7, t:8, n:9}
            }],
            material: "m1".to_string()
        }]);
    }

    #[test]
    fn test_usemtl() {
        assert_eq!(obj("usemtl m1").chunks, [Chunk{
            faces: vec![],
            material: "m1".to_string(),
        }]);
    }

    #[test]
    fn test_mtllib() {
        assert_eq!(obj("mtllib test.mtl").mtl_lib, Path::new("test.mtl"));
    }

    #[test]
    fn test_unsupported() {
        assert_eq!(obj("o todo").chunks.len(), 0);
        assert_eq!(obj("s todo").chunks.len(), 0);
    }
}
