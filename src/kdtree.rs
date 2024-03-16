use crate::geometry::aabb::Aabb;
use crate::geometry::aap::*;
use crate::geometry::algorithms::*;
use crate::geometry::ray::*;
use crate::geometry::triangle::*;

pub mod build;
pub mod build_naive;
pub mod build_sah;

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
    pub fn new_leaf(triangle_indices: Vec<u32>) -> Box<KdNode> {
        Box::new(KdNode::Leaf(triangle_indices))
    }

    pub fn new_node(
        axis: Axis,
        distance: f32,
        left: Box<KdNode>,
        right: Box<KdNode>,
    ) -> Box<KdNode> {
        Box::new(KdNode::Node {
            plane: Aap { axis, distance },
            left,
            right,
        })
    }

    pub fn is_empty(&self) -> bool {
        match self {
            KdNode::Leaf(triangle_indices) => triangle_indices.is_empty(),
            KdNode::Node { .. } => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct KdTree {
    pub root: Box<KdNode>,
    pub triangles: Vec<Triangle>,
}

fn tree_cost(
    cost_traverse: f32,
    cost_intersect: f32,
    empty_factor: f32,
    scene_surface_area: f32,
    parent: Aabb,
    node: &KdNode,
) -> f32 {
    match node {
        KdNode::Leaf(triangle_indices) => {
            cost_intersect * triangle_indices.len() as f32 * parent.surface_area()
                / scene_surface_area
        }
        KdNode::Node { plane, left, right } => {
            let split_cost = parent.surface_area() / scene_surface_area;
            let (left_aabb, right_aabb) = parent.split(plane);
            let left_cost = tree_cost(
                cost_traverse,
                cost_intersect,
                empty_factor,
                scene_surface_area,
                left_aabb,
                left,
            );
            let right_cost = tree_cost(
                cost_traverse,
                cost_intersect,
                empty_factor,
                scene_surface_area,
                right_aabb,
                right,
            );
            let node_cost = cost_traverse + split_cost + left_cost + right_cost;
            let factor = match left.is_empty() || right.is_empty() {
                true => empty_factor,
                _ => 1.0,
            };
            factor * node_cost
        }
    }
}

impl KdTree {
    pub fn cost(&self, cost_traverse: f32, cost_intersect: f32, empty_factor: f32) -> f32 {
        let bounding_box = triangles_bounding_box(&self.triangles);
        tree_cost(
            cost_traverse,
            cost_intersect,
            empty_factor,
            bounding_box.surface_area(),
            bounding_box,
            self.root.as_ref(),
        )
    }

    fn intersect_closest_triangle_ray(
        &self,
        triangles: &[u32],
        ray: &Ray,
        tmin: f32,
        tmax: f32,
    ) -> Option<(usize, TriangleRayIntersection)> {
        debug_assert!(tmin < tmax);
        let mut closest: Option<(usize, TriangleRayIntersection)> = None;
        let t1 = tmin;
        let mut t2 = tmax;
        for index in triangles {
            let index = *index as usize;
            closest = match intersect_triangle_ray(&self.triangles[index], ray) {
                Some(intersection) if intersection.t >= t1 && intersection.t <= t2 => {
                    t2 = intersection.t;
                    Some((index, intersection))
                }
                _ => closest,
            };
        }
        closest
    }

    pub fn intersect(
        &self,
        ray: &Ray,
        tmin: f32,
        tmax: f32,
    ) -> Option<(usize, TriangleRayIntersection)> {
        debug_assert!(tmin < tmax);
        let mut node = self.root.as_ref();
        let mut t1 = tmin;
        let mut t2 = tmax;
        loop {
            match node {
                KdNode::Leaf(triangle_indices) => {
                    if let Some(result) =
                        self.intersect_closest_triangle_ray(triangle_indices, ray, t1, t2)
                    {
                        return Some(result);
                    } else if t2 == tmax {
                        return None;
                    } else {
                        t1 = t2;
                        t2 = tmax;
                        node = &self.root;
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
                            left.as_ref()
                        } else {
                            right.as_ref()
                        };
                        let snd: &KdNode = if ray.direction[axis] >= 0. {
                            right.as_ref()
                        } else {
                            left.as_ref()
                        };
                        if t >= t2 {
                            node = fst;
                        } else if t <= t1 {
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
            root: Box::new(KdNode::Leaf(vec![])),
            triangles: vec![],
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
            root: Box::new(KdNode::Node {
                plane: Aap {
                    axis: Axis::X,
                    distance: 1.,
                },
                left: Box::new(KdNode::Leaf(vec![0, 1])),
                right: Box::new(KdNode::Leaf(vec![0, 1])),
            }),
            triangles: vec![triangle0, triangle1],
        };
        let ray1 = Ray::between(&vector![1., 1., -2.], &vector![1., 1., 2.]);
        let ray2 = ray1.reverse();

        assert_eq!(
            tree.intersect(&ray1, 0., 1.),
            Some((
                0,
                TriangleRayIntersection {
                    t: 0.25,
                    u: 0.,
                    v: 0.5
                }
            ))
        );
        assert_eq!(
            tree.intersect(&ray2, 0., 1.),
            Some((
                1,
                TriangleRayIntersection {
                    t: 0.25,
                    u: 0.,
                    v: 0.5
                }
            ))
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
            root: Box::new(KdNode::Node {
                plane: Aap {
                    axis: Axis::X,
                    distance: 1.,
                },
                left: Box::new(KdNode::Leaf(vec![0])),
                right: Box::new(KdNode::Leaf(vec![1])),
            }),
            triangles: vec![triangle0, triangle1],
        };
        let ray_triangle0_v0 = Ray::between(&vector![0., 0., -1.], &vector![0., 0., 1.]);
        let ray_triangle1_v1 = Ray::between(&vector![2., 0., -1.], &vector![2., 0., 1.]);

        assert_eq!(
            tree.intersect(&ray_triangle0_v0, 0., 1.),
            Some((
                0,
                TriangleRayIntersection {
                    t: 0.5,
                    u: 0.,
                    v: 0.
                }
            ))
        );
        assert_eq!(
            tree.intersect(&ray_triangle1_v1, 0., 1.),
            Some((
                1,
                TriangleRayIntersection {
                    t: 0.5,
                    u: 1.,
                    v: 0.
                }
            ))
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
            root: Box::new(KdNode::Node {
                plane: Aap {
                    axis: Axis::X,
                    distance: 1.,
                },
                left: Box::new(KdNode::Leaf(vec![0])),
                right: Box::new(KdNode::Leaf(vec![1])),
            }),
            triangles: vec![triangle0, triangle1],
        };
        let ray1 = Ray::between(&vector![-1., 0., 0.], &vector![3., 0., 0.]);
        let ray2 = ray1.reverse();

        assert_eq!(
            tree.intersect(&ray1, 0., 1.),
            Some((
                0,
                TriangleRayIntersection {
                    t: 0.25,
                    u: 0.,
                    v: 0.5
                }
            ))
        );
        assert_eq!(
            tree.intersect(&ray2, 0., 1.),
            Some((
                1,
                TriangleRayIntersection {
                    t: 0.25,
                    u: 0.,
                    v: 0.5
                }
            ))
        );
    }
}
