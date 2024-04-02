use std::fmt::Display;

use geometry::{
    aabb::Aabb,
    aap::Aap,
    algorithms::{intersect_triangle_ray, triangles_bounding_box, TriangleRayIntersection},
    ray::Ray,
    triangle::Triangle,
};

pub mod build;
pub mod build_median;
pub mod build_sah;
pub mod split;

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
    pub fn empty() -> Box<Self> {
        Box::new(Self::Leaf(vec![]))
    }

    pub fn new_leaf(triangle_indices: Vec<u32>) -> Box<Self> {
        Box::new(Self::Leaf(triangle_indices))
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
            Self::Leaf(triangle_indices) => triangle_indices.is_empty(),
            Self::Node { .. } => false,
        }
    }
}

impl Display for KdNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KdNode::Leaf(triangle_indices) => write!(f, "{triangle_indices:?}"),
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
    boundary: Aabb,
    node: &KdNode,
) -> f32 {
    match node {
        KdNode::Leaf(triangle_indices) => {
            cost_intersect * triangle_indices.len() as f32 * boundary.surface_area()
                / scene_surface_area
        }
        KdNode::Node { plane, left, right } => {
            let split_cost = boundary.surface_area() / scene_surface_area;
            let (left_aabb, right_aabb) = boundary.split(plane);
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
            (node, t1, t2) = match node {
                KdNode::Leaf(triangle_indices) => {
                    match self.intersect_closest_triangle_ray(triangle_indices, ray, t1, t2) {
                        Some(result) => return Some(result),
                        _ if t2 == tmax => return None,
                        _ => (self.root.as_ref(), t2, tmax),
                    }
                }
                KdNode::Node { plane, left, right } => {
                    let axis = plane.axis;
                    if ray.direction[axis] == 0. {
                        match ray.origin[axis] <= plane.distance {
                            true => (left.as_ref(), t1, t2),
                            false => (right.as_ref(), t1, t2),
                        }
                    } else {
                        let t = (plane.distance - ray.origin[axis]) / ray.direction[axis];
                        let (near, far) = match ray.direction[axis] >= 0. {
                            true => (left.as_ref(), right.as_ref()),
                            false => (right.as_ref(), left.as_ref()),
                        };
                        if t >= t2 {
                            (near, t1, t2)
                        } else if t <= t1 {
                            (far, t1, t2)
                        } else {
                            (near, t1, t)
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
    use geometry::aap::Axis;
    use nalgebra::Vector3;

    #[test]
    fn intersect_empty_tree() {
        let tree = KdTree {
            root: Box::new(KdNode::Leaf(vec![])),
            triangles: vec![],
        };
        let ray = Ray::between(&Vector3::new(0., 0., 0.), &Vector3::new(1., 1., 1.));

        assert_eq!(tree.intersect(&ray, 0., 1.), None);
    }

    #[test]
    fn intersect_ray_intersecting_split_plane_and_both_triangles() {
        let triangle0 = Triangle {
            v0: Vector3::new(0., 0., -1.),
            v1: Vector3::new(2., 0., -1.),
            v2: Vector3::new(2., 2., -1.),
        };
        let triangle1 = Triangle {
            v0: Vector3::new(0., 0., 1.),
            v1: Vector3::new(2., 0., 1.),
            v2: Vector3::new(2., 2., 1.),
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
        let ray = Ray::between(&Vector3::new(1., 1., -2.), &Vector3::new(1., 1., 2.));

        assert_eq!(
            tree.intersect(&ray, 0., 1.),
            Some((0, TriangleRayIntersection::new(0.25, 0., 0.5)))
        );
        assert_eq!(
            tree.intersect(&ray.reverse(), 0., 1.),
            Some((1, TriangleRayIntersection::new(0.25, 0., 0.5)))
        );
    }

    #[test]
    fn intersect_ray_parallel_to_split_plane_and_intersecting_one_triangle() {
        let triangle0 = Triangle {
            v0: Vector3::new(0., 0., 0.),
            v1: Vector3::new(1., 0., 0.),
            v2: Vector3::new(0., 1., 0.),
        };
        let triangle1 = Triangle {
            v0: Vector3::new(1., 0., 0.),
            v1: Vector3::new(2., 0., 0.),
            v2: Vector3::new(2., 1., 0.),
        };
        let tree = KdTree {
            root: KdNode::new_node(
                Aap::new_x(1.0),
                KdNode::new_leaf(vec![0]),
                KdNode::new_leaf(vec![1]),
            ),
            triangles: vec![triangle0, triangle1],
        };
        let ray_triangle0_v0 = Ray::between(&Vector3::new(0., 0., -1.), &Vector3::new(0., 0., 1.));
        let ray_triangle1_v1 = Ray::between(&Vector3::new(2., 0., -1.), &Vector3::new(2., 0., 1.));

        assert_eq!(
            tree.intersect(&ray_triangle0_v0, 0., 1.),
            Some((0, TriangleRayIntersection::new(0.5, 0., 0.)))
        );
        assert_eq!(
            tree.intersect(&ray_triangle1_v1, 0., 1.),
            Some((1, TriangleRayIntersection::new(0.5, 1., 0.)))
        );
    }

    #[test]
    fn intersect_ray_orthogonal_to_split_plane_and_intersecting_both_triangles() {
        let triangle0 = Triangle {
            v0: Vector3::new(0., -1., -1.),
            v1: Vector3::new(0., 1., -1.),
            v2: Vector3::new(0., 1., 1.),
        };
        let triangle1 = Triangle {
            v0: Vector3::new(2., -1., -1.),
            v1: Vector3::new(2., 1., -1.),
            v2: Vector3::new(2., 1., 1.),
        };
        let tree = KdTree {
            root: KdNode::new_node(
                Aap::new_x(1.0),
                KdNode::new_leaf(vec![0]),
                KdNode::new_leaf(vec![1]),
            ),
            triangles: vec![triangle0, triangle1],
        };
        let ray = Ray::between(&Vector3::new(-1., 0., 0.), &Vector3::new(3., 0., 0.));

        assert_eq!(
            tree.intersect(&ray, 0., 1.),
            Some((0, TriangleRayIntersection::new(0.25, 0., 0.5)))
        );
        assert_eq!(
            tree.intersect(&ray.reverse(), 0., 1.),
            Some((1, TriangleRayIntersection::new(0.25, 0., 0.5)))
        );
    }

    #[test]
    fn intersect_split_at_axially_aligned_triangle() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 1.),
            v1: Vector3::new(1., 0., 1.),
            v2: Vector3::new(0., 1., 1.),
        };
        let tree_left = KdTree {
            root: KdNode::new_node(Aap::new_z(1.0), KdNode::new_leaf(vec![0]), KdNode::empty()),
            triangles: vec![triangle.clone()],
        };
        let tree_right = KdTree {
            root: KdNode::new_node(Aap::new_z(1.0), KdNode::empty(), KdNode::new_leaf(vec![0])),
            triangles: vec![triangle],
        };
        let ray = Ray::between(&Vector3::new(0., 0., 0.), &Vector3::new(0., 0., 2.));

        assert_eq!(
            tree_left.intersect(&ray, 0., 1.),
            Some((0, TriangleRayIntersection::new(0.5, 0., 0.)))
        );
        assert_eq!(
            tree_right.intersect(&ray, 0., 1.),
            Some((0, TriangleRayIntersection::new(0.5, 0., 0.)))
        );
    }

    #[test]
    fn intersect_flat_cell() {
        let triangle = Triangle {
            v0: Vector3::new(0., 0., 1.),
            v1: Vector3::new(1., 0., 1.),
            v2: Vector3::new(0., 1., 1.),
        };
        let tree = KdTree {
            root: KdNode::new_node(
                Aap::new_z(1.0),
                KdNode::new_node(Aap::new_z(1.0), KdNode::new_leaf(vec![0]), KdNode::empty()),
                KdNode::empty(),
            ),
            triangles: vec![triangle.clone()],
        };
        let ray = Ray::between(&Vector3::new(0., 0., 0.), &Vector3::new(0., 0., 2.));

        assert_eq!(
            tree.intersect(&ray, 0., 1.),
            Some((0, TriangleRayIntersection::new(0.5, 0., 0.)))
        );
        assert_eq!(
            tree.intersect(&ray.reverse(), 0., 1.),
            Some((0, TriangleRayIntersection::new(0.5, 0., 0.)))
        );
    }
}
