use geometry::{aabb::Aabb, aap::Aap};

use super::{KdNode, KdTree};

#[derive(Debug)]
pub struct KdBox {
    boundary: Aabb,
    triangle_indices: Vec<u32>,
}

impl KdBox {
    pub fn new(boundary: Aabb, triangle_indices: Vec<u32>) -> Self {
        debug_assert!(!(boundary.volume() == 0.0 && triangle_indices.is_empty()));
        KdBox {
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
    pub left: KdBox,
    pub right: KdBox,
}

pub trait KdTreeBuilder {
    fn starting_box(&self) -> KdBox;

    fn find_best_split(&self, depth: u32, parent: &KdBox) -> Option<KdSplit>;

    fn terminate(&self, parent: &KdBox, split: &KdSplit) -> bool;

    fn make_tree(self, root: Box<KdNode>) -> KdTree;
}

fn build_helper<B>(builder: &B, max_depth: u32, depth: u32, parent: KdBox) -> Box<KdNode>
where
    B: KdTreeBuilder,
{
    if depth >= max_depth || parent.triangle_indices.is_empty() {
        return KdNode::new_leaf(parent.triangle_indices);
    }

    match builder.find_best_split(depth, &parent) {
        None => KdNode::new_leaf(parent.triangle_indices),
        Some(split) if builder.terminate(&parent, &split) => {
            KdNode::new_leaf(parent.triangle_indices)
        }
        Some(split) => {
            let left = build_helper(builder, max_depth, depth + 1, split.left);
            let right = build_helper(builder, max_depth, depth + 1, split.right);
            KdNode::new_node(split.plane, left, right)
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
