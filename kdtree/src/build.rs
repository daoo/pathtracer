use geometry::shape::Shape;

use crate::{
    MAX_DEPTH,
    cell::KdCell,
    sah::{EventSide, SahCost, find_best_split},
};

use super::KdNode;

fn build_helper(
    geometries: &[Shape],
    sah: &SahCost,
    depth: u32,
    cell: KdCell,
    sides: &mut [EventSide],
) -> Box<KdNode> {
    let _ = sides;
    if depth as usize >= MAX_DEPTH || cell.indices.is_empty() {
        return KdNode::new_leaf(cell.indices);
    }

    match find_best_split(geometries, sah, &cell, sides) {
        None => KdNode::new_leaf(cell.indices),
        Some(split) => {
            let left = build_helper(geometries, sah, depth + 1, split.left, sides);
            let right = build_helper(geometries, sah, depth + 1, split.right, sides);
            KdNode::new_node(split.plane, left, right)
        }
    }
}

pub fn build_kdtree(geometries: &[Shape], sah: &SahCost) -> KdNode {
    *build_helper(
        geometries,
        sah,
        1,
        KdCell::generate_initial(geometries),
        &mut (0..geometries.len())
            .map(|_| EventSide::Both)
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use geometry::{aap::Aap, triangle::Triangle};
    use glam::Vec3;

    use crate::{KdNode, build::build_kdtree};

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
        let geometries = [triangle1, triangle2].map(Shape::from);
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
        let geometries = [triangle1, triangle2].map(Shape::from);
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
        let geometries = triangles.map(Shape::from);
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
