use nalgebra::Vector3;
use smallvec::SmallVec;

use crate::geometry::{
    aabb::Aabb,
    aap::{Aap, Axis},
    algorithms::clip_triangle_aabb,
    triangle::Triangle,
};

use super::build::{KdBox, KdSplit};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SplitKind {
    Start,
    End,
}

#[derive(Debug, PartialEq)]
pub struct PerfectSplit {
    pub axis: Axis,
    pub kind: SplitKind,
    pub distance: f32,
}

impl PerfectSplit {
    fn new_x(kind: SplitKind, distance: f32) -> Self {
        PerfectSplit {
            axis: Axis::X,
            kind,
            distance,
        }
    }

    fn new_y(kind: SplitKind, distance: f32) -> Self {
        PerfectSplit {
            axis: Axis::Y,
            kind,
            distance,
        }
    }

    fn new_z(kind: SplitKind, distance: f32) -> Self {
        PerfectSplit {
            axis: Axis::Z,
            kind,
            distance,
        }
    }

    fn total_cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.axis
            .cmp(&other.axis)
            .then(self.kind.cmp(&other.kind))
            .then(f32::total_cmp(&self.distance, &other.distance))
    }
}

pub fn clip_triangles(
    triangles: &[Triangle],
    parent: &KdBox,
) -> Vec<(u32, SmallVec<[Vector3<f32>; 18]>)> {
    parent
        .triangle_indices
        .iter()
        .filter_map(|i| {
            let triangle = &triangles[*i as usize];
            let clipped = clip_triangle_aabb(&triangle, &parent.boundary);
            (!clipped.is_empty()).then_some((*i, clipped))
        })
        .collect()
}

pub fn perfect_splits(
    clipped_triangles: &[(u32, SmallVec<[Vector3<f32>; 18]>)],
) -> Vec<PerfectSplit> {
    let mut points = clipped_triangles
        .iter()
        .flat_map(|clipped_triangle| {
            let start = (clipped_triangle.1[0], clipped_triangle.1[0]);
            let (min, max) = clipped_triangle
                .1
                .iter()
                .fold(start, |(min, max), b| (min.inf(b), max.sup(b)));
            vec![
                PerfectSplit::new_x(SplitKind::Start, min.x),
                PerfectSplit::new_x(SplitKind::End, max.x),
                PerfectSplit::new_y(SplitKind::Start, min.y),
                PerfectSplit::new_y(SplitKind::End, max.y),
                PerfectSplit::new_z(SplitKind::Start, min.z),
                PerfectSplit::new_z(SplitKind::End, max.z),
            ]
        })
        .collect::<Vec<_>>();
    points.sort_unstable_by(PerfectSplit::total_cmp);
    points
}

#[cfg(test)]
mod tests {
    use nalgebra::vector;

    use crate::geometry::aabb::Aabb;

    use super::*;

    #[test]
    fn test() {
        let triangle = Triangle {
            v0: vector![0.0, 0.0, 0.0],
            v1: vector![1.0, 0.0, 0.0],
            v2: vector![1.0, 1.0, 0.0],
        };
        let triangles = [triangle];
        let parent = KdBox {
            boundary: Aabb::from_extents(&vector![0.0, 0.0, 0.0], &vector![2.0, 1.0, 1.0]),
            triangle_indices: vec![0],
        };
        let clipped_triangles = clip_triangles(&triangles, &parent);

        let actual = perfect_splits(&clipped_triangles);

        let expected = vec![
            PerfectSplit::new_x(SplitKind::Start, 0.0),
            PerfectSplit::new_x(SplitKind::End, 1.0),
            PerfectSplit::new_y(SplitKind::Start, 0.0),
            PerfectSplit::new_y(SplitKind::End, 1.0),
            PerfectSplit::new_z(SplitKind::Start, 0.0),
            PerfectSplit::new_z(SplitKind::End, 0.0),
        ];
        assert_eq!(actual, expected);
    }
}

pub enum Spanning {
    Left,
    Plane,
    Right,
    Both,
}

pub fn partition_triangles(
    clipped: &[(u32, SmallVec<[Vector3<f32>; 18]>)],
    plane: &Aap,
) -> (Vec<u32>, Vec<u32>) {
    let mut left_triangles: Vec<u32> = Vec::new();
    let mut right_triangles: Vec<u32> = Vec::new();
    for (i, points) in clipped {
        let (a, b) = points.iter().fold(
            (points[0][plane.axis], points[0][plane.axis]),
            |(a, b), p| (a.min(p[plane.axis]), b.max(p[plane.axis])),
        );
        let planar = a == plane.distance && b == plane.distance;
        let left = a < plane.distance;
        let right = b > plane.distance;
        // TODO: What to do with planar triangles?
        if left || planar {
            left_triangles.push(*i);
        }
        if right || planar {
            right_triangles.push(*i);
        }
    }
    (left_triangles, right_triangles)
}

#[cfg(test)]
mod partition_triangles_tests {
    use nalgebra::vector;

    use crate::geometry::aabb::Aabb;

    use super::*;

    #[test]
    fn test() {
        let triangle0 = Triangle {
            v0: vector![0.0, 0.0, 0.0],
            v1: vector![1.0, 0.0, 0.0],
            v2: vector![1.0, 1.0, 0.0],
        };
        let triangle1 = Triangle {
            v0: vector![1.0, 0.0, 0.0],
            v1: vector![1.0, 1.0, 0.0],
            v2: vector![1.0, 1.0, 1.0],
        };
        let triangle2 = Triangle {
            v0: vector![1.0, 0.0, 0.0],
            v1: vector![2.0, 0.0, 0.0],
            v2: vector![2.0, 1.0, 0.0],
        };
        let triangles = [triangle0, triangle1, triangle2];
        let aabb = Aabb::from_extents(&vector![0.0, 0.0, 0.0], &vector![2.0, 1.0, 1.0]);
        let clipped: Vec<_> = triangles
            .iter()
            .enumerate()
            .map(|(i, t)| (i as u32, clip_triangle_aabb(&t, &aabb)))
            .collect();
        let plane = Aap {
            axis: Axis::X,
            distance: 1.0,
        };

        let actual = partition_triangles(clipped.as_slice(), &plane);

        assert_eq!(actual, (vec![0, 1], vec![1, 2]));
    }
}

pub fn split_and_partition(
    clipped: &[(u32, SmallVec<[Vector3<f32>; 18]>)],
    aabb: &Aabb,
    plane: Aap,
) -> KdSplit {
    let (left_aabb, right_aabb) = aabb.split(&plane);
    let (left_triangles, right_triangles) = partition_triangles(&clipped, &plane);
    KdSplit {
        plane,
        left: KdBox {
            boundary: left_aabb,
            triangle_indices: left_triangles,
        },
        right: KdBox {
            boundary: right_aabb,
            triangle_indices: right_triangles,
        },
    }
}
