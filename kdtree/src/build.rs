use geometry::{aabb::Aabb, aap::Aap};

use super::{KdNode, KdTree};

#[derive(Debug)]
pub struct KdBox {
    pub boundary: Aabb,
    pub triangle_indices: Vec<u32>,
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
        return Box::new(KdNode::Leaf(parent.triangle_indices));
    }

    match builder.find_best_split(depth, &parent) {
        None => Box::new(KdNode::Leaf(parent.triangle_indices)),
        Some(split) if builder.terminate(&parent, &split) => {
            Box::new(KdNode::Leaf(parent.triangle_indices))
        }
        Some(split) => {
            let left = build_helper(builder, max_depth, depth + 1, split.left);
            let right = build_helper(builder, max_depth, depth + 1, split.right);
            Box::new(KdNode::Node {
                plane: split.plane,
                left,
                right,
            })
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
