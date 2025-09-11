use std::{fmt::Display, ops::RangeInclusive};

use arrayvec::ArrayVec;
use geometry::{
    aap::Aap,
    geometry::{Geometry, IndexedIntersection, intersect_closest_geometry},
    ray::Ray,
};

pub mod build;
mod cell;
mod event;
pub mod format;
pub mod sah;

pub const MAX_DEPTH: usize = 25;

#[derive(Debug, PartialEq)]
pub enum KdNode {
    Leaf(Vec<u32>),
    Node {
        plane: Aap,
        left: Box<KdNode>,
        right: Box<KdNode>,
    },
}

impl KdNode {
    #[inline]
    pub const fn empty() -> Self {
        Self::Leaf(Vec::new())
    }

    #[inline]
    pub(crate) const fn new_leaf(indices: Vec<u32>) -> Self {
        Self::Leaf(indices)
    }

    #[inline]
    pub(crate) fn new_node(plane: Aap, left: Self, right: Self) -> Self {
        debug_assert!(
            !(left.is_empty() && right.is_empty()),
            "kd-node with both children empty worsens performance"
        );
        let left = Box::new(left);
        let right = Box::new(right);
        Self::Node { plane, left, right }
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Leaf(indices) => indices.is_empty(),
            Self::Node { .. } => false,
        }
    }

    #[inline]
    pub fn iter_nodes(&self) -> KdNodeIter<'_> {
        KdNodeIter::new(self)
    }

    #[inline]
    pub fn iter_leafs(&self) -> impl Iterator<Item = (usize, &Vec<u32>)> {
        self.iter_nodes().filter_map(|(depth, node)| match node {
            Self::Leaf(indices) => Some((depth, indices)),
            Self::Node { .. } => None,
        })
    }
}

impl KdNode {
    pub fn intersect<G>(
        &self,
        geometries: &[G],
        ray: &Ray,
        t_range: RangeInclusive<f32>,
    ) -> Option<IndexedIntersection<G::Intersection>>
    where
        G: Geometry,
    {
        let mut node = self;
        let mut t1 = *t_range.start();
        let mut t2 = *t_range.end();
        let mut stack: ArrayVec<(&Self, f32, f32), MAX_DEPTH> = ArrayVec::new();
        loop {
            match node {
                Self::Leaf(indices) => {
                    match intersect_closest_geometry(
                        geometries,
                        indices.iter().copied(),
                        ray,
                        t1..=t2,
                    ) {
                        Some(result) => return Some(result),
                        _ if t2 == *t_range.end() => return None,
                        _ => match stack.pop() {
                            Some(s) => {
                                (node, t1, t2) = s;
                            }
                            None => return None,
                        },
                    }
                }
                Self::Node { plane, left, right } => {
                    let axis = plane.axis;
                    if let Some(t) = plane.intersect_ray(ray) {
                        let (near, far) = if ray.direction[axis] >= 0. {
                            (left.as_ref(), right.as_ref())
                        } else {
                            (right.as_ref(), left.as_ref())
                        };
                        if t > t2 {
                            node = near;
                        } else if t < t1 {
                            node = far;
                        } else {
                            unsafe {
                                stack.push_unchecked((far, t, t2));
                            }
                            node = near;
                            t2 = t;
                        }
                    } else if ray.origin[axis] <= plane.distance {
                        node = left;
                    } else {
                        node = right;
                    }
                }
            }
        }
    }
}

impl Display for KdNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leaf(indices) => write!(f, "{indices:?}"),
            Self::Node { plane, left, right } => {
                write!(
                    f,
                    "node({:?}, {}, {}, {})",
                    plane.axis, plane.distance, left, right
                )
            }
        }
    }
}

pub struct KdNodeIter<'a> {
    pub(crate) stack: ArrayVec<(usize, &'a KdNode), MAX_DEPTH>,
}

