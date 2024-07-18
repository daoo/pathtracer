use geometry::{aabb::Aabb, aap::Aap};

use crate::build_sah::SahKdTreeBuilder;

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

fn build_helper(
    builder: &SahKdTreeBuilder,
    max_depth: u32,
    depth: u32,
    cell: KdCell,
) -> Box<KdNode> {
    if depth >= max_depth || cell.indices.is_empty() {
        return KdNode::new_leaf(cell.indices);
    }

    match builder.find_best_split(depth, &cell) {
        None => KdNode::new_leaf(cell.indices),
        Some(split) => {
            if builder.terminate(&cell, &split) {
                KdNode::new_leaf(cell.indices)
            } else {
                let left = build_helper(builder, max_depth, depth + 1, split.left);
                let right = build_helper(builder, max_depth, depth + 1, split.right);
                KdNode::new_node(split.plane, left, right)
            }
        }
    }
}

pub fn build_kdtree(builder: SahKdTreeBuilder, max_depth: u32) -> KdTree {
    if max_depth as usize > super::MAX_DEPTH {
        panic!(
            "Max depth ({}) must be smaller than hard coded value ({}).",
            max_depth,
            super::MAX_DEPTH
        );
    }
    let root = build_helper(&builder, max_depth, 1, builder.starting_box());
    builder.make_tree(root)
}
