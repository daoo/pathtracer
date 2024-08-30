use crate::{
    cell::KdCell,
    event::{generate_event_list, EventKind},
};
use geometry::{aabb::Aabb, aap::Aap, axis::Axis, geometry::Geometry};

#[derive(Debug, PartialEq)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
struct SahSplit {
    plane: Aap,
    side: Side,
    cost: f32,
}

impl SahSplit {
    fn new_left(plane: Aap, cost: f32) -> Self {
        Self {
            plane,
            side: Side::Left,
            cost,
        }
    }

    fn new_right(plane: Aap, cost: f32) -> Self {
        Self {
            plane,
            side: Side::Right,
            cost,
        }
    }

    fn min(self, other: Self) -> Self {
        if self.cost <= other.cost {
            self
        } else {
            other
        }
    }

    fn zip_min(a: Option<SahSplit>, b: Option<SahSplit>) -> Option<SahSplit> {
        match (a, b) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => Some(a.min(b)),
        }
    }
}

pub struct SahCost {
    pub traverse_cost: f32,
    pub intersect_cost: f32,
    pub empty_factor: f32,
}

impl SahCost {
    fn leaf_cost(&self, count: usize) -> f32 {
        self.intersect_cost * count as f32
    }

    fn split_cost(
        &self,
        volume: (f32, f32),
        probability: (f32, f32),
        counts: (usize, usize),
    ) -> f32 {
        debug_assert!((0.0..=1.0).contains(&probability.0) && (0.0..=1.0).contains(&probability.1));
        debug_assert!(probability.0 > 0.0 || probability.1 > 0.0);
        // TODO: if empty side is flat, apply no empty factor
        let empty_factor = if counts.0 == 0 && volume.0 > 0.01 || counts.1 == 0 && volume.1 > 0.01 {
            self.empty_factor
        } else {
            1.0
        };
        let intersect_cost = self.intersect_cost
            * (probability.0 * counts.0 as f32 + probability.1 * counts.1 as f32);
        empty_factor * (self.traverse_cost + intersect_cost)
    }

    fn split_cost_with_planar(
        &self,
        boundary: &Aabb,
        plane: Aap,
        counts: (usize, usize, usize),
    ) -> Option<SahSplit> {
        let count = counts.0 + counts.1 + counts.2;
        if boundary.volume() == 0.0 || count == 0 {
            return None;
        }
        let (left, right) = boundary.split(&plane);
        let surface_area = boundary.surface_area();
        let volume = (left.volume(), right.volume());
        let probability = (
            left.surface_area() / surface_area,
            right.surface_area() / surface_area,
        );
        let intersect_cost = self.leaf_cost(count);
        if volume.0 > 0.0 && volume.1 > 0.0 {
            let l = self.split_cost(volume, probability, (counts.0 + counts.1, counts.2));
            let r = self.split_cost(volume, probability, (counts.0, counts.2 + counts.1));
            (l < intersect_cost || r < intersect_cost).then(|| {
                if l <= r {
                    SahSplit::new_left(plane, l)
                } else {
                    SahSplit::new_right(plane, r)
                }
            })
        } else if volume.0 == 0.0 && counts.0 + counts.1 > 0 {
            let split_cost = self.split_cost(volume, probability, (counts.0 + counts.1, counts.2));
            (split_cost < intersect_cost).then(|| SahSplit::new_left(plane, split_cost))
        } else if volume.1 == 0.0 && counts.1 + counts.2 > 0 {
            let split_cost = self.split_cost(volume, probability, (counts.0, counts.1 + counts.2));
            (split_cost < intersect_cost).then(|| SahSplit::new_right(plane, split_cost))
        } else {
            None
        }
    }
}

impl Default for SahCost {
    fn default() -> Self {
        SahCost {
            traverse_cost: 1.0,
            intersect_cost: 1.5,
            empty_factor: 0.8,
        }
    }
}

#[derive(Debug)]
pub(crate) struct KdSplit {
    pub(crate) plane: Aap,
    pub(crate) left: KdCell,
    pub(crate) right: KdCell,
}

fn sweep_plane(
    sah: &SahCost,
    boundary: &Aabb,
    clipped: &[(u32, Aabb)],
    axis: Axis,
) -> Option<SahSplit> {
    let events = generate_event_list(clipped, axis);
    let mut best_cost: Option<SahSplit> = None;
    let mut n_left = 0;
    let mut n_right = clipped.len();
    let mut i = 0;
    while i < events.len() {
        let p = Aap {
            axis,
            distance: events[i].distance,
        };

        let p_end = events[i..]
            .iter()
            .take_while(|e| e.distance == p.distance && e.kind == EventKind::End)
            .count();
        i += p_end;
        let p_planar = events[i..]
            .iter()
            .take_while(|e| e.distance == p.distance && e.kind == EventKind::Planar)
            .count();
        i += p_planar;
        let p_start = events[i..]
            .iter()
            .take_while(|e| e.distance == p.distance && e.kind == EventKind::Start)
            .count();
        i += p_start;

        n_right -= p_planar;
        n_right -= p_end;
        let cost = sah.split_cost_with_planar(boundary, p, (n_left, p_planar, n_right));
        best_cost = SahSplit::zip_min(best_cost, cost);
        n_left += p_start;
        n_left += p_planar;
    }
    best_cost
}

fn repartition(cell: &KdCell, clipped: &[(u32, Aabb)], best: SahSplit) -> KdSplit {
    let plane = best.plane;
    let mut left_indices: Vec<u32> = Vec::with_capacity(clipped.len());
    let mut right_indices: Vec<u32> = Vec::with_capacity(clipped.len());
    for (index, boundary) in clipped {
        let planar = boundary.min()[plane.axis] == plane.distance
            && boundary.max()[plane.axis] == plane.distance;
        let left = boundary.min()[plane.axis] < plane.distance;
        let right = boundary.max()[plane.axis] > plane.distance;
        if left || planar && best.side == Side::Left {
            left_indices.push(*index);
        }
        if right || planar && best.side == Side::Right {
            right_indices.push(*index);
        }
    }
    let (left_aabb, right_aabb) = cell.boundary.split(&plane);
    KdSplit {
        plane,
        left: KdCell::new(left_aabb, left_indices),
        right: KdCell::new(right_aabb, right_indices),
    }
}

pub(crate) fn find_best_split(
    geometries: &[Geometry],
    sah: &SahCost,
    cell: &KdCell,
) -> Option<KdSplit> {
    debug_assert!(
        !cell.indices.is_empty(),
        "splitting a kd-cell with no geometries only worsens performance"
    );

    let clipped = cell.clip_geometries(geometries);
    let x = sweep_plane(sah, &cell.boundary, &clipped, Axis::X);
    let y = sweep_plane(sah, &cell.boundary, &clipped, Axis::Y);
    let z = sweep_plane(sah, &cell.boundary, &clipped, Axis::Z);
    SahSplit::zip_min(x, SahSplit::zip_min(y, z)).map(|best| repartition(cell, &clipped, best))
}
