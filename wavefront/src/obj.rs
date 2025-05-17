use nom::{
    IResult, Parser,
    bytes::complete::tag_no_case,
    character::complete::{char, i32, space0, space1},
    combinator::{opt, rest},
    multi::separated_list0,
    number::complete::float,
};
use std::{cmp::Ordering, io::BufRead, path::PathBuf};

#[derive(Debug, PartialEq)]
pub struct Point {
    pub v: i32,
    pub t: i32,
    pub n: i32,
}

#[derive(Debug, PartialEq)]
pub struct Face {
    pub points: Vec<Point>,
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub faces: Vec<Face>,
    pub material: String,
}

impl Chunk {
    pub fn new(material: String) -> Chunk {
        Chunk {
            faces: Vec::new(),
            material,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Obj {
    pub mtl_lib: PathBuf,

    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub texcoords: Vec<[f32; 2]>,
    pub chunks: Vec<Chunk>,
}

impl Obj {
    pub fn index_vertex(&self, point: &Point) -> [f32; 3] {
        index_wavefront_vec(&self.vertices, point.v)
    }

    pub fn index_texcoord(&self, point: &Point) -> [f32; 2] {
        index_wavefront_vec(&self.texcoords, point.t)
    }

    pub fn index_normal(&self, point: &Point) -> [f32; 3] {
        index_wavefront_vec(&self.normals, point.n)
    }
}

fn index_wavefront_vec<T: Default + Copy>(v: &[T], i: i32) -> T {
    match i.cmp(&0) {
        Ordering::Equal => Default::default(),
        Ordering::Less => v[((v.len() as i32) + i) as usize],
        Ordering::Greater => v[(i - 1) as usize],
    }
}

fn tagged<'a, O>(
    name: &str,
    data: impl Fn(&'a str) -> IResult<&'a str, O>,
    input: &'a str,
) -> IResult<&'a str, O> {
    let (input, _) = tag_no_case(name)(input)?;
    let (input, _) = space0(input)?;
    data(input)
}

fn vec2(input: &str) -> IResult<&str, [f32; 2]> {
    let (input, x) = float(input)?;
    let (input, _) = space0(input)?;
    let (input, y) = float(input)?;
    Ok((input, [x, y]))
}

fn vec3(input: &str) -> IResult<&str, [f32; 3]> {
    let (input, x) = float(input)?;
    let (input, _) = space0(input)?;
    let (input, y) = float(input)?;
    let (input, _) = space0(input)?;
    let (input, z) = float(input)?;
    Ok((input, [x, y, z]))
}

fn i32_or_zero(input: &str) -> IResult<&str, i32> {
    opt(i32)
        .parse(input)
        .map(|(input, n)| (input, n.unwrap_or(0)))
}

fn point(input: &str) -> IResult<&str, Point> {
    let (input, v) = i32(input)?;
    let (input, _) = char('/')(input)?;
    let (input, t) = i32_or_zero(input)?;
    let (input, _) = char('/')(input)?;
    let (input, n) = i32_or_zero(input)?;
    Ok((input, Point { v, t, n }))
}

fn face(input: &str) -> IResult<&str, Face> {
    separated_list0(space1, point)
        .parse(input)
        .map(|(input, points)| (input, Face { points }))
}

pub fn obj<R>(input: &mut R) -> Obj
where
    R: BufRead,
{
    let mut mtl_lib = PathBuf::new();
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut texcoords: Vec<[f32; 2]> = Vec::new();

    let mut line = String::new();
    while input.read_line(&mut line).unwrap() > 0 {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            line.clear();
            continue;
        }

        if let Ok((_, x)) = tagged("mtllib", rest, trimmed) {
            mtl_lib = PathBuf::from(x);
        } else if let Ok((_, x)) = tagged("usemtl", rest, trimmed) {
            chunks.push(Chunk::new(x.to_owned()));
        } else if let Ok((_, x)) = tagged("v", vec3, trimmed) {
            vertices.push(x);
        } else if let Ok((_, x)) = tagged("vn", vec3, trimmed) {
            normals.push(x);
        } else if let Ok((_, x)) = tagged("vt", vec2, trimmed) {
            texcoords.push(x);
        } else if let Ok((_, x)) = tagged("f", face, trimmed) {
            chunks.last_mut().unwrap().faces.push(x);
        } else if let Ok((_, _)) = tagged("g", rest, trimmed) {
            // TODO: not supported
        } else if let Ok((_, _)) = tagged("o", rest, trimmed) {
            // TODO: not supported
        } else if let Ok((_, _)) = tagged("s", rest, trimmed) {
            // TODO: not supported
        } else {
            panic!("Unexpected line: \"{line}\"");
        }
        line.clear();
    }

    Obj {
        mtl_lib,
        vertices,
        normals,
        texcoords,
        chunks,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj_test(str: &str) -> Obj {
        obj(&mut str.as_bytes())
    }

    #[test]
    fn test_i32_or_zero() {
        assert_eq!(i32_or_zero("1"), Ok(("", 1)));
        assert_eq!(i32_or_zero("-1"), Ok(("", -1)));
        assert_eq!(i32_or_zero(""), Ok(("", 0)));
    }

    #[test]
    fn test_vec2() {
        assert_eq!(vec2("0 0"), Ok(("", [0., 0.])));
        assert_eq!(vec2("1 2"), Ok(("", [1., 2.])));
        assert_eq!(vec2("1. 2."), Ok(("", [1., 2.])));
        assert_eq!(vec2("-1. -2."), Ok(("", [-1., -2.])));
    }

    #[test]
    fn test_vec3() {
        assert_eq!(vec3("0 0 0"), Ok(("", [0., 0., 0.])));
        assert_eq!(vec3("1 2 3"), Ok(("", [1., 2., 3.])));
        assert_eq!(vec3("1. 2. 3."), Ok(("", [1., 2., 3.])));
        assert_eq!(vec3("-1. -2. -3."), Ok(("", [-1., -2., -3.])));
    }

    #[test]
    fn test_point() {
        assert_eq!(point("1/2/3"), Ok(("", Point { v: 1, t: 2, n: 3 })));
        assert_eq!(point("1//3"), Ok(("", Point { v: 1, t: 0, n: 3 })));
    }

    #[test]
    fn test_data() {
        assert_eq!(obj_test("v 1 2 3").vertices, [[1., 2., 3.]]);
        assert_eq!(obj_test("vt 1 2").texcoords, [[1., 2.]]);
        assert_eq!(obj_test("vn 1 2 3").normals, [[1., 2., 3.]]);
    }

    #[test]
    fn test_faces() {
        assert_eq!(
            obj_test("usemtl m1\nf 1/2/3").chunks,
            [Chunk {
                faces: vec![Face {
                    points: vec![Point { v: 1, t: 2, n: 3 },]
                }],
                material: "m1".to_string()
            }]
        );
        assert_eq!(
            obj_test("usemtl m1\nf 1//3").chunks,
            [Chunk {
                faces: vec![Face {
                    points: vec![Point { v: 1, t: 0, n: 3 },]
                }],
                material: "m1".to_string()
            }]
        );
        assert_eq!(
            obj_test("usemtl m1\nf 1//").chunks,
            [Chunk {
                faces: vec![Face {
                    points: vec![Point { v: 1, t: 0, n: 0 },]
                }],
                material: "m1".to_string()
            }]
        );
        assert_eq!(
            obj_test("usemtl m1\nf 1/2/3 4/5/6 7/8/9").chunks,
            [Chunk {
                faces: vec![Face {
                    points: vec![
                        Point { v: 1, t: 2, n: 3 },
                        Point { v: 4, t: 5, n: 6 },
                        Point { v: 7, t: 8, n: 9 }
                    ]
                }],
                material: "m1".to_string()
            }]
        );
        assert_eq!(
            obj_test("usemtl m1\nf 1/2/3 4/5/6 7/8/9 10/11/12").chunks,
            [Chunk {
                faces: vec![Face {
                    points: vec![
                        Point { v: 1, t: 2, n: 3 },
                        Point { v: 4, t: 5, n: 6 },
                        Point { v: 7, t: 8, n: 9 },
                        Point {
                            v: 10,
                            t: 11,
                            n: 12
                        }
                    ]
                }],
                material: "m1".to_string()
            }]
        );
    }

    #[test]
    fn test_usemtl() {
        assert_eq!(
            obj_test("usemtl m1").chunks,
            [Chunk {
                faces: vec![],
                material: "m1".to_string(),
            }]
        );
    }

    #[test]
    fn test_mtllib() {
        assert_eq!(
            obj_test("mtllib test.mtl").mtl_lib,
            PathBuf::from("test.mtl")
        );
    }

    #[test]
    fn test_comment() {
        assert_eq!(obj_test("# comment\nusemtl m1").chunks.len(), 1);
    }

    #[test]
    fn test_unsupported() {
        assert_eq!(obj_test("g mesh1 model").chunks.len(), 0);
        assert_eq!(obj_test("o todo").chunks.len(), 0);
        assert_eq!(obj_test("s todo").chunks.len(), 0);
    }
}
