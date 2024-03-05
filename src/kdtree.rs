use crate::geometry::aap::*;
use crate::geometry::algorithms::*;
use crate::geometry::ray::*;
use crate::geometry::triangle::*;

pub mod build;

#[derive(Debug, PartialEq)]
pub enum KdNode {
    Leaf(Vec<Triangle>),
    Node {
        plane: Aap,
        left: Box<KdNode>,
        right: Box<KdNode>,
    },
}

#[derive(Debug, PartialEq)]
pub struct KdTree {
    pub root: KdNode,
}

impl KdTree {
    pub fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<TriangleRayIntersection> {
        debug_assert!(tmin < tmax);
        let mut node = &self.root;
        let mut t1 = tmin;
        let mut t2 = tmax;
        loop {
            match node {
                KdNode::Leaf(triangles) => {
                    match intersect_closest_triangle_ray(triangles, ray, t1, t2) {
                        Some((_, result)) => return Some(result),
                        None if t2 == tmax => return None,
                        _ => {
                            t1 = t2;
                            t2 = tmax;
                            node = &self.root;
                        }
                    }
                }
                KdNode::Node { plane, left, right } => {
                    let axis = plane.axis;
                    if ray.direction[axis] == 0. {
                        node = if ray.origin[axis] <= plane.distance {
                            left
                        } else {
                            right
                        }
                    } else {
                        let t = (plane.distance - ray.origin[axis]) / ray.direction[axis];
                        let fst = if ray.direction[axis] >= 0. {
                            left
                        } else {
                            right
                        };
                        let snd = if ray.direction[axis] >= 0. {
                            right
                        } else {
                            left
                        };
                        if t >= tmax {
                            node = fst;
                        } else if t <= tmin {
                            node = snd;
                        } else {
                            node = fst;
                            t2 = t;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::aap::Axis;
    use nalgebra::vector;

    #[test]
    fn intersect_empty_tree() {
        let tree = KdTree {
            root: KdNode::Leaf(vec![]),
        };
        let ray = Ray::between(&vector![0., 0., 0.], &vector![1., 1., 1.]);

        assert_eq!(tree.intersect(&ray, 0., 1.), None);
    }

    #[test]
    fn intersect_ray_intersecting_split_plane_and_both_triangles() {
        let triangle0 = Triangle {
            v0: vector![0., 0., -1.],
            v1: vector![2., 0., -1.],
            v2: vector![2., 2., -1.],
        };
        let triangle1 = Triangle {
            v0: vector![0., 0., 1.],
            v1: vector![2., 0., 1.],
            v2: vector![2., 2., 1.],
        };
        let tree = KdTree {
            root: KdNode::Node {
                plane: Aap {
                    axis: Axis::X,
                    distance: 1.,
                },
                left: Box::new(KdNode::Leaf(vec![triangle0.clone(), triangle1.clone()])),
                right: Box::new(KdNode::Leaf(vec![triangle0.clone(), triangle1.clone()])),
            },
        };
        let ray1 = Ray::between(&vector![1., 1., -2.], &vector![1., 1., 2.]);
        let ray2 = ray1.reverse();

        assert_eq!(
            tree.intersect(&ray1, 0., 1.),
            Some(TriangleRayIntersection {
                t: 0.25,
                u: 0.,
                v: 0.5
            })
        );
        assert_eq!(
            tree.intersect(&ray2, 0., 1.),
            Some(TriangleRayIntersection {
                t: 0.25,
                u: 0.,
                v: 0.5
            })
        );
    }

    #[test]
    fn intersect_ray_parallel_to_split_plane_and_intersecting_one_triangle() {
        let triangle0 = Triangle {
            v0: vector![0., 0., 0.],
            v1: vector![1., 0., 0.],
            v2: vector![0., 1., 0.],
        };
        let triangle1 = Triangle {
            v0: vector![1., 0., 0.],
            v1: vector![2., 0., 0.],
            v2: vector![2., 1., 0.],
        };
        let tree = KdTree {
            root: KdNode::Node {
                plane: Aap {
                    axis: Axis::X,
                    distance: 1.,
                },
                left: Box::new(KdNode::Leaf(vec![triangle0])),
                right: Box::new(KdNode::Leaf(vec![triangle1])),
            },
        };
        let ray_triangle0_v0 = Ray::between(&vector![0., 0., -1.], &vector![0., 0., 1.]);
        let ray_triangle1_v1 = Ray::between(&vector![2., 0., -1.], &vector![2., 0., 1.]);

        assert_eq!(
            tree.intersect(&ray_triangle0_v0, 0., 1.),
            Some(TriangleRayIntersection {
                t: 0.5,
                u: 0.,
                v: 0.
            })
        );
        assert_eq!(
            tree.intersect(&ray_triangle1_v1, 0., 1.),
            Some(TriangleRayIntersection {
                t: 0.5,
                u: 1.,
                v: 0.
            })
        );
    }

    #[test]
    fn intersect_ray_orthogonal_to_split_plane_and_intersecting_both_triangles() {
        let triangle0 = Triangle {
            v0: vector![0., -1., -1.],
            v1: vector![0., 1., -1.],
            v2: vector![0., 1., 1.],
        };
        let triangle1 = Triangle {
            v0: vector![2., -1., -1.],
            v1: vector![2., 1., -1.],
            v2: vector![2., 1., 1.],
        };
        let tree = KdTree {
            root: KdNode::Node {
                plane: Aap {
                    axis: Axis::X,
                    distance: 1.,
                },
                left: Box::new(KdNode::Leaf(vec![triangle0])),
                right: Box::new(KdNode::Leaf(vec![triangle1])),
            },
        };
        let ray1 = Ray::between(&vector![-1., 0., 0.], &vector![3., 0., 0.]);
        let ray2 = ray1.reverse();

        assert_eq!(
            tree.intersect(&ray1, 0., 1.),
            Some(TriangleRayIntersection {
                t: 0.25,
                u: 0.,
                v: 0.5
            })
        );
        assert_eq!(
            tree.intersect(&ray2, 0., 1.),
            Some(TriangleRayIntersection {
                t: 0.25,
                u: 0.,
                v: 0.5
            })
        );
    }
}
