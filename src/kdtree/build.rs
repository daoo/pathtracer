use crate::geometry::aabb::*;
use crate::geometry::bounding::*;
use crate::kdtree::*;

#[derive(Clone, Debug)]
struct KdBox<'t> {
    boundary: Aabb,
    triangles: Vec<&'t Triangle>,
}

#[derive(Clone, Debug)]
struct KdSplit<'t> {
    left: KdBox<'t>,
    right: KdBox<'t>,
}

fn split_box<'t>(parent: KdBox<'t>, plane: &Aap) -> KdSplit<'t> {
    let (left_aabb, right_aabb) = parent.boundary.split(plane);
    let mut left_triangles: Vec<&'t Triangle> = Vec::new();
    let mut right_triangles: Vec<&'t Triangle> = Vec::new();
    for triangle in &parent.triangles {
        let in_left = intersect_triangle_aabb(triangle, &left_aabb);
        let in_right = intersect_triangle_aabb(triangle, &right_aabb);
        if !(in_left || in_right) {
            let strfix = |s: String| s.replace("[[", "vector![").replace("]]", "]");
            println!("{}", strfix(format!("let left_aabb = {:?};", &left_aabb)));
            println!("{}", strfix(format!("let right_aabb = {:?};", &right_aabb)));
            println!("{}", strfix(format!("let triangles = {:?};", &parent.triangles)));
            println!("{}", strfix(format!("let wrong_triangle = {:?};", [triangle])));
        }
        debug_assert!(in_left || in_right);
        if in_left { left_triangles.push(&triangle); }
        if in_right { right_triangles.push(&triangle); }
    }
    KdSplit{
        left: KdBox{boundary: left_aabb, triangles: left_triangles},
        right: KdBox{boundary: right_aabb, triangles: right_triangles},
    }
}

fn median(triangles: &[&Triangle], axis: Axis) -> f32 {
    let mut points: Vec<f32> = triangles
        .iter()
        .map(|t| [t.min()[axis], t.max()[axis]])
        .flatten()
        .collect();
    points.sort_by(f32::total_cmp);
    let middle = points.len() / 2;
    if points.len() % 2 == 0 {
        (points[middle] + points[middle + 1]) / 2.
    } else {
        points[middle]
    }
}

static NEXT_AXIS: [Axis; 3] = [Axis::Y, Axis::Z, Axis::X];

fn build_kdtree_median_internal<'t>(max_depth: u32, depth: u32, axis: Axis, parent: KdBox<'t>) -> Box<KdNode<'t>> {
    if depth >= max_depth {
        return Box::new(KdNode::Leaf(parent.triangles.clone()))
    }

    let plane = Aap{ axis, distance: median(&parent.triangles, axis) };
    let split = split_box(parent, &plane);
    let left = build_kdtree_median_internal(max_depth, depth + 1, NEXT_AXIS[axis], split.left);
    let right = build_kdtree_median_internal(max_depth, depth + 1, NEXT_AXIS[axis], split.right);
    Box::new(KdNode::Node { plane, left, right })
}

pub fn build_kdtree_median<'t>(max_depth: u32, triangles: &'t [Triangle]) -> KdTree<'t> {
    let triangle_refs: Vec<&'t Triangle> = triangles.iter().collect();
    let boundary: KdBox<'t> = KdBox{ boundary: bounding(triangles), triangles: triangle_refs };
    KdTree{ root: *build_kdtree_median_internal(max_depth, 0, Axis::X, boundary) }
}
