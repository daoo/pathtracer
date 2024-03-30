use geometry::{aabb::Aabb, aap::Aap};

use super::{KdNode, KdTree};

#[derive(Debug)]
pub struct KdCell {
    boundary: Aabb,
    triangle_indices: Vec<u32>,
}

impl KdCell {
    pub fn new(boundary: Aabb, triangle_indices: Vec<u32>) -> Self {
        debug_assert!(
            boundary.surface_area() != 0.0,
            "empty kd-cell cannot intersect a ray"
        );
        debug_assert!(
            !(boundary.volume() == 0.0 && triangle_indices.is_empty()),
            "flat kd-cell without any triangles likely worsens performance"
        );
        KdCell {
            boundary,
            triangle_indices,
        }
    }

    pub fn boundary(&self) -> &Aabb {
        &self.boundary
    }

    pub fn triangle_indices(&self) -> &[u32] {
        &self.triangle_indices
    }
}

#[derive(Debug)]
pub struct KdSplit {
    pub plane: Aap,
    pub left: KdCell,
    pub right: KdCell,
}

pub trait KdTreeBuilder {
    fn starting_box(&self) -> KdCell;

    fn find_best_split(&self, depth: u32, cell: &KdCell) -> Option<KdSplit>;

    fn terminate(&self, cell: &KdCell, split: &KdSplit) -> bool;

    fn make_tree(self, root: Box<KdNode>) -> KdTree;
}

fn build_helper<B>(builder: &B, max_depth: u32, depth: u32, cell: KdCell) -> Box<KdNode>
where
    B: KdTreeBuilder,
{
    if depth >= max_depth || cell.triangle_indices.is_empty() {
        return KdNode::new_leaf(cell.triangle_indices);
    }

    match builder.find_best_split(depth, &cell) {
        None => KdNode::new_leaf(cell.triangle_indices),
        Some(split) => {
            if builder.terminate(&cell, &split) {
                KdNode::new_leaf(cell.triangle_indices)
            } else {
                let left = build_helper(builder, max_depth, depth + 1, split.left);
                let right = build_helper(builder, max_depth, depth + 1, split.right);
                KdNode::new_node(split.plane, left, right)
            }
        }
    }
}

pub fn build_kdtree<B>(builder: B, max_depth: u32) -> KdTree
where
    B: KdTreeBuilder,
{
    let root = build_helper(&builder, max_depth, 0, builder.starting_box());
    builder.make_tree(root)
}
