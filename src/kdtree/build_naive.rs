use nalgebra::vector;

use crate::{
    geometry::{
        aap::{Aap, Axis},
        algorithms::triangles_bounding_box,
        triangle::Triangle,
    },
    kdtree::KdNode,
};

use super::{
    build::{KdBox, KdTreeInputs},
    KdTree,
};

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

fn potential_split_points(inputs: &KdTreeInputs, parent: &KdBox, axis: Axis) -> Vec<f32> {
    let min = parent.boundary.min()[axis] + 0.1;
    let max = parent.boundary.max()[axis] - 0.1;
    let mut points = parent
        .triangle_indices
        .iter()
        .flat_map(|i| {
            let triangle = &inputs.triangles[*i as usize];
            [triangle.min()[axis] - 0.1, triangle.max()[axis] + 0.1]
        })
        .filter(|p| p > &min && p < &max)
        .collect::<Vec<_>>();
    points.sort_by(f32::total_cmp);
    points
}

fn build(inputs: &KdTreeInputs, depth: u32, axis: Axis, parent: KdBox) -> Box<KdNode> {
    if depth >= inputs.max_depth || parent.triangle_indices.is_empty() {
        return Box::new(KdNode::Leaf(parent.triangle_indices));
    }

    let points = potential_split_points(inputs, &parent, axis);
    if points.is_empty() {
        return Box::new(KdNode::Leaf(parent.triangle_indices));
    }

    let plane = Aap {
        axis,
        distance: median(&points),
    };
    let split = inputs.split_box(&parent, plane);
    const NEXT_AXIS: [Axis; 3] = [Axis::Y, Axis::Z, Axis::X];
    let left = build(inputs, depth + 1, NEXT_AXIS[axis], split.left);
    let right = build(inputs, depth + 1, NEXT_AXIS[axis], split.right);
    Box::new(KdNode::Node {
        plane: split.plane,
        left,
        right,
    })
}

pub fn build_kdtree_median(max_depth: u32, triangles: Vec<Triangle>) -> KdTree {
    let kdbox: KdBox = KdBox {
        boundary: triangles_bounding_box(&triangles).enlarge(&vector![0.1, 0.1, 0.1]),
        triangle_indices: (0u32..triangles.len() as u32).collect(),
    };
    let builder = KdTreeInputs {
        max_depth,
        triangles,
    };
    KdTree {
        root: build(&builder, 0, Axis::X, kdbox),
        triangles: builder.triangles,
    }
}
