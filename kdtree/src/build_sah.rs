use nalgebra::Vector3;
use rayon::prelude::*;

use geometry::{aabb::Aabb, aap::Aap, bound::geometries_bounding_box, triangle::Triangle};

use crate::split::perfect_splits;

use super::{
    build::{KdCell, KdSplit, KdTreeBuilder},
    split::split_and_partition,
    KdNode, KdTree,
};

pub struct SahKdTreeBuilder {
    pub traverse_cost: f32,
    pub intersect_cost: f32,
    pub empty_factor: f32,
    pub geometries: Vec<Triangle>,
}

impl SahKdTreeBuilder {
    fn calculate_sah_cost(&self, probability: (f32, f32), counts: (usize, usize)) -> f32 {
        debug_assert!((0.0..=1.0).contains(&probability.0) && (0.0..=1.0).contains(&probability.1));
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

    fn split_and_calculate_cost(
        &self,
        cell: &KdCell,
        plane: Aap,
        clipped: &[(u32, Aabb)],
    ) -> Option<(KdSplit, f32)> {
        let mut split = split_and_partition(clipped, cell.boundary(), plane);
        // TODO: Place planes to the left or to the right depending on what gives best cost.
        if (split.left_aabb.volume() == 0.0 || split.right_aabb.volume() == 0.0)
            && split.middle_indices.is_empty()
        {
            return None;
        }
        let probability_left = split.left_aabb.surface_area() / cell.boundary().surface_area();
        let probability_right = split.right_aabb.surface_area() / cell.boundary().surface_area();
        let probability = (probability_left, probability_right);
        let counts = (
            split.left_indices.len() + split.middle_indices.len(),
            split.right_indices.len() + split.middle_indices.len(),
        );
        let cost = self.calculate_sah_cost(probability, counts);
        split.left_indices.extend(&split.middle_indices);
        split.right_indices.extend(split.middle_indices);
        let left = KdCell::new(split.left_aabb, split.left_indices);
        let right = KdCell::new(split.right_aabb, split.right_indices);
        Some((KdSplit { plane, left, right }, cost))
    }
}

impl KdTreeBuilder for SahKdTreeBuilder {
    fn starting_box(&self) -> KdCell {
        KdCell::new(
            geometries_bounding_box(&self.geometries).enlarge(&Vector3::new(1.0, 1.0, 1.0)),
            (0u32..self.geometries.len() as u32).collect(),
        )
    }

    fn find_best_split(&self, _: u32, cell: &KdCell) -> Option<KdSplit> {
        debug_assert!(
            !cell.indices().is_empty(),
            "splitting a kd-cell with no geometries only worsens performance"
        );

        let min_by_snd = |a: (_, f32), b: (_, f32)| if a.1 <= b.1 { a } else { b };

        let (clipped, mut splits) = perfect_splits(&self.geometries, cell);
        splits.sort_unstable_by(Aap::total_cmp);
        splits.dedup();
        splits
            .into_par_iter()
            .filter_map(|plane| self.split_and_calculate_cost(cell, plane, &clipped))
            .reduce_with(min_by_snd)
            .map(|a| a.0)
    }

    fn terminate(&self, cell: &KdCell, split: &KdSplit) -> bool {
        let probability_left =
            split.left.boundary().surface_area() / cell.boundary().surface_area();
        let probability_right =
            split.right.boundary().surface_area() / cell.boundary().surface_area();
        let probability = (probability_left, probability_right);
        let counts = (split.left.indices().len(), split.right.indices().len());
        let split_cost = self.calculate_sah_cost(probability, counts);
        let intersect_cost = self.intersect_cost * cell.indices().len() as f32;
        split_cost >= intersect_cost
    }

    fn make_tree(self, root: Box<KdNode>) -> KdTree {
        KdTree {
            root,
            geometries: self.geometries,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::build::build_kdtree;

    use super::*;

    #[test]
    fn non_axially_aligned_triangle() {
        let geometries = vec![Triangle {
            v0: Vector3::new(0.0, 0.0, 0.0),
            v1: Vector3::new(1.0, 0.0, 0.0),
            v2: Vector3::new(1.0, 1.0, 1.0),
        }];
        let builder = SahKdTreeBuilder {
            traverse_cost: 0.1,
            intersect_cost: 1.0,
            empty_factor: 0.8,
            geometries,
        };
        let tree = build_kdtree(builder, 6);

        let expected = KdNode::new_node(
            Aap::new_x(0.0),
            KdNode::empty(),
            KdNode::new_node(
                Aap::new_x(1.0),
                KdNode::new_node(
                    Aap::new_y(0.0),
                    KdNode::empty(),
                    KdNode::new_node(
                        Aap::new_y(1.0),
                        KdNode::new_node(
                            Aap::new_z(0.0),
                            KdNode::empty(),
                            KdNode::new_node(
                                Aap::new_z(1.0),
                                KdNode::new_leaf(vec![0]),
                                KdNode::empty(),
                            ),
                        ),
                        KdNode::empty(),
                    ),
                ),
                KdNode::empty(),
            ),
        );
        assert_eq!(
            tree.root, expected,
            "\n   actual: {}\n expected: {}",
            tree.root, expected
        );
    }

    #[test]
    fn axially_aligned_triangle() {
        let geometries = vec![Triangle {
            v0: Vector3::new(0.0, 0.0, 0.0),
            v1: Vector3::new(1.0, 0.0, 0.0),
            v2: Vector3::new(1.0, 1.0, 0.0),
        }];
        let builder = SahKdTreeBuilder {
            traverse_cost: 1.0,
            intersect_cost: 10.0,
            empty_factor: 0.8,
            geometries,
        };
        let tree = build_kdtree(builder, 6);

        let expected = KdNode::new_node(
            Aap::new_x(0.0),
            KdNode::empty(),
            KdNode::new_node(
                Aap::new_x(1.0),
                KdNode::new_node(
                    Aap::new_y(0.0),
                    KdNode::empty(),
                    KdNode::new_node(Aap::new_y(1.0), KdNode::new_leaf(vec![0]), KdNode::empty()),
                ),
                KdNode::empty(),
            ),
        );
        assert_eq!(
            tree.root, expected,
            "\n   actual: {}\n expected: {}",
            tree.root, expected
        );
    }
}
