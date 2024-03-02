use crate::geometry::aap::*;
use crate::geometry::ray::*;
use crate::geometry::triangle::*;
use crate::geometry::intersect::*;

pub mod build;

#[derive(Debug, PartialEq)]
pub enum KdNode<'t> {
    Leaf(Vec<&'t Triangle>),
    Node { plane: Aap, left: Box<KdNode<'t>>, right: Box<KdNode<'t>>, },
}

#[derive(Debug, PartialEq)]
pub struct KdTree<'t> {
    pub root: KdNode<'t>
}

pub fn intersect_closest_triangle_ray(triangles: &[&Triangle], ray: &Ray, tmin: f32, tmax: f32) -> Option<TriangleRayIntersection> {
    debug_assert!(tmin < tmax);
    let mut closest: Option<TriangleRayIntersection> = None;
    let t1 = tmin;
    let mut t2 = tmax;
    for triangle in triangles {
        closest = match intersect_triangle_ray(triangle, ray) {
            Some(intersection) if intersection.t >= t1 && intersection.t <= t2 => {
                t2 = intersection.t;
                Some(intersection)
            },
            _ => closest
        };
    }
    closest
}

impl<'t> KdTree<'t> {
    pub fn intersect(&'t self, ray: &Ray, tmin: f32, tmax: f32) -> Option<TriangleRayIntersection> {
        debug_assert!(tmin < tmax);
        let mut node = &self.root;
        let mut t1 = tmin;
        let mut t2 = tmax;
        loop {
            match node {
                KdNode::Leaf(triangles) => {
                    match intersect_closest_triangle_ray(triangles, ray, t1, t2) {
                        result@Some(_) => return result,
                        None if t2 == tmax => return None,
                        _ => {
                            t1 = t2;
                            t2 = tmax;
                            node = &self.root;
                        }
                    }
                },
                KdNode::Node{plane, left, right} => {
                    let axis = plane.axis;
                    let (fst, snd) = if ray.direction[axis] >= 0.0 {
                        (left, right)
                    } else {
                        (right, left)
                    };

                    if ray.direction[axis] == 0.0 {
                        node = if ray.origin[axis] <= plane.distance { left } else { right }
                    } else {
                        let t = (plane.distance - ray.origin[axis]) / ray.direction[axis];
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
    use nalgebra::Vector2;
    use nalgebra::Vector3;
    use crate::geometry::aap::Axis;

    fn triangle(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>) -> Triangle {
        Triangle {
            v0,
            v1,
            v2,
            n0: Vector3::zeros(),
            n1: Vector3::zeros(),
            n2: Vector3::zeros(),
            uv0: Vector2::zeros(),
            uv1: Vector2::zeros(),
            uv2: Vector2::zeros(),
        }
    }

    #[test]
    fn intersect_empty_tree() {
        let tree = KdTree{ root: KdNode::Leaf(vec![]) };
        let ray = Ray::between(&Vector3::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 1.0, 1.0));

        assert_eq!(tree.intersect(&ray, 0.0, 1.0), None);
    }

    #[test]
    fn intersect_ray_intersecting_split_plane_and_both_triangles() {
        let triangle0 = triangle(Vector3::new(0.0, 0.0, -1.0), Vector3::new(2.0, 0.0, -1.0), Vector3::new(2.0, 2.0, -1.0));
        let triangle1 = triangle(Vector3::new(0.0, 0.0, 1.0), Vector3::new(2.0, 0.0, 1.0), Vector3::new(2.0, 2.0, 1.0));
        let tree = KdTree{
            root: KdNode::Node{
                plane: Aap { axis: Axis::X, distance: 1.0 },
                left: Box::new(KdNode::Leaf(vec![&triangle0, &triangle1])),
                right: Box::new(KdNode::Leaf(vec![&triangle0, &triangle1])),
            }
        };
        let ray1 = Ray::between(&Vector3::new(1.0, 1.0, -2.0), &Vector3::new(1.0, 1.0, 2.0));
        let ray2 = ray1.reverse();

        assert_eq!(tree.intersect(&ray1, 0.0, 1.0), Some(TriangleRayIntersection { t: 0.25, u: 0.0, v: 0.5 }));
        assert_eq!(tree.intersect(&ray2, 0.0, 1.0), Some(TriangleRayIntersection { t: 0.25, u: 0.0, v: 0.5 }));
    }

    #[test]
    fn intersect_ray_parallel_to_split_plane_and_intersecting_one_triangle() {
        let triangle0 = triangle(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let triangle1 = triangle(Vector3::new(1.0, 0.0, 0.0), Vector3::new(2.0, 0.0, 0.0), Vector3::new(2.0, 1.0, 0.0));
        let tree = KdTree{
            root: KdNode::Node{
                plane: Aap { axis: Axis::X, distance: 1.0 },
                left: Box::new(KdNode::Leaf(vec![&triangle0])),
                right: Box::new(KdNode::Leaf(vec![&triangle1])),
            }
        };
        let ray_triangle0_v0 = Ray::between(&Vector3::new(0.0, 0.0, -1.0), &Vector3::new(0.0, 0.0, 1.0));
        let ray_triangle1_v1 = Ray::between(&Vector3::new(2.0, 0.0, -1.0), &Vector3::new(2.0, 0.0, 1.0));

        assert_eq!(tree.intersect(&ray_triangle0_v0, 0.0, 1.0), Some(TriangleRayIntersection { t: 0.5, u: 0.0, v: 0.0 }));
        assert_eq!(tree.intersect(&ray_triangle1_v1, 0.0, 1.0), Some(TriangleRayIntersection { t: 0.5, u: 1.0, v: 0.0 }));
    }

    #[test]
    fn intersect_ray_orthogonal_to_split_plane_and_intersecting_both_triangles() {
        let triangle0 = triangle(Vector3::new(0.0, -1.0, -1.0), Vector3::new(0.0, 1.0, -1.0), Vector3::new(0.0, 1.0, 1.0));
        let triangle1 = triangle(Vector3::new(2.0, -1.0, -1.0), Vector3::new(2.0, 1.0, -1.0), Vector3::new(2.0, 1.0, 1.0));
        let tree = KdTree{
            root: KdNode::Node{
                plane: Aap { axis: Axis::X, distance: 1.0 },
                left: Box::new(KdNode::Leaf(vec![&triangle0])),
                right: Box::new(KdNode::Leaf(vec![&triangle1])),
            }
        };
        let ray1 = Ray::between(&Vector3::new(-1.0, 0.0, 0.0), &Vector3::new(3.0, 0.0, 0.0));
        let ray2 = ray1.reverse();

        assert_eq!(tree.intersect(&ray1, 0.0, 1.0), Some(TriangleRayIntersection{ t: 0.25, u: 0.0, v: 0.5 }));
        assert_eq!(tree.intersect(&ray2, 0.0, 1.0), Some(TriangleRayIntersection{ t: 0.25, u: 0.0, v: 0.5 }));
    }
}
