use nalgebra::Vector3;

use geometry::{aap::Aap, axis::Axis, bound::geometries_bounding_box, triangle::Triangle};

use crate::split::perfect_splits;

use super::{
    build::{KdCell, KdSplit, KdTreeBuilder},
    split::split_and_partition,
    KdNode, KdTree,
};

pub struct MedianKdTreeBuilder {
    pub geometries: Vec<Triangle>,
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
            geometries_bounding_box(&self.geometries).enlarge(&Vector3::new(0.5, 0.5, 0.5)),
            (0u32..self.geometries.len() as u32).collect(),
        )
    }

    fn find_best_split(&self, depth: u32, cell: &KdCell) -> Option<KdSplit> {
        let axis = Axis::from_u32(depth % 3);
        let min = cell.boundary().min()[axis];
        let max = cell.boundary().max()[axis];
        let (clipped, splits) = perfect_splits(&self.geometries, cell);
        let splits = splits
            .into_iter()
            .filter(|s| s.axis == axis && s.distance > min && s.distance < max)
            .collect::<Vec<_>>();
        if splits.is_empty() {
            return None;
        }
        let plane = median(&splits);
        let mut split = split_and_partition(&clipped, cell.boundary(), plane);
        split.left_indices.extend(split.middle_indices);
        let left = KdCell::new(split.left_aabb, split.left_indices);
        let right = KdCell::new(split.right_aabb, split.right_indices);
        Some(KdSplit { plane, left, right })
    }

    fn terminate(&self, _: &KdCell, _: &super::build::KdSplit) -> bool {
        false
    }

    fn make_tree(self, root: Box<KdNode>) -> KdTree {
        KdTree {
            root,
            geometries: self.geometries,
        }
    }
}
