use crate::geometry::aabb::*;
use crate::geometry::algorithms::*;
use crate::kdtree::*;
use nalgebra::vector;

#[derive(Debug)]
struct KdBox {
    boundary: Aabb,
    triangles: Vec<Triangle>,
}

#[derive(Debug)]
struct KdSplit {
    left: KdBox,
    right: KdBox,
}

fn split_box(parent: KdBox, plane: &Aap) -> KdSplit {
    let (left_aabb, right_aabb) = parent.boundary.split(plane);
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
    let mut left_triangles: Vec<Triangle> = Vec::new();
    let mut right_triangles: Vec<Triangle> = Vec::new();
    for triangle in &parent.triangles {
        let in_left = intersect_triangle_aabb(triangle, &left_aabb);
        let in_right = intersect_triangle_aabb(triangle, &right_aabb);
        if !(in_left || in_right) {
            let strfix = |s: String| s.replace("[[", "vector![").replace("]]", "]");
            println!("{}", strfix(format!("let left_aabb = {:?};", &left_aabb)));
            println!("{}", strfix(format!("let right_aabb = {:?};", &right_aabb)));
            println!(
                "{}",
                strfix(format!("let triangles = {:?};", &parent.triangles))
            );
            println!(
                "{}",
                strfix(format!("let wrong_triangle = {:?};", [triangle]))
            );
        }
        debug_assert!(in_left || in_right);
        if in_left {
            left_triangles.push(triangle.clone());
        }
        if in_right {
            right_triangles.push(triangle.clone());
        }
    }
    KdSplit {
        left: KdBox {
            boundary: left_aabb,
            triangles: left_triangles,
        },
        right: KdBox {
            boundary: right_aabb,
            triangles: right_triangles,
        },
    }
}

fn potential_split_points(triangles: &[Triangle], axis: Axis, boundary: &Aabb) -> Vec<f32> {
    let min = boundary.min()[axis] + 0.1;
    let max = boundary.max()[axis] - 0.1;
    let mut points = triangles
        .iter()
        .flat_map(|t| [t.min()[axis] - 0.1, t.max()[axis] + 0.1])
        .filter(|p| p > &min && p < &max)
        .collect::<Vec<_>>();
    points.sort_by(f32::total_cmp);
    points
}

fn median(values: &[f32]) -> f32 {
    debug_assert!(!values.is_empty());
    if values.len() == 1 {
        return values[0];
    }

    let middle = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[middle - 1] + values[middle]) / 2.
    } else {
        values[middle]
    }
}

static NEXT_AXIS: [Axis; 3] = [Axis::Y, Axis::Z, Axis::X];

fn build_kdtree_median_internal(
    max_depth: u32,
    depth: u32,
    axis: Axis,
    parent: KdBox,
) -> Box<KdNode> {
    if depth >= max_depth {
        return Box::new(KdNode::Leaf(parent.triangles));
    }

    let points = potential_split_points(&parent.triangles, axis, &parent.boundary);
    if points.is_empty() {
        return  Box::new(KdNode::Leaf(parent.triangles));
    }

    let plane = Aap {
        axis,
        distance: median(&points),
    };
    let split = split_box(parent, &plane);
    let left = build_kdtree_median_internal(max_depth, depth + 1, NEXT_AXIS[axis], split.left);
    let right = build_kdtree_median_internal(max_depth, depth + 1, NEXT_AXIS[axis], split.right);
    Box::new(KdNode::Node { plane, left, right })
}

pub fn build_kdtree_median(max_depth: u32, triangles: &[Triangle]) -> KdTree {
    let kdbox: KdBox = KdBox {
        boundary: triangles_bounding_box(triangles).enlarge(&vector![0.1, 0.1, 0.1]),
        triangles: triangles.to_vec(),
    };
    KdTree {
        root: *build_kdtree_median_internal(max_depth, 0, Axis::X, kdbox),
    }
}
