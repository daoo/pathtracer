use crate::geometry::{
    aabb::Aabb,
    aap::{Aap, Axis},
    algorithms::intersect_triangle_aabb,
    triangle::Triangle,
};

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

pub fn potential_split_points(triangles: &[Triangle], parent: &KdBox, axis: Axis) -> Vec<f32> {
    let min = parent.boundary.min()[axis] + 0.1;
    let max = parent.boundary.max()[axis] - 0.1;
    let mut points = parent
        .triangle_indices
        .iter()
        .flat_map(|i| {
            let triangle = &triangles[*i as usize];
            [triangle.min()[axis] - 0.1, triangle.max()[axis] + 0.1]
        })
        .filter(|p| p > &min && p < &max)
        .collect::<Vec<_>>();
    points.sort_unstable_by(f32::total_cmp);
    points
}

pub fn split_box(triangles: &Vec<Triangle>, parent: &KdBox, plane: Aap) -> KdSplit {
    let (left_aabb, right_aabb) = parent.boundary.split(&plane);
    debug_assert!(
        left_aabb.size()[plane.axis] > 0.1,
        "left_aabb too small {:?} {:?}",
        plane,
        left_aabb
    );
    debug_assert!(
        right_aabb.size()[plane.axis] > 0.1,
        "right_aabb to small {:?} {:?}",
        plane,
        right_aabb
    );
    let mut left_triangle_indices: Vec<u32> = Vec::new();
    let mut right_triangle_indices: Vec<u32> = Vec::new();
    for triangle_index in &parent.triangle_indices {
        let triangle = &triangles[*triangle_index as usize];
        let in_left = intersect_triangle_aabb(triangle, &left_aabb);
        let in_right = intersect_triangle_aabb(triangle, &right_aabb);
        debug_assert!(in_left || in_right);
        if in_left {
            left_triangle_indices.push(*triangle_index);
        }
        if in_right {
            right_triangle_indices.push(*triangle_index);
        }
    }
    KdSplit {
        plane,
        left: KdBox {
            boundary: left_aabb,
            triangle_indices: left_triangle_indices,
        },
        right: KdBox {
            boundary: right_aabb,
            triangle_indices: right_triangle_indices,
        },
    }
}
