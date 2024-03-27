use nalgebra::Vector3;
use rayon::prelude::*;

use geometry::{aap::Aap, algorithms::triangles_bounding_box, triangle::Triangle};

use crate::split::clip_triangle;

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
    fn calculate_sah_cost(&self, probability: (f32, f32), counts: (usize, usize)) -> f32 {
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

    fn split_and_calculate_cost(
        &self,
        parent: &KdBox,
        plane: Aap,
        clipped: &[ClippedTriangle],
    ) -> (KdSplit, f32) {
        let split = split_and_partition(clipped, &parent.boundary, plane);
        // TODO: Place planes to the left or to the right depending on what gives best cost.
        let probability_left = split.left_aabb.surface_area() / parent.boundary.surface_area();
        let probability_right = split.right_aabb.surface_area() / parent.boundary.surface_area();
        let probability = (probability_left, probability_right);
        let counts = (
            split.left_triangle_indices.len() + split.middle_triangle_indices.len(),
            split.right_triangle_indices.len() + split.middle_triangle_indices.len(),
        );
        let cost = self.calculate_sah_cost(probability, counts);
        let left = KdBox {
            boundary: split.left_aabb,
            triangle_indices: [
                split.left_triangle_indices,
                split.middle_triangle_indices.clone(),
            ]
            .concat(),
        };
        let right = KdBox {
            boundary: split.right_aabb,
            triangle_indices: [split.right_triangle_indices, split.middle_triangle_indices]
                .concat(),
        };
        (KdSplit { plane, left, right }, cost)
    }
}

impl KdTreeBuilder for SahKdTreeBuilder {
    fn starting_box(&self) -> KdBox {
        KdBox {
            boundary: triangles_bounding_box(&self.triangles).enlarge(&Vector3::new(1.0, 1.0, 1.0)),
            triangle_indices: (0u32..self.triangles.len() as u32).collect(),
        }
    }

    fn find_best_split(&self, _: u32, parent: &KdBox) -> Option<KdSplit> {
        debug_assert!(!parent.boundary.is_empty());
        debug_assert!(!parent.triangle_indices.is_empty());

        let min_by_snd = |a: (_, f32), b: (_, f32)| if a.1 <= b.1 { a } else { b };

        let clipped_triangles = parent
            .triangle_indices
            .iter()
            .filter_map(|i| clip_triangle(&self.triangles, &parent.boundary, *i))
            .collect::<Vec<_>>();
        let mut splits = clipped_triangles
            .iter()
            .flat_map(ClippedTriangle::perfect_splits)
            .collect::<Vec<_>>();
        splits.sort_unstable_by(Aap::total_cmp);
        splits.dedup();
        splits
            .into_par_iter()
            .map(|plane| self.split_and_calculate_cost(parent, plane, &clipped_triangles))
            .reduce_with(min_by_snd)
            .map(|a| a.0)
    }

    fn terminate(&self, parent: &KdBox, split: &KdSplit) -> bool {
        let probability_left = split.left.boundary.surface_area() / parent.boundary.surface_area();
        let probability_right =
            split.right.boundary.surface_area() / parent.boundary.surface_area();
        let probability = (probability_left, probability_right);
        let counts = (
            split.left.triangle_indices.len(),
            split.right.triangle_indices.len(),
        );
        let split_cost = self.calculate_sah_cost(probability, counts);
        let intersect_cost = self.intersect_cost * parent.triangle_indices.len() as f32;
        split_cost >= intersect_cost
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
    use crate::build::build_kdtree;

    use super::*;

    #[test]
    fn test_non_axially_aligned_triangle() {
        let triangles = vec![Triangle {
            v0: Vector3::new(0.0, 0.0, 0.0),
            v1: Vector3::new(1.0, 0.0, 0.0),
            v2: Vector3::new(1.0, 1.0, 1.0),
        }];
        let builder = SahKdTreeBuilder {
            traverse_cost: 0.1,
            intersect_cost: 1.0,
            empty_factor: 0.8,
            triangles,
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
    fn test_axially_aligned_triangle() {
        let triangles = vec![Triangle {
            v0: Vector3::new(0.0, 0.0, 0.0),
            v1: Vector3::new(1.0, 0.0, 0.0),
            v2: Vector3::new(1.0, 1.0, 0.0),
        }];
        let builder = SahKdTreeBuilder {
            traverse_cost: 0.1,
            intersect_cost: 1.0,
            empty_factor: 0.8,
            triangles,
        };
        let tree = build_kdtree(builder, 4);

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
