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

fn build_helper(geometries: &[Geometry], sah: &SahCost, depth: u32, cell: KdCell) -> Box<KdNode> {
    if depth as usize >= MAX_DEPTH || cell.indices.is_empty() {
        return KdNode::new_leaf(cell.indices);
    }

    match find_best_split(geometries, sah, &cell) {
        None => KdNode::new_leaf(cell.indices),
        Some(split) => {
            if should_terminate(sah, &cell, &split) {
                KdNode::new_leaf(cell.indices)
            } else {
                let left = build_helper(geometries, sah, depth + 1, split.left);
                let right = build_helper(geometries, sah, depth + 1, split.right);
                KdNode::new_node(split.plane, left, right)
            }
        }
    }
}

pub fn build_kdtree(geometries: &[Geometry], sah: &SahCost) -> KdNode {
    *build_helper(geometries, sah, 1, starting_box(geometries))
}

#[cfg(test)]
mod tests {
    use geometry::{aap::Aap, triangle::Triangle};
    use glam::Vec3;

    use crate::{build::build_kdtree, KdNode};

    use super::*;

    #[test]
    fn two_oriented_triangles() {
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
        let sah = SahCost {
            traverse_cost: 0.1,
            intersect_cost: 1.0,
            empty_factor: 0.8,
        };
        let actual = build_kdtree(&geometries, &sah);

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
    fn two_axially_aligned_triangles() {
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
        let geometries = [triangle1, triangle2].map(Geometry::from);
        let sah = SahCost {
            traverse_cost: 0.0,
            intersect_cost: 1.0,
            empty_factor: 1.0,
        };
        let actual = build_kdtree(&geometries, &sah);

        let expected = KdNode::new_node(
            Aap::new_z(0.0),
            KdNode::new_leaf(vec![0]),
            KdNode::new_node(Aap::new_z(1.0), KdNode::empty(), KdNode::new_leaf(vec![1])),
        );
        assert_eq!(
            actual, *expected,
            "\n   actual: {}\n expected: {}",
            actual, *expected
        );
    }

    #[test]
    fn one_cube() {
        let triangles = [
            // Front
            Triangle {
                v0: Vec3::new(0.0, 0.0, 0.0),
                v1: Vec3::new(1.0, 0.0, 0.0),
                v2: Vec3::new(1.0, 1.0, 0.0),
            },
            Triangle {
                v0: Vec3::new(0.0, 0.0, 0.0),
                v1: Vec3::new(0.0, 1.0, 0.0),
                v2: Vec3::new(1.0, 1.0, 0.0),
            },
            // Back
            Triangle {
                v0: Vec3::new(0.0, 0.0, 1.0),
                v1: Vec3::new(1.0, 0.0, 1.0),
                v2: Vec3::new(1.0, 1.0, 1.0),
            },
            Triangle {
                v0: Vec3::new(0.0, 0.0, 1.0),
                v1: Vec3::new(0.0, 1.0, 1.0),
                v2: Vec3::new(1.0, 1.0, 1.0),
            },
            // Bottom
            Triangle {
                v0: Vec3::new(0.0, 0.0, 0.0),
                v1: Vec3::new(1.0, 0.0, 0.0),
                v2: Vec3::new(1.0, 0.0, 1.0),
            },
            Triangle {
                v0: Vec3::new(0.0, 0.0, 0.0),
                v1: Vec3::new(0.0, 0.0, 1.0),
                v2: Vec3::new(1.0, 0.0, 1.0),
            },
            // Top
            Triangle {
                v0: Vec3::new(0.0, 1.0, 0.0),
                v1: Vec3::new(1.0, 1.0, 0.0),
                v2: Vec3::new(1.0, 1.0, 1.0),
            },
            Triangle {
                v0: Vec3::new(0.0, 1.0, 0.0),
                v1: Vec3::new(0.0, 1.0, 1.0),
                v2: Vec3::new(1.0, 1.0, 1.0),
            },
            // Right
            Triangle {
                v0: Vec3::new(0.0, 0.0, 0.0),
                v1: Vec3::new(0.0, 0.0, 1.0),
                v2: Vec3::new(0.0, 1.0, 1.0),
            },
            Triangle {
                v0: Vec3::new(0.0, 0.0, 0.0),
                v1: Vec3::new(0.0, 1.0, 0.0),
                v2: Vec3::new(0.0, 1.0, 1.0),
            },
            // Left
            Triangle {
                v0: Vec3::new(1.0, 0.0, 0.0),
                v1: Vec3::new(1.0, 1.0, 0.0),
                v2: Vec3::new(1.0, 1.0, 1.0),
            },
            Triangle {
                v0: Vec3::new(1.0, 0.0, 0.0),
                v1: Vec3::new(1.0, 0.0, 1.0),
                v2: Vec3::new(1.0, 1.0, 1.0),
            },
        ];
        let geometries = triangles.map(Geometry::from);
        let sah = SahCost {
            traverse_cost: 0.0,
            intersect_cost: 1.0,
            empty_factor: 1.0,
        };
        let actual = build_kdtree(&geometries, &sah);

        let expected = KdNode::new_node(
            Aap::new_x(0.0),
            KdNode::new_leaf(vec![8, 9]),
            KdNode::new_node(
                Aap::new_x(1.0),
                KdNode::new_node(
                    Aap::new_y(0.0),
                    KdNode::new_leaf(vec![4, 5]),
                    KdNode::new_node(
                        Aap::new_y(1.0),
                        KdNode::new_node(
                            Aap::new_z(0.0),
                            KdNode::new_leaf(vec![0, 1]),
                            KdNode::new_node(
                                Aap::new_z(1.0),
                                KdNode::empty(),
                                KdNode::new_leaf(vec![2, 3]),
                            ),
                        ),
                        KdNode::new_leaf(vec![6, 7]),
                    ),
                ),
                KdNode::new_leaf(vec![10, 11]),
            ),
        );
        assert_eq!(
            actual, *expected,
            "\n   actual: {}\n expected: {}",
            actual, *expected
        );
    }
}
