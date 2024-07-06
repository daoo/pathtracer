use std::{fmt::Display, ops::RangeInclusive};

use geometry::{aap::Aap, geometric::Geometric, intersection::RayIntersection, ray::Ray, Geometry};
use smallvec::SmallVec;

pub mod build;
pub mod build_sah;
pub mod format;
mod split;

fn intersect_closest(
    geometries: &[Geometric],
    indices: &[u32],
    ray: &Ray,
    t_range: RangeInclusive<f32>,
) -> Option<(u32, RayIntersection)> {
    indices
        .iter()
        .filter_map(|index| {
            let index = *index;
            geometries[index as usize]
                .intersect_ray(ray)
                .and_then(|intersection| {
                    t_range
                        .contains(&intersection.t)
                        .then_some((index, intersection))
                })
        })
        .min_by(|a, b| f32::total_cmp(&a.1.t, &b.1.t))
}

#[derive(Clone, Debug, PartialEq)]
pub enum KdNode {
    Leaf(Vec<u32>),
    Node {
        plane: Aap,
        left: Box<KdNode>,
        right: Box<KdNode>,
    },
}

impl KdNode {
    pub fn empty() -> Box<Self> {
        Box::new(Self::Leaf(vec![]))
    }

    pub fn new_leaf(indices: Vec<u32>) -> Box<Self> {
        Box::new(Self::Leaf(indices))
    }

    pub fn new_node(plane: Aap, left: Box<Self>, right: Box<Self>) -> Box<Self> {
        debug_assert!(
            !(left.is_empty() && right.is_empty()),
            "kd-node with both children empty worsens performance"
        );
        Box::new(Self::Node { plane, left, right })
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Leaf(indices) => indices.is_empty(),
            Self::Node { .. } => false,
        }
    }

    pub fn count_nodes(&self) -> usize {
        match self {
            KdNode::Leaf(_) => 1,
            KdNode::Node {
                plane: _,
                left,
                right,
            } => 1 + left.count_nodes() + right.count_nodes(),
        }
    }

    pub fn count_geometries(&self) -> usize {
        match self {
            KdNode::Leaf(indices) => indices.len(),
            KdNode::Node {
                plane: _,
                left,
                right,
            } => left.count_geometries() + right.count_geometries(),
        }
    }

    pub fn all_indices(&self) -> Vec<u32> {
        match self {
            KdNode::Leaf(indicies) => indicies.clone(),
            KdNode::Node {
                plane: _,
                left,
                right,
            } => [left.all_indices(), right.all_indices()].concat(),
        }
    }

    pub fn intersect(
        &self,
        geometries: &[Geometric],
        ray: &Ray,
        t_range: RangeInclusive<f32>,
    ) -> Option<(u32, RayIntersection)> {
        let mut node = self;
        let mut t1 = *t_range.start();
        let mut t2 = *t_range.end();
        let mut stack: SmallVec<[(&KdNode, f32, f32); 5]> = SmallVec::new();
        loop {
            match node {
                KdNode::Leaf(indices) => {
                    match intersect_closest(geometries, indices, ray, t1..=t2) {
                        Some(result) => return Some(result),
                        _ if t2 == *t_range.end() => return None,
                        _ => {
                            if let Some(s) = stack.pop() {
                                (node, t1, t2) = s;
                            } else {
                                return None;
                            }
                        }
                    }
                }
                KdNode::Node { plane, left, right } => {
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
                            stack.push((far, t, t2));
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
            KdNode::Leaf(indices) => write!(f, "{indices:?}"),
            KdNode::Node { plane, left, right } => {
                write!(
                    f,
                    "node({:?}, {}, {}, {})",
                    plane.axis, plane.distance, left, right
                )
            }
        }
    }
}

pub struct KdTreeNodeIter<'a> {
    stack: Vec<&'a KdNode>,
}

impl<'a> Iterator for KdTreeNodeIter<'a> {
    type Item = &'a KdNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            match node {
                KdNode::Leaf(_) => Some(node),
                KdNode::Node {
                    plane: _,
                    left,
                    right,
                } => {
                    self.stack.push(left);
                    self.stack.push(right);
                    Some(node)
                }
            }
        } else {
            None
        }
    }
}
pub struct KdTree {
    pub root: Box<KdNode>,
    pub geometries: Vec<Geometric>,
}

