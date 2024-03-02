use crate::geometry::aap::Aap;
use crate::geometry::ray::Ray;
use crate::geometry::triangle::Triangle;
use crate::geometry::intersect::TriangleRayIntersection;
use crate::geometry::intersect::find_closest_intersection;

#[derive(Debug, PartialEq)]
pub enum KdNode<'t> {
    Leaf(&'t [Triangle]),
    Node { plane: Aap, left: Box<KdNode<'t>>, right: Box<KdNode<'t>>, },
}

#[derive(Debug, PartialEq)]
pub struct KdTree<'t> {
    root: KdNode<'t>
}

impl<'t> KdTree<'t> {
    pub fn intersect(&'t self, ray: &Ray, tmin: f32, tmax: f32) -> Option<TriangleRayIntersection> {
        let mut node = &self.root;
        let mut t1 = tmin;
        let mut t2 = tmax;
        loop {
            match node {
                KdNode::Leaf(triangles) => {
                    match find_closest_intersection(&triangles, ray, t1, t2) {
                        result@Some(_) => return result,
                        None if tmax == tmax => return None,
                        _ => {
                            t1 = t2;
                            t2 = tmax;
                            node = &self.root;
                        }
                    }
                },
                KdNode::Node{plane, left, right} => {
                    let axis = plane.axis;
                    let t = (plane.distance - ray.origin[axis]) / ray.direction[axis];
                    dbg!(plane.distance - ray.origin[axis]);
                    dbg!(ray.direction[axis]);
                    let (fst, snd) = if ray.direction[axis] >= 0.0 {
                        (&left, &right)
                    } else {
                        (&right, &left)
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
    fn test_intersect_empty_tree() {
        let tree = KdTree{ root: KdNode::Leaf(&[]) };
        let ray = Ray::between(&Vector3::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 1.0, 1.0));

        assert_eq!(tree.intersect(&ray, 0.0, 1.0), None);
    }

    #[test]
    fn test_intersect_split_with_one_triangle_in_each() {
        let triangles = [
            triangle(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0)),
            triangle(Vector3::new(1.0, 0.0, 0.0), Vector3::new(2.0, 0.0, 0.0), Vector3::new(2.0, 1.0, 0.0)),
        ];
        let tree = KdTree{
            root: KdNode::Node{
                plane: Aap { axis: Axis::X, distance: 1.0 },
                left: Box::new(KdNode::Leaf(&triangles[0..1])),
                right: Box::new(KdNode::Leaf(&triangles[1..2])),
            }
        };
        let ray1 = Ray::between(&Vector3::new(1.0, 1.0, -1.0), &Vector3::new(1.0, 1.0, 1.0));
        let ray2 = Ray::between(&Vector3::new(0.0, 0.0, -1.0), &Vector3::new(0.0, 0.0, 1.0));
        let ray3 = Ray::between(&Vector3::new(2.0, 0.0, -1.0), &Vector3::new(2.0, 0.0, 1.0));

        assert_eq!(tree.intersect(&ray1, 0.0, 1.0), None);
        assert_eq!(tree.intersect(&ray2, 0.0, 1.0), Some(TriangleRayIntersection { t: 0.5, u: 0.0, v: 0.0 }));
        assert_eq!(tree.intersect(&ray3, 0.0, 1.0), Some(TriangleRayIntersection { t: 0.5, u: 1.0, v: 0.0 }));
    }
}
