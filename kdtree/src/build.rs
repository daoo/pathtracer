use geometry::{bound::geometries_bounding_box, geometry::Geometry};

use crate::{
    cell::KdCell,
    sah::{find_best_split, should_terminate, SahCost},
    MAX_DEPTH,
};

use super::KdNode;

fn starting_box(geometries: &[Geometry]) -> KdCell {
    KdCell::new(
        geometries_bounding_box(geometries),
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

pub fn build_kdtree(geometries: &[Geometry], max_depth: u32, cost: &SahCost) -> KdNode {
    if max_depth as usize > MAX_DEPTH {
        panic!(
            "Max depth ({}) must be smaller than hard coded value ({}).",
            max_depth, MAX_DEPTH
        );
    }
    *build_helper(geometries, cost, max_depth, 1, starting_box(geometries))
}

#[cfg(test)]
mod tests {
    use geometry::{aap::Aap, triangle::Triangle};
    use glam::Vec3;

    use crate::{build::build_kdtree, KdNode};

    use super::*;

    #[test]
    fn non_axially_aligned_triangle() {
        let triangle1 = Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 1.0),
        };
        let triangle2 = Triangle {
            v0: Vec3::new(1.0, 0.0, 0.0),
            v1: Vec3::new(2.0, 0.0, 0.0),
            v2: Vec3::new(2.0, 1.0, 1.0),
        };
        let geometries = [triangle1, triangle2].map(Geometry::from);
        let cost = SahCost {
            traverse_cost: 0.1,
            intersect_cost: 1.0,
            empty_factor: 0.8,
        };
        let actual = build_kdtree(&geometries, 7, &cost);

        let expected = KdNode::new_node(
            Aap::new_x(1.0),
            KdNode::new_leaf(vec![0]),
            KdNode::new_leaf(vec![1]),
        );
        assert_eq!(
            actual, *expected,
            "\n   actual: {}\n expected: {}",
            actual, *expected
        );
    }

    #[test]
    fn axially_aligned_triangle() {
        let triangle1 = Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 0.0),
        };
        let triangle2 = Triangle {
            v0: Vec3::new(0.0, 0.0, 1.0),
            v1: Vec3::new(1.0, 0.0, 1.0),
            v2: Vec3::new(1.0, 1.0, 1.0),
        };
        let triangle3 = Triangle {
            v0: Vec3::new(0.0, 0.0, 2.0),
            v1: Vec3::new(1.0, 0.0, 2.0),
            v2: Vec3::new(1.0, 1.0, 2.0),
        };
        let geometries = [triangle1, triangle2, triangle3].map(Geometry::from);
        let cost = SahCost {
            traverse_cost: 0.0,
            intersect_cost: 1.0,
            empty_factor: 1.0,
        };
        let actual = build_kdtree(&geometries, 6, &cost);

        let expected = KdNode::new_node(
            Aap::new_z(1.0),
            KdNode::new_leaf(vec![0, 1]),
            KdNode::new_leaf(vec![2, 1]),
        );
        assert_eq!(
            actual, *expected,
            "\n   actual: {}\n expected: {}",
            actual, *expected
        );
    }
}