impl KdTree {
    pub fn iter_nodes(&self) -> KdTreeNodeIter<'_> {
        KdTreeNodeIter {
            stack: vec![&self.root],
        }
    }

    pub fn intersect(
        &self,
        ray: &Ray,
        t_range: RangeInclusive<f32>,
    ) -> Option<(u32, RayIntersection)> {
        self.root.intersect(&self.geometries, ray, t_range)
    }
}

#[cfg(test)]
mod tests {
    use geometry::{axis::Axis, triangle::Triangle};
    use glam::Vec3;

    use super::*;

    #[test]
    fn intersect_empty_tree() {
        let tree = KdTree {
            root: Box::new(KdNode::Leaf(vec![])),
            geometries: vec![],
        };
        let ray = Ray::between(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.));

        assert_eq!(tree.intersect(&ray, 0.0..=1.0), None);
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
        let tree = KdTree {
            root: Box::new(KdNode::Node {
                plane: Aap {
                    axis: Axis::X,
                    distance: 1.,
                },
                left: Box::new(KdNode::Leaf(vec![0, 1])),
                right: Box::new(KdNode::Leaf(vec![0, 1])),
            }),
            geometries: vec![triangle0.into(), triangle1.into()],
        };
        let ray = Ray::between(Vec3::new(1., 1., -2.), Vec3::new(1., 1., 2.));

