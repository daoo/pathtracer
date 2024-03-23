use nalgebra::vector;

use crate::geometry::{
    aap::{Aap, Axis},
    algorithms::triangles_bounding_box,
    triangle::Triangle,
};

use super::{
    build::{KdBox, KdSplit, KdTreeBuilder},
    split::{clip_triangles, perfect_splits, split_and_partition},
    KdNode, KdTree,
};

pub struct MedianKdTreeBuilder {
    pub triangles: Vec<Triangle>,
}

fn median(splits: &[Aap]) -> Aap {
    debug_assert!(!splits.is_empty());
    if splits.len() == 1 {
        return splits[0];
    }

    let middle = splits.len() / 2;
    // TODO: If not evenly divisible by 2 this biases towards the first in order.
    splits[middle]
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
        let min = parent.boundary.min()[axis];
        let max = parent.boundary.max()[axis];
        let clipped = clip_triangles(&self.triangles, parent);
        let planes = perfect_splits(&clipped)
            .into_iter()
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
        let best = median(&planes);
        Some(split_and_partition(&clipped, &parent.boundary, best))
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
