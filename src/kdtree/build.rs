use std::ops::Range;

use crate::geometry::aabb::*;
use crate::geometry::algorithms::*;
use crate::kdtree::*;
use nalgebra::vector;

#[derive(Debug)]
struct KdBox {
    boundary: Aabb,
    triangle_indices: Vec<u32>,
}

#[derive(Debug)]
struct KdSplit {
    left: KdBox,
    right: KdBox,
}

struct KdTreeBuilder {
    max_depth: u32,
    triangles: Vec<Triangle>,
}

impl KdTreeBuilder {
    fn split_box(&self, parent: KdBox, plane: &Aap) -> KdSplit {
        let (left_aabb, right_aabb) = parent.boundary.split(plane);
        debug_assert!(
            left_aabb.size()[plane.axis] > 0.1,
            "left_aabb too small {:?} {:?}",
            plane,
            left_aabb
        );
        debug_assert!(
            right_aabb.size()[plane.axis] > 0.1,
            "right_aabb to small {:?} {:?}",
            plane,
            right_aabb
        );
        let mut left_triangle_indices: Vec<u32> = Vec::new();
        let mut right_triangle_indices: Vec<u32> = Vec::new();
        for triangle_index in parent.triangle_indices {
            let triangle = &self.triangles[triangle_index as usize];
            let in_left = intersect_triangle_aabb(&triangle, &left_aabb);
            let in_right = intersect_triangle_aabb(&triangle, &right_aabb);
            debug_assert!(in_left || in_right);
            if in_left {
                left_triangle_indices.push(triangle_index);
            }
            if in_right {
                right_triangle_indices.push(triangle_index);
            }
        }
        KdSplit {
            left: KdBox {
                boundary: left_aabb,
                triangle_indices: left_triangle_indices,
            },
            right: KdBox {
                boundary: right_aabb,
                triangle_indices: right_triangle_indices,
            },
        }
    }

    fn potential_split_points(
        &self,
        triangle_indices: &Vec<u32>,
        axis: Axis,
        boundary: &Aabb,
    ) -> Vec<f32> {
        let min = boundary.min()[axis] + 0.1;
        let max = boundary.max()[axis] - 0.1;
        let mut points = triangle_indices
            .into_iter()
            .flat_map(|i| {
                let triangle = &self.triangles[*i as usize];
                [triangle.min()[axis] - 0.1, triangle.max()[axis] + 0.1]
            })
            .filter(|p| p > &min && p < &max)
            .collect::<Vec<_>>();
        points.sort_by(f32::total_cmp);
        points
    }

    fn build(&self, depth: u32, axis: Axis, parent: KdBox) -> Box<KdNode> {
        if depth >= self.max_depth || parent.triangle_indices.is_empty() {
            return Box::new(KdNode::Leaf(parent.triangle_indices));
        }

        let points = self.potential_split_points(&parent.triangle_indices, axis, &parent.boundary);
        if points.is_empty() {
            return Box::new(KdNode::Leaf(parent.triangle_indices));
        }

        let plane = Aap {
            axis,
            distance: median(&points),
        };
        let split = self.split_box(parent, &plane);
        let left = self.build(depth + 1, NEXT_AXIS[axis], split.left);
        let right = self.build(depth + 1, NEXT_AXIS[axis], split.right);
        Box::new(KdNode::Node { plane, left, right })
    }
}

fn median(values: &[f32]) -> f32 {
    debug_assert!(!values.is_empty());
    if values.len() == 1 {
        return values[0];
    }

    let middle = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[middle - 1] + values[middle]) / 2.
    } else {
        values[middle]
    }
}

static NEXT_AXIS: [Axis; 3] = [Axis::Y, Axis::Z, Axis::X];

pub fn build_kdtree_median(max_depth: u32, triangles: Vec<Triangle>) -> KdTree {
    let kdbox: KdBox = KdBox {
        boundary: triangles_bounding_box(&triangles).enlarge(&vector![0.1, 0.1, 0.1]),
        triangle_indices: Range {
            start: 0u32,
            end: triangles.len() as u32,
        }
        .into_iter()
        .collect(),
    };
    let builder = KdTreeBuilder {
        max_depth,
        triangles,
    };
    KdTree {
        root: builder.build(0, Axis::X, kdbox),
        triangles: builder.triangles,
    }
}