        assert_eq!(
            tree.intersect(&ray, 0.0..=1.0),
            Some((0, RayIntersection::new(0.25, 0., 0.5)))
        );
        assert_eq!(
            tree.intersect(&ray.reverse(), 0.0..=1.0),
            Some((1, RayIntersection::new(0.25, 0., 0.5)))
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
        let tree = KdTree {
            root: KdNode::new_node(
                Aap::new_x(1.0),
                KdNode::new_leaf(vec![0]),
                KdNode::new_leaf(vec![1]),
            ),
            geometries: vec![triangle0.into(), triangle1.into()],
        };
        let ray_triangle0_v0 = Ray::between(Vec3::new(0., 0., -1.), Vec3::new(0., 0., 1.));
        let ray_triangle1_v1 = Ray::between(Vec3::new(2., 0., -1.), Vec3::new(2., 0., 1.));

        assert_eq!(
            tree.intersect(&ray_triangle0_v0, 0.0..=1.0),
            Some((0, RayIntersection::new(0.5, 0., 0.)))
        );
        assert_eq!(
            tree.intersect(&ray_triangle1_v1, 0.0..=1.0),
            Some((1, RayIntersection::new(0.5, 1., 0.)))
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
        let tree = KdTree {
            root: KdNode::new_node(
                Aap::new_x(1.0),
                KdNode::new_leaf(vec![0]),
                KdNode::new_leaf(vec![1]),
            ),
            geometries: vec![triangle0.into(), triangle1.into()],
        };
        let ray = Ray::between(Vec3::new(-1., 0., 0.), Vec3::new(3., 0., 0.));

        assert_eq!(
            tree.intersect(&ray, 0.0..=1.0),
            Some((0, RayIntersection::new(0.25, 0., 0.5)))
        );
        assert_eq!(
            tree.intersect(&ray.reverse(), 0.0..=1.0),
            Some((1, RayIntersection::new(0.25, 0., 0.5)))
        );
    }

    #[test]
    fn intersect_split_at_axially_aligned_triangle() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 1.),
            v1: Vec3::new(1., 0., 1.),
            v2: Vec3::new(0., 1., 1.),
        };
        let tree_left = KdTree {
            root: KdNode::new_node(Aap::new_z(1.0), KdNode::new_leaf(vec![0]), KdNode::empty()),
            geometries: vec![triangle.into()],
        };
        let tree_right = KdTree {
            root: KdNode::new_node(Aap::new_z(1.0), KdNode::empty(), KdNode::new_leaf(vec![0])),
            geometries: vec![triangle.into()],
        };
        let ray = Ray::between(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 2.));

        assert_eq!(
            tree_left.intersect(&ray, 0.0..=1.0),
            Some((0, RayIntersection::new(0.5, 0., 0.)))
        );
        assert_eq!(
            tree_right.intersect(&ray, 0.0..=1.0),
            Some((0, RayIntersection::new(0.5, 0., 0.)))
        );
    }

    #[test]
    fn intersect_flat_cell_left_left() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 1.),
            v1: Vec3::new(1., 0., 1.),
            v2: Vec3::new(0., 1., 1.),
        };
        let tree = KdTree {
            root: KdNode::new_node(
                Aap::new_z(1.0),
                KdNode::new_node(Aap::new_z(1.0), KdNode::new_leaf(vec![0]), KdNode::empty()),
                KdNode::empty(),
            ),
            geometries: vec![triangle.into()],
        };
        let ray = Ray::between(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 2.));

        assert_eq!(
            tree.intersect(&ray, 0.0..=1.0),
            Some((0, RayIntersection::new(0.5, 0., 0.)))
        );
        assert_eq!(
            tree.intersect(&ray.reverse(), 0.0..=1.0),
            Some((0, RayIntersection::new(0.5, 0., 0.)))
        );
    }

    #[test]
    fn intersect_flat_cell_right_left() {
        let triangle = Triangle {
            v0: Vec3::new(0., 0., 1.),
            v1: Vec3::new(1., 0., 1.),
            v2: Vec3::new(0., 1., 1.),
        };
        let tree = KdTree {
            root: KdNode::new_node(
                Aap::new_z(1.0),
                KdNode::empty(),
                KdNode::new_node(Aap::new_z(1.0), KdNode::new_leaf(vec![0]), KdNode::empty()),
            ),
            geometries: vec![triangle.into()],
        };
        let ray = Ray::between(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 2.));

        assert_eq!(
            tree.intersect(&ray, 0.0..=1.0),
            Some((0, RayIntersection::new(0.5, 0., 0.)))
        );
        assert_eq!(
            tree.intersect(&ray.reverse(), 0.0..=1.0),
            Some((0, RayIntersection::new(0.5, 0., 0.)))
        );
    }

    #[test]
    fn intersect_flat_cell_minimized_example() {
        let triangle = Triangle {
            v0: Vec3::new(1.0, 1.0, -1.0),
            v1: Vec3::new(-1.0, 1.0, -1.0),
            v2: Vec3::new(1.0, -1.0, -1.0),
        };
        let root = KdNode::new_node(
            Aap::new_z(-1.0),
            KdNode::empty(),
            KdNode::new_node(Aap::new_z(-1.0), KdNode::new_leaf(vec![0]), KdNode::empty()),
        );
        let tree = KdTree {
            geometries: vec![triangle.into()],
            root,
        };
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 3.0),
            direction: Vec3::new(0.06646079, 0.08247295, -0.9238795),
        };

        let actual = tree.intersect(&ray, 0.0..=f32::MAX);

        assert_eq!(
            actual,
            Some((
                0,
                RayIntersection {
                    t: 4.329569,
                    u: 0.35612673,
                    v: 0.32146382
                }
            ))
        );
    }

    #[test]
    fn intersect_rounding_error_example() {
        let triangle = Triangle {
            v0: Vec3::new(-1.0, -1.0, 1.0),
            v1: Vec3::new(-1.0, -1.0, -1.0),
            v2: Vec3::new(-1.0, 1.0, 1.0),
        };
        let tree = KdTree {
            root: KdNode::new_node(Aap::new_x(-1.0), KdNode::empty(), KdNode::new_leaf(vec![0])),
            geometries: vec![triangle.into()],
        };
        let ray = Ray {
            origin: Vec3::new(-0.5170438, -0.4394186, -0.045965273),
            direction: Vec3::new(-0.8491798, -0.1408107, -0.5089852),
        };

        let actual = tree.intersect(&ray, 0.0..=f32::MAX);

        assert_eq!(
            actual,
            Some((
                0,
                RayIntersection {
                    t: 0.5687325,
                    u: 0.66772085,
                    v: 0.24024889
                }
            ))
        );
    }
}
