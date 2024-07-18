use super::{
    build::{KdCell, KdSplit},
    split::split_and_partition,
    KdNode, KdTree,
};
use crate::split::{clip_geometries, SplitPartitioning};
use geometry::{aap::Aap, bound::geometries_bounding_box, geometry::Geometry};
use glam::Vec3;

pub struct SahKdTreeBuilder {
    pub traverse_cost: f32,
    pub intersect_cost: f32,
    pub empty_factor: f32,
    pub geometries: Vec<Geometry>,
}

pub const MAX_DEPTH: u32 = 20;
pub const TRAVERSE_COST: f32 = 2.0;
pub const INTERSECT_COST: f32 = 1.0;
pub const EMPTY_FACTOR: f32 = 0.8;

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

    fn select_best_split_based_on_cost(&self, split: SplitPartitioning) -> Option<(KdSplit, f32)> {
        // TODO: Place planes to the left or to the right depending on what gives best cost.
        if (split.left_aabb.volume() == 0.0 || split.right_aabb.volume() == 0.0)
            && split.middle_indices.is_empty()
        {
            return None;
        }
        let surface_area = split.parent_aabb.surface_area();
        let probability_left = split.left_aabb.surface_area() / surface_area;
        let probability_right = split.right_aabb.surface_area() / surface_area;
        let probability = (probability_left, probability_right);
        let counts = (
            split.left_indices.len() + split.middle_indices.len(),
            split.right_indices.len() + split.middle_indices.len(),
        );
        let cost = self.calculate_sah_cost(probability, counts);
        let mut left_indices = split.left_indices;
        let mut right_indices = split.right_indices;
        left_indices.extend(&split.middle_indices);
        right_indices.extend(split.middle_indices);
        let left = KdCell::new(split.left_aabb, left_indices);
        let right = KdCell::new(split.right_aabb, right_indices);
        let plane = split.plane;
        Some((KdSplit { plane, left, right }, cost))
    }

    pub(crate) fn starting_box(&self) -> KdCell {
        KdCell::new(
            geometries_bounding_box(&self.geometries).enlarge(Vec3::new(1.0, 1.0, 1.0)),
            (0u32..self.geometries.len() as u32).collect(),
        )
    }

    pub(crate) fn find_best_split(&self, _: u32, cell: &KdCell) -> Option<KdSplit> {
        debug_assert!(
            !cell.indices().is_empty(),
            "splitting a kd-cell with no geometries only worsens performance"
        );

        let min_by_snd = |a: (_, f32), b: (_, f32)| if a.1 <= b.1 { a } else { b };

        let clipped = clip_geometries(&self.geometries, cell);
        let mut splits = clipped
            .iter()
            .flat_map(|(_, aabb)| aabb.sides())
            .collect::<Vec<_>>();
        splits.sort_unstable_by(Aap::total_cmp);
        splits.dedup();
        splits
            .into_iter()
            .filter_map(|plane| {
                self.select_best_split_based_on_cost(split_and_partition(
                    &clipped,
                    *cell.boundary(),
                    plane,
                ))
            })
            .reduce(min_by_snd)
            .map(|a| a.0)
    }

    pub(crate) fn terminate(&self, cell: &KdCell, split: &KdSplit) -> bool {
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

    pub(crate) fn make_tree(self, root: Box<KdNode>) -> KdTree {
        KdTree {
            root,
            geometries: self.geometries,
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::triangle::Triangle;

    use crate::build::build_kdtree;

    use super::*;

    #[test]
    fn non_axially_aligned_triangle() {
        let triangle = Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 1.0),
        };
        let builder = SahKdTreeBuilder {
            traverse_cost: 0.1,
            intersect_cost: 1.0,
            empty_factor: 0.8,
            geometries: vec![triangle.into()],
        };
        let tree = build_kdtree(builder, 7);

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
        let triangle = Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 0.0),
        };
        let builder = SahKdTreeBuilder {
            traverse_cost: 1.0,
            intersect_cost: 10.0,
            empty_factor: 0.8,
            geometries: vec![triangle.into()],
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
