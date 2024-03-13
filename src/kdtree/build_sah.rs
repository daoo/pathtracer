use nalgebra::vector;

use crate::geometry::{
    aabb::Aabb,
    aap::{Aap, Axis},
    algorithms::triangles_bounding_box,
    triangle::Triangle,
};

use super::{
    build::{KdBox, KdSplit, KdTreeInputs},
    KdNode, KdTree,
};

const COST_EMPTY_FACTOR: f32 = 0.8;
const COST_TRAVERSE: f32 = 0.1;
const COST_INTERSECT: f32 = 1.0;

fn calculate_sah_cost_helper(probability: (f32, f32), counts: (usize, usize)) -> f32 {
    debug_assert!(probability.0 >= 0.0 && probability.1 >= 0.0);
    debug_assert!(probability.0 > 0.0 || probability.1 > 0.0);
    let empty_factor = if counts.0 == 0 || counts.1 == 0 {
        COST_EMPTY_FACTOR
    } else {
        1.0
    };
    let intersect_cost =
        COST_INTERSECT * (probability.0 * counts.0 as f32 + probability.1 * counts.1 as f32);
    empty_factor * (COST_TRAVERSE + intersect_cost)
}

fn calculate_sah_cost(parent: &Aabb, split: &KdSplit) -> f32 {
    let probability_left = split.left.boundary.surface_area() / parent.surface_area();
    let probability_right = split.right.boundary.surface_area() / parent.surface_area();
    let probability = (probability_left, probability_right);
    let counts = (
        split.left.triangle_indices.len(),
        split.right.triangle_indices.len(),
    );
    calculate_sah_cost_helper(probability, counts)
}

fn split_and_calculate_cost(inputs: &KdTreeInputs, parent: &KdBox, plane: Aap) -> (KdSplit, f32) {
    let split = inputs.split_box(parent, plane);
    let cost = calculate_sah_cost(&parent.boundary, &split);
    (split, cost)
}

fn find_best_split(inputs: &KdTreeInputs, parent: &KdBox) -> Option<KdSplit> {
    debug_assert!(parent.boundary.volume() > 0.0);
    debug_assert!(!parent.triangle_indices.is_empty());

    const AXES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

    let min_by_snd = |a: (_, f32), b: (_, f32)| if a.1 <= b.1 { a } else { b };

    AXES.iter()
        .flat_map(|axis| {
            let mut points = inputs.potential_split_points(parent, *axis);
            points.dedup();
            points
                .iter()
                .map(|distance| {
                    split_and_calculate_cost(
                        inputs,
                        parent,
                        Aap {
                            axis: *axis,
                            distance: *distance,
                        },
                    )
                })
                .reduce(min_by_snd)
        })
        .reduce(min_by_snd)
        .map(|a| a.0)
}

fn build(inputs: &KdTreeInputs, depth: u32, parent: KdBox) -> Box<KdNode> {
    if depth >= inputs.max_depth || parent.triangle_indices.is_empty() {
        return Box::new(KdNode::Leaf(parent.triangle_indices));
    }

    match find_best_split(inputs, &parent) {
        None => Box::new(KdNode::Leaf(parent.triangle_indices)),
        Some(split) => {
            let left = build(inputs, depth + 1, split.left);
            let right = build(inputs, depth + 1, split.right);
            Box::new(KdNode::Node {
                plane: split.plane,
                left,
                right,
            })
        }
    }
}

pub fn build_kdtree_sah(max_depth: u32, triangles: Vec<Triangle>) -> KdTree {
    let kdbox: KdBox = KdBox {
        boundary: triangles_bounding_box(&triangles).enlarge(&vector![0.5, 0.5, 0.5]),
        triangle_indices: (0u32..triangles.len() as u32).collect(),
    };
    let inputs = KdTreeInputs {
        max_depth,
        triangles,
    };
    KdTree {
        root: build(&inputs, 0, kdbox),
        triangles: inputs.triangles,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let triangles = vec![Triangle {
            v0: vector![0.0, 0.0, 0.0],
            v1: vector![1.0, 0.0, 0.0],
            v2: vector![1.0, 1.0, 0.0],
        }];
        let tree = build_kdtree_sah(6, triangles);

        let expected = KdNode::new_node(
            Axis::Z,
            -0.1,
            KdNode::new_leaf(vec![]),
            KdNode::new_node(
                Axis::Z,
                0.1,
                KdNode::new_node(
                    Axis::Y,
                    -0.1,
                    KdNode::new_leaf(vec![]),
                    KdNode::new_node(
                        Axis::Y,
                        1.1,
                        KdNode::new_node(
                            Axis::X,
                            -0.1,
                            KdNode::new_leaf(vec![]),
                            KdNode::new_node(
                                Axis::X,
                                1.1,
                                KdNode::new_leaf(vec![0]),
                                KdNode::new_leaf(vec![]),
                            ),
                        ),
                        KdNode::new_leaf(vec![]),
                    ),
                ),
                KdNode::new_leaf(vec![]),
            ),
        );
        dbg!(&tree.root);
        assert_eq!(tree.root, expected);
    }
}