impl<'a> KdNodeIter<'a> {
    #[inline]
    fn new(node: &'a KdNode) -> Self {
        let mut stack = ArrayVec::<(usize, &'a KdNode), MAX_DEPTH>::new();
        unsafe {
            stack.push_unchecked((1, node));
        }
        Self { stack }
    }
}

impl<'a> Iterator for KdNodeIter<'a> {
    type Item = (usize, &'a KdNode);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((depth, node)) = self.stack.pop() {
            match node {
                KdNode::Leaf(_) => Some((depth, node)),
                KdNode::Node {
                    plane: _,
                    left,
                    right,
                } => {
                    self.stack.push((depth + 1, left));
                    self.stack.push((depth + 1, right));
                    Some((depth, node))
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::{
        aap::Aap,
        any_triangle::AnyTriangle,
        axis::Axis,
        ray::Ray,
        triangle::{Triangle, TriangleIntersection},
    };
    use glam::Vec3;

    use super::*;

    #[test]
    fn intersect_empty_node() {
        let geometries: [Triangle; 0] = [];
        let node = KdNode::empty();
        let ray = Ray::between(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.));

        assert_eq!(node.intersect(&geometries, &ray, 0.0..=1.0), None);
    }

    #[test]
    fn intersect_ray_intersecting_split_plane_and_both_triangles() {
        let triangle0 = Triangle {
            v0: Vec3::new(0., 0., -1.),
            v1: Vec3::new(2., 0., -1.),
            v2: Vec3::new(2., 2., -1.),
        };
        let triangle1 = Triangle {
            v0: Vec3::new(0., 0., 1.),
            v1: Vec3::new(2., 0., 1.),
            v2: Vec3::new(2., 2., 1.),
        };
        let geometries = [triangle0, triangle1];
        let node = KdNode::new_node(
            Aap {
                axis: Axis::X,
                distance: 1.,
            },
            KdNode::new_leaf(vec![0, 1]),
            KdNode::new_leaf(vec![0, 1]),
        );
        let ray = Ray::between(Vec3::new(1., 1., -2.), Vec3::new(1., 1., 2.));

        assert_eq!(
            node.intersect(&geometries, &ray, 0.0..=1.0),
            Some(IndexedIntersection::new(
                0,
                TriangleIntersection::new(0.25, 0., 0.5)
            ))
        );
        assert_eq!(
            node.intersect(&geometries, &ray.reverse(), 0.0..=1.0),
            Some(IndexedIntersection::new(
                1,
                TriangleIntersection::new(0.25, 0., 0.5)
            ))
        );
    }

    #[test]
    fn intersect_ray_parallel_to_split_plane_and_intersecting_one_triangle() {
        let triangle0 = Triangle {
            v0: Vec3::new(0., 0., 0.),
            v1: Vec3::new(1., 0., 0.),
            v2: Vec3::new(0., 1., 0.),
        };
        let triangle1 = Triangle {
            v0: Vec3::new(1., 0., 0.),
            v1: Vec3::new(2., 0., 0.),
            v2: Vec3::new(2., 1., 0.),
        };
        let geometries = [triangle0, triangle1];
        let node = KdNode::new_node(
            Aap::new_x(1.0),
            KdNode::new_leaf(vec![0]),
            KdNode::new_leaf(vec![1]),
        );
        let ray_triangle0_v0 = Ray::between(Vec3::new(0., 0., -1.), Vec3::new(0., 0., 1.));
        let ray_triangle1_v1 = Ray::between(Vec3::new(2., 0., -1.), Vec3::new(2., 0., 1.));

        assert_eq!(
            node.intersect(&geometries, &ray_triangle0_v0, 0.0..=1.0),
            Some(IndexedIntersection::new(
                0,
                TriangleIntersection::new(0.5, 0., 0.)
            ))
        );
        assert_eq!(
            node.intersect(&geometries, &ray_triangle1_v1, 0.0..=1.0),
            Some(IndexedIntersection::new(
                1,
                TriangleIntersection::new(0.5, 1., 0.)
            ))
        );
    }

    #[test]
    fn intersect_ray_orthogonal_to_split_plane_and_intersecting_both_triangles() {
        let triangle0 = Triangle {
            v0: Vec3::new(0., -1., -1.),
            v1: Vec3::new(0., 1., -1.),
            v2: Vec3::new(0., 1., 1.),
        };
        let triangle1 = Triangle {
            v0: Vec3::new(2., -1., -1.),
            v1: Vec3::new(2., 1., -1.),
            v2: Vec3::new(2., 1., 1.),
        };
        let geometries = [triangle0, triangle1];
        let node = KdNode::new_node(
            Aap::new_x(1.0),
            KdNode::new_leaf(vec![0]),
            KdNode::new_leaf(vec![1]),
        );
        let ray = Ray::between(Vec3::new(-1., 0., 0.), Vec3::new(3., 0., 0.));

        assert_eq!(
            node.intersect(&geometries, &ray, 0.0..=1.0),
            Some(IndexedIntersection::new(
                0,
                TriangleIntersection::new(0.25, 0., 0.5)
            ))
        );
        assert_eq!(
            node.intersect(&geometries, &ray.reverse(), 0.0..=1.0),
            Some(IndexedIntersection::new(
                1,
                TriangleIntersection::new(0.25, 0., 0.5)
            ))
        );
    }

    #[test]
    fn intersect_split_at_axially_aligned_triangle() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 1.),
            v1: Vec3::new(1., 0., 1.),
            v2: Vec3::new(0., 1., 1.),
        };
        let geometries = [triangle];
        let tree_left =
            KdNode::new_node(Aap::new_z(1.0), KdNode::new_leaf(vec![0]), KdNode::empty());
        let tree_right =
            KdNode::new_node(Aap::new_z(1.0), KdNode::empty(), KdNode::new_leaf(vec![0]));
        let ray = Ray::between(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 2.));

