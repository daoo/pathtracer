use geometry::{aabb::Aabb, aap::Aap, bound::geometries_bounding_box, geometry::Geometry};
use glam::Vec3;

use crate::sah::{find_best_split, should_terminate, SahCost};

use super::{KdNode, KdTree};

#[derive(Debug)]
pub struct KdCell {
    boundary: Aabb,
    indices: Vec<u32>,
}

impl KdCell {
    pub fn new(boundary: Aabb, indices: Vec<u32>) -> Self {
        debug_assert!(
            boundary.surface_area() != 0.0,
            "empty kd-cell cannot intersect a ray"
        );
        debug_assert!(
            !(boundary.volume() == 0.0 && indices.is_empty()),
            "flat kd-cell without any triangles likely worsens performance"
        );
        KdCell { boundary, indices }
    }

    pub fn boundary(&self) -> &Aabb {
        &self.boundary
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}

#[derive(Debug)]
pub struct KdSplit {
    pub plane: Aap,
    pub left: KdCell,
    pub right: KdCell,
}

fn starting_box(geometries: &[Geometry]) -> KdCell {
    KdCell::new(
        geometries_bounding_box(geometries).enlarge(Vec3::new(1.0, 1.0, 1.0)),
        (0u32..geometries.len() as u32).collect(),
    )
}

fn build_helper(
    geometries: &[Geometry],
    cost: &SahCost,
    max_depth: u32,
    depth: u32,
    cell: KdCell,
) -> Box<KdNode> {
    if depth >= max_depth || cell.indices.is_empty() {
        return KdNode::new_leaf(cell.indices);
    }

    match find_best_split(geometries, cost, &cell) {
        None => KdNode::new_leaf(cell.indices),
        Some(split) => {
            if should_terminate(cost, &cell, &split) {
                KdNode::new_leaf(cell.indices)
            } else {
                let left = build_helper(geometries, cost, max_depth, depth + 1, split.left);
                let right = build_helper(geometries, cost, max_depth, depth + 1, split.right);
                KdNode::new_node(split.plane, left, right)
            }
        }
    }
}

pub fn build_kdtree(geometries: Vec<Geometry>, max_depth: u32, cost: &SahCost) -> KdTree {
    if max_depth as usize > super::MAX_DEPTH {
        panic!(
            "Max depth ({}) must be smaller than hard coded value ({}).",
            max_depth,
            super::MAX_DEPTH
        );
    }
    let root = build_helper(&geometries, cost, max_depth, 1, starting_box(&geometries));
    KdTree { root, geometries }
}

#[cfg(test)]
mod tests {
    use geometry::triangle::Triangle;
    use glam::Vec3;

    use crate::{build::build_kdtree, KdNode};

    use super::*;

    #[test]
    fn non_axially_aligned_triangle() {
        let triangle = Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 1.0),
        };
        let cost = SahCost {
            traverse_cost: 0.1,
            intersect_cost: 1.0,
            empty_factor: 0.8,
        };
        let tree = build_kdtree(vec![triangle.into()], 7, &cost);

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
        let cost = SahCost {
            traverse_cost: 1.0,
            intersect_cost: 10.0,
            empty_factor: 0.8,
        };
        let tree = build_kdtree(vec![triangle.into()], 6, &cost);

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
