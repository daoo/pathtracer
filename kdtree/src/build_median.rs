use nalgebra::Vector3;
use rayon::prelude::*;

use geometry::{
    aap::Aap, algorithms::triangles_bounding_box, axis::Axis, triangle::Triangle
};

use crate::split::ClippedTriangle;

use super::{
    build::{KdCell, KdSplit, KdTreeBuilder},
    split::{clip_triangle, split_and_partition},
    KdNode, KdTree,
};

pub struct MedianKdTreeBuilder {
    pub triangles: Vec<Triangle>,
}

fn median(splits: &[Aap]) -> Aap {
    if splits.len() == 1 {
        return splits[0];
    }

    let middle = splits.len() / 2;
    // TODO: If not evenly divisible by 2 this biases towards the first in order.
    splits[middle]
}

impl KdTreeBuilder for MedianKdTreeBuilder {
    fn starting_box(&self) -> KdCell {
        KdCell::new(
            triangles_bounding_box(&self.triangles).enlarge(&Vector3::new(0.5, 0.5, 0.5)),
            (0u32..self.triangles.len() as u32).collect(),
        )
    }

    fn find_best_split(&self, depth: u32, cell: &KdCell) -> Option<KdSplit> {
        let axis = Axis::from_u32(depth % 3);
        let min = cell.boundary().min()[axis];
        let max = cell.boundary().max()[axis];
        let clipped_triangles = cell
            .triangle_indices()
            .par_iter()
            .filter_map(|i| clip_triangle(&self.triangles, cell.boundary(), *i))
            .collect::<Vec<_>>();
        let planes = clipped_triangles
            .iter()
            .flat_map(ClippedTriangle::perfect_splits)
            .filter_map(|s| {
                (s.axis == axis && s.distance > min && s.distance < max).then_some(Aap {
                    axis: s.axis,
                    distance: s.distance,
                })
            })
            .collect::<Vec<_>>();
        if planes.is_empty() {
            return None;
        }
        let plane = median(&planes);
        let mut split = split_and_partition(&clipped_triangles, cell.boundary(), plane);
        split
            .left_triangle_indices
            .extend(split.middle_triangle_indices);
        let left = KdCell::new(split.left_aabb, split.left_triangle_indices);
        let right = KdCell::new(split.right_aabb, split.right_triangle_indices);
        Some(KdSplit { plane, left, right })
    }

    fn terminate(&self, _: &KdCell, _: &super::build::KdSplit) -> bool {
        false
    }

    fn make_tree(self, root: Box<KdNode>) -> KdTree {
        KdTree {
            root,
            triangles: self.triangles,
        }
    }
}
