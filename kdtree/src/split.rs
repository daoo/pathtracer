use nalgebra::Vector3;

use geometry::{
    aabb::Aabb,
    aap::{Aap, Axis},
    algorithms::clip_triangle_aabb,
    triangle::Triangle,
};

#[derive(Debug, PartialEq)]
pub struct PerfectSplit {
    pub axis: Axis,
    pub distance: f32,
}

impl PerfectSplit {
    fn new_x(distance: f32) -> Self {
        PerfectSplit {
            axis: Axis::X,
            distance,
        }
    }

    fn new_y(distance: f32) -> Self {
        PerfectSplit {
            axis: Axis::Y,
            distance,
        }
    }

    fn new_z(distance: f32) -> Self {
        PerfectSplit {
            axis: Axis::Z,
            distance,
        }
    }

    pub fn total_cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.axis
            .cmp(&other.axis)
            .then(f32::total_cmp(&self.distance, &other.distance))
    }
}

#[derive(Debug, PartialEq)]
pub struct ClippedTriangle {
    index: u32,
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl ClippedTriangle {
    pub fn perfect_splits(&self) -> [PerfectSplit; 6] {
        [
            PerfectSplit::new_x(self.min.x),
            PerfectSplit::new_x(self.max.x),
            PerfectSplit::new_y(self.min.y),
            PerfectSplit::new_y(self.max.y),
            PerfectSplit::new_z(self.min.z),
            PerfectSplit::new_z(self.max.z),
        ]
    }
}

pub fn clip_triangle(triangles: &[Triangle], aabb: &Aabb, index: u32) -> Option<ClippedTriangle> {
    let triangle = &triangles[index as usize];
    let clipped = clip_triangle_aabb(triangle, aabb);
    if clipped.is_empty() {
        return None;
    }
    let start = (clipped[0], clipped[0]);
    let (min, max) = clipped[1..]
        .iter()
        .fold(start, |(min, max), b| (min.inf(b), max.sup(b)));

    Some(ClippedTriangle { index, min, max })
}

#[cfg(test)]
mod clip_triangle_tests {
    use super::*;

    #[test]
    fn test() {
        let triangle = Triangle {
            v0: Vector3::new(0.0, 0.0, 0.0),
            v1: Vector3::new(1.0, 0.0, 0.0),
            v2: Vector3::new(1.0, 1.0, 0.0),
        };
        let aabb = Aabb::from_extents(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 1.0, 1.0));

        let actual = clip_triangle(&[triangle], &aabb, 0);

        let expected = ClippedTriangle {
            index: 0,
            min: Vector3::new(0.0, 0.0, 0.0),
            max: Vector3::new(1.0, 1.0, 0.0),
        };
        assert_eq!(actual, Some(expected));
    }
}

pub fn partition_triangles(
    clipped_triangles: &[ClippedTriangle],
    plane: &Aap,
) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut left_triangles: Vec<u32> = Vec::new();
    let mut middle_triangles: Vec<u32> = Vec::new();
    let mut right_triangles: Vec<u32> = Vec::new();
    for clipped in clipped_triangles {
        let planar =
            clipped.min[plane.axis] == plane.distance && clipped.max[plane.axis] == plane.distance;
        let left = clipped.min[plane.axis] < plane.distance;
        let right = clipped.max[plane.axis] > plane.distance;
        if left {
            left_triangles.push(clipped.index);
        }
        if planar {
            middle_triangles.push(clipped.index);
        }
        if right {
            right_triangles.push(clipped.index);
        }
    }
    (left_triangles, middle_triangles, right_triangles)
}

#[cfg(test)]
mod partition_triangles_tests {
    use super::*;

    #[test]
    fn test() {
        let triangle0 = ClippedTriangle {
            index: 0,
            min: Vector3::new(0.0, 0.0, 0.0),
            max: Vector3::new(1.0, 1.0, 0.0),
        };
        let triangle1 = ClippedTriangle {
            index: 1,
            min: Vector3::new(1.0, 0.0, 0.0),
            max: Vector3::new(1.0, 1.0, 1.0),
        };
        let triangle2 = ClippedTriangle {
            index: 2,
            min: Vector3::new(1.0, 0.0, 0.0),
            max: Vector3::new(2.0, 1.0, 0.0),
        };
        let clipped = [triangle0, triangle1, triangle2];
        let plane = Aap {
            axis: Axis::X,
            distance: 1.0,
        };

        let actual = partition_triangles(clipped.as_slice(), &plane);

        assert_eq!(actual, (vec![0], vec![1], vec![2]));
    }
}

pub struct SplitPartitioning {
    pub left_aabb: Aabb,
    pub right_aabb: Aabb,
    pub left_triangle_indices: Vec<u32>,
    pub middle_triangle_indices: Vec<u32>,
    pub right_triangle_indices: Vec<u32>,
}

pub fn split_and_partition(
    clipped: &[ClippedTriangle],
    aabb: &Aabb,
    plane: Aap,
) -> SplitPartitioning {
    let (left_aabb, right_aabb) = aabb.split(&plane);
    let (left_triangle_indices, middle_triangle_indices, right_triangle_indices) =
        partition_triangles(clipped, &plane);
    SplitPartitioning {
        left_aabb,
        right_aabb,
        left_triangle_indices,
        middle_triangle_indices,
        right_triangle_indices,
    }
}
