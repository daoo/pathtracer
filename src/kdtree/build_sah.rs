use nalgebra::vector;
use rayon::prelude::*;

use crate::{
    geometry::{aabb::Aabb, aap::Aap, algorithms::triangles_bounding_box, triangle::Triangle},
    kdtree::split::{clip_triangle, PerfectSplit},
};

use super::{
    build::{KdBox, KdSplit, KdTreeBuilder},
    split::{split_and_partition, ClippedTriangle},
    KdNode, KdTree,
};

pub struct SahKdTreeBuilder {
    pub traverse_cost: f32,
    pub intersect_cost: f32,
    pub empty_factor: f32,
    pub triangles: Vec<Triangle>,
}

impl SahKdTreeBuilder {
    fn calculate_sah_cost_helper(&self, probability: (f32, f32), counts: (usize, usize)) -> f32 {
        debug_assert!(probability.0 >= 0.0 && probability.1 >= 0.0);
        debug_assert!(probability.0 > 0.0 || probability.1 > 0.0);
        let empty_factor = if counts.0 == 0 || counts.1 == 0 {
            self.empty_factor
        } else {
            1.0
        };
        let intersect_cost = self.intersect_cost
            * (probability.0 * counts.0 as f32 + probability.1 * counts.1 as f32);
        empty_factor * (self.traverse_cost + intersect_cost)
    }

    fn calculate_sah_cost(&self, parent: &Aabb, split: &KdSplit) -> f32 {
        let probability_left = split.left.boundary.surface_area() / parent.surface_area();
        let probability_right = split.right.boundary.surface_area() / parent.surface_area();
        let probability = (probability_left, probability_right);
        let counts = (
            split.left.triangle_indices.len(),
            split.right.triangle_indices.len(),
        );
        self.calculate_sah_cost_helper(probability, counts)
    }

    fn split_and_calculate_cost(
        &self,
        parent: &KdBox,
        plane: Aap,
        clipped: &[ClippedTriangle],
    ) -> (KdSplit, f32) {
        let split = split_and_partition(&clipped, &parent.boundary, plane);
        let cost = self.calculate_sah_cost(&parent.boundary, &split);
        (split, cost)
    }
}

impl KdTreeBuilder for SahKdTreeBuilder {
    fn starting_box(&self) -> KdBox {
        KdBox {
            boundary: triangles_bounding_box(&self.triangles).enlarge(&vector![0.5, 0.5, 0.5]),
            triangle_indices: (0u32..self.triangles.len() as u32).collect(),
        }
    }

    fn find_best_split(&self, _: u32, parent: &KdBox) -> Option<KdSplit> {
        debug_assert!(parent.boundary.volume() > 0.0);
        debug_assert!(!parent.triangle_indices.is_empty());

        let min = parent.boundary.min();
        let max = parent.boundary.max();

        let min_by_snd = |a: (_, f32), b: (_, f32)| if a.1 <= b.1 { a } else { b };

        let clipped_triangles = parent
            .triangle_indices
            .iter()
            .filter_map(|i| clip_triangle(&self.triangles, &parent.boundary, *i))
            .collect::<Vec<_>>();
        let mut splits = clipped_triangles
            .iter()
            .flat_map(|clipped| clipped.perfect_splits())
            .collect::<Vec<_>>();
        splits.sort_unstable_by(PerfectSplit::total_cmp);
        splits.dedup();
        splits
            .par_iter()
            .filter_map(|s| {
                if s.distance > min[s.axis] && s.distance < max[s.axis] {
                    let plane = Aap {
                        axis: s.axis,
                        distance: s.distance,
                    };
                    Some(self.split_and_calculate_cost(parent, plane, &clipped_triangles))
                } else {
                    None
                }
            })
            .reduce_with(min_by_snd)
            .map(|a| a.0)
    }

    fn terminate(&self, parent: &KdBox, split: &KdSplit) -> bool {
        let split_cost = self.calculate_sah_cost(&parent.boundary, split);
        split_cost >= self.intersect_cost * parent.triangle_indices.len() as f32
    }

    fn make_tree(self, root: Box<KdNode>) -> KdTree {
        KdTree {
            root,
            triangles: self.triangles,
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::vector;

    use crate::{
        geometry::aap::Axis,
        kdtree::{build::build_kdtree, KdNode},
    };

    use super::*;

    #[test]
    fn test() {
        let triangles = vec![Triangle {
            v0: vector![0.0, 0.0, 0.0],
            v1: vector![1.0, 0.0, 0.0],
            v2: vector![1.0, 1.0, 0.0],
        }];
        let builder = SahKdTreeBuilder {
            traverse_cost: 0.1,
            intersect_cost: 1.0,
            empty_factor: 0.8,
            triangles,
        };
        let tree = build_kdtree(builder, 6);

        let expected = KdNode::new_node(
            Axis::X,
            0.0,
            KdNode::new_leaf(vec![]),
            KdNode::new_node(
                Axis::X,
                1.0,
                KdNode::new_node(
                    Axis::Y,
                    0.0,
                    KdNode::new_leaf(vec![]),
                    KdNode::new_node(
                        Axis::Y,
                        1.0,
                        KdNode::new_leaf(vec![0]),
                        KdNode::new_leaf(vec![]),
                    ),
                ),
                KdNode::new_leaf(vec![]),
            ),
        );
        assert_eq!(tree.root, expected);
    }
}
