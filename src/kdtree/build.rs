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

fn split_box<'t>(kd_box: KdBox<'t>, plane: &Aap) -> KdSplit<'t> {
    let (left_aabb, right_aabb) = kd_box.boundary.split(plane);
    let mut left_triangles: Vec<&'t Triangle> = Vec::new();
    let mut right_triangles: Vec<&'t Triangle> = Vec::new();
    for triangle in &kd_box.triangles {
        let clamped_min = kd_box.boundary.clamp(triangle.min())[plane.axis];
        let clamped_max = kd_box.boundary.clamp(triangle.max())[plane.axis];
        let in_left = clamped_min < plane.distance;
        let in_right = clamped_max > plane.distance;
        let in_plane = !in_left && !in_right;
        if in_left { left_triangles.push(&triangle); }
        if in_right { right_triangles.push(&triangle); }
        if in_plane { left_triangles.push(&triangle); }
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
        (points[middle] + points[middle + 1]) / 2.0
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
    dbg!(boundary.boundary.min(), boundary.boundary.max(), boundary.triangles.iter().filter(|t| !intersect_triangle_aabb(t, &boundary.boundary)).collect::<Vec<_>>());
    debug_assert!(boundary.triangles.iter().all(|t| intersect_triangle_aabb(t, &boundary.boundary)));
    KdTree{ root: *build_kdtree_median_internal(max_depth, 0, Axis::X, boundary) }
}