        assert_eq!(
            tree_left.intersect(&geometries, &ray, 0.0..=1.0),
            Some(IndexedIntersection::new(
                0,
                TriangleIntersection::new(0.5, 0., 0.)
            ))
        );
        assert_eq!(
            tree_right.intersect(&geometries, &ray, 0.0..=1.0),
            Some(IndexedIntersection::new(
                0,
                TriangleIntersection::new(0.5, 0., 0.)
            ))
        );
    }

    #[test]
    fn intersect_flat_cell_left_left() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 1.),
            v1: Vec3::new(1., 0., 1.),
            v2: Vec3::new(0., 1., 1.),
        };
        let geometries = [triangle];
        let node = KdNode::new_node(
            Aap::new_z(1.0),
            KdNode::new_node(Aap::new_z(1.0), KdNode::new_leaf(vec![0]), KdNode::empty()),
            KdNode::empty(),
        );
        let ray = Ray::between(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 2.));

        assert_eq!(
            node.intersect(&geometries, &ray, 0.0..=1.0),
            Some(IndexedIntersection::new(
                0,
                TriangleIntersection::new(0.5, 0., 0.)
            ))
        );
        assert_eq!(
            node.intersect(&geometries, &ray.reverse(), 0.0..=1.0),
            Some(IndexedIntersection::new(
                0,
                TriangleIntersection::new(0.5, 0., 0.)
            ))
        );
    }

    #[test]
    fn intersect_flat_cell_right_left() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 1.),
            v1: Vec3::new(1., 0., 1.),
            v2: Vec3::new(0., 1., 1.),
        };
        let geometries = [triangle];
        let node = KdNode::new_node(
            Aap::new_z(1.0),
            KdNode::empty(),
            KdNode::new_node(Aap::new_z(1.0), KdNode::new_leaf(vec![0]), KdNode::empty()),
        );
        let ray = Ray::between(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 2.));

        assert_eq!(
            node.intersect(&geometries, &ray, 0.0..=1.0),
            Some(IndexedIntersection::new(
                0,
                TriangleIntersection::new(0.5, 0., 0.)
            ))
        );
        assert_eq!(
            node.intersect(&geometries, &ray.reverse(), 0.0..=1.0),
            Some(IndexedIntersection::new(
                0,
                TriangleIntersection::new(0.5, 0., 0.)
            ))
        );
    }

    #[test]
    fn intersect_flat_cell_minimized_example() {
        let triangle = Triangle {
            v0: Vec3::new(1.0, 1.0, -1.0),
            v1: Vec3::new(-1.0, 1.0, -1.0),
            v2: Vec3::new(1.0, -1.0, -1.0),
        };
        let geometries = [triangle];
        let node = KdNode::new_node(
            Aap::new_z(-1.0),
            KdNode::empty(),
            KdNode::new_node(Aap::new_z(-1.0), KdNode::new_leaf(vec![0]), KdNode::empty()),
        );
        let ray = Ray::new(
            Vec3::new(0.0, 0.0, 3.0),
            Vec3::new(0.06646079, 0.08247295, -0.9238795),
        );

        let actual = node.intersect(&geometries, &ray, 0.0..=f32::MAX);

        assert_eq!(
            actual,
            Some(IndexedIntersection::new(
                0,
                TriangleIntersection::new(4.329569, 0.35612673, 0.32146382)
            ))
        );
    }

    #[test]
    fn intersect_rounding_error_example() {
        let triangle = AnyTriangle::from(Triangle {
            v0: Vec3::new(-1.0, -1.0, 1.0),
            v1: Vec3::new(-1.0, -1.0, -1.0),
            v2: Vec3::new(-1.0, 1.0, 1.0),
        });
        let geometries = [triangle];
        let node = KdNode::new_node(Aap::new_x(-1.0), KdNode::empty(), KdNode::new_leaf(vec![0]));
        let ray = Ray::new(
            Vec3::new(-0.5170438, -0.4394186, -0.045965273),
            Vec3::new(-0.8491798, -0.1408107, -0.5089852),
        );

        let actual = node.intersect(&geometries, &ray, 0.0..=f32::MAX);

        assert_eq!(
            actual,
            Some(IndexedIntersection::new(
                0,
                TriangleIntersection::new(0.5687325, 0.66772085, 0.24024889)
            ))
        );
    }
}
