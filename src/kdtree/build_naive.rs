use nalgebra::vector;

use crate::{
    geometry::{
        aap::{Aap, Axis},
        algorithms::triangles_bounding_box,
        triangle::Triangle,
    },
    kdtree::build::potential_split_points,
};

use super::{
    build::{split_box, KdBox, KdSplit, KdTreeBuilder},
    KdNode, KdTree,
};

pub struct MedianKdTreeBuilder {
    pub triangles: Vec<Triangle>,
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

impl KdTreeBuilder for MedianKdTreeBuilder {
    fn starting_box(&self) -> KdBox {
        KdBox {
            boundary: triangles_bounding_box(&self.triangles).enlarge(&vector![0.5, 0.5, 0.5]),
            triangle_indices: (0u32..self.triangles.len() as u32).collect(),
        }
    }

    fn find_best_split(&self, depth: u32, parent: &KdBox) -> Option<KdSplit> {
        let axis = Axis::from_u32(depth % 3);
        let points = potential_split_points(&self.triangles, &parent, axis);
        if points.is_empty() {
            return None;
        }
        let plane = Aap {
            axis,
            distance: median(&points),
        };
        Some(split_box(&self.triangles, &parent, plane))
    }

    fn terminate(&self, _: &KdBox, _: &super::build::KdSplit) -> bool {
        false
    }

    fn make_tree(self, root: Box<KdNode>) -> KdTree {
        KdTree {
            root,
            triangles: self.triangles,
        }
    }
}
