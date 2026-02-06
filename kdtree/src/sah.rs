use crate::{
    cell::KdCell,
    event::{Event, EventKind, extend_vec_with_events},
};
use geometry::{aabb::Aabb, aap::Aap, axis::Axis, geometry::Geometry};
use itertools::Itertools;

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
    const fn new_left(plane: Aap, cost: f32) -> Self {
        Self {
            plane,
            side: Side::Left,
            cost,
        }
    }

    const fn new_right(plane: Aap, cost: f32) -> Self {
        Self {
            plane,
            side: Side::Right,
            cost,
        }
    }

    fn min(self, other: Self) -> Self {
        if self.cost <= other.cost { self } else { other }
    }

    fn zip_min(a: Option<Self>, b: Option<Self>) -> Option<Self> {
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
        Self {
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
    count: usize,
    axis: Axis,
    events: &[Event],
) -> Option<SahSplit> {
    let advance = |mut i: usize, distance: f32, kind: EventKind| {
        let mut count = 0;
        while i < events.len() && events[i].distance == distance && events[i].kind == kind {
            count += 1;
            i += 1;
        }
        count
    };

    let mut best_cost: Option<SahSplit> = None;
    let mut n_left = 0;
    let mut n_right = count;
    let mut i = 0;
    while i < events.len() {
        let p = Aap {
            axis,
            distance: events[i].distance,
        };

        let p_end = advance(i, p.distance, EventKind::End);
        i += p_end;
        let p_planar = advance(i, p.distance, EventKind::Planar);
        i += p_planar;
        let p_start = advance(i, p.distance, EventKind::Start);
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum EventSide {
    Both,
    LeftOnly,
    RightOnly,
}

fn update_geometry_side(
    indices: &[u32],
    events: &(Vec<Event>, Vec<Event>, Vec<Event>),
    best: &SahSplit,
    sides: &mut [EventSide],
) {
    indices
        .iter()
        .for_each(|i| unsafe { *sides.get_unchecked_mut(*i as usize) = EventSide::Both });
    events[best.plane.axis].iter().for_each(|e| {
        let side = unsafe { sides.get_unchecked_mut(e.index as usize) };
        if e.kind == EventKind::End && e.distance <= best.plane.distance {
            *side = EventSide::LeftOnly;
        } else if e.kind == EventKind::Start && e.distance >= best.plane.distance {
            *side = EventSide::RightOnly;
        } else if e.kind == EventKind::Planar {
            if e.distance < best.plane.distance
                || (e.distance == best.plane.distance && best.side == Side::Left)
            {
                *side = EventSide::LeftOnly;
            } else if e.distance > best.plane.distance
                || (e.distance == best.plane.distance && best.side == Side::Right)
            {
                *side = EventSide::RightOnly;
            }
        }
    });
}

fn repartition(
    geometries: &[impl Geometry],
    cell: &KdCell,
    best: SahSplit,
    sides: &mut [EventSide],
) -> KdSplit {
    update_geometry_side(&cell.indices, &cell.events, &best, sides);

    let (left_aabb, right_aabb) = cell.boundary.split(&best.plane);

    let partition_events = |events: &Vec<Event>| {
        let mut left = Vec::with_capacity(events.len());
        let mut right = Vec::with_capacity(events.len());
        for event in events {
            let side = unsafe { sides.get_unchecked(event.index as usize) };
            match side {
                EventSide::Both => (),
                EventSide::LeftOnly => left.push(event.clone()),
                EventSide::RightOnly => right.push(event.clone()),
            }
        }
        (left, right)
    };
    let partitioned_events_x = partition_events(&cell.events.0);
    let partitioned_events_y = partition_events(&cell.events.1);
    let partitioned_events_z = partition_events(&cell.events.2);

    let mut events_left_both = (
        Vec::with_capacity(cell.events.0.len()),
        Vec::with_capacity(cell.events.1.len()),
        Vec::with_capacity(cell.events.2.len()),
    );
    let mut events_right_both = (
        Vec::with_capacity(cell.events.0.len()),
        Vec::with_capacity(cell.events.1.len()),
        Vec::with_capacity(cell.events.2.len()),
    );
    let mut left_indices = Vec::with_capacity(cell.indices.len());
    let mut right_indices = Vec::with_capacity(cell.indices.len());
    for index in &cell.indices {
        let side = unsafe { sides.get_unchecked(*index as usize) };
        match side {
            EventSide::Both => {
                let geometry = unsafe { geometries.get_unchecked(*index as usize) };
                if let Some(clipped) = geometry.clip_aabb(&left_aabb) {
                    extend_vec_with_events(
                        &mut events_left_both,
                        *index,
                        clipped.min(),
                        clipped.max(),
                    );
                    left_indices.push(*index);
                }
                if let Some(clipped) = geometry.clip_aabb(&right_aabb) {
                    extend_vec_with_events(
                        &mut events_right_both,
                        *index,
                        clipped.min(),
                        clipped.max(),
                    );
                    right_indices.push(*index);
                }
            }
            EventSide::LeftOnly => left_indices.push(*index),
            EventSide::RightOnly => right_indices.push(*index),
        }
    }

    events_left_both.0.sort_unstable_by(Event::total_cmp);
    events_left_both.1.sort_unstable_by(Event::total_cmp);
    events_left_both.2.sort_unstable_by(Event::total_cmp);
    events_right_both.0.sort_unstable_by(Event::total_cmp);
    events_right_both.1.sort_unstable_by(Event::total_cmp);
    events_right_both.2.sort_unstable_by(Event::total_cmp);

    let merge_events =
        |a: Vec<Event>, b: Vec<Event>| a.into_iter().merge_by(b, Event::le).collect::<Vec<_>>();

    let left_events = (
        merge_events(partitioned_events_x.0, events_left_both.0),
        merge_events(partitioned_events_y.0, events_left_both.1),
        merge_events(partitioned_events_z.0, events_left_both.2),
    );
    let right_events = (
        merge_events(partitioned_events_x.1, events_right_both.0),
        merge_events(partitioned_events_y.1, events_right_both.1),
        merge_events(partitioned_events_z.1, events_right_both.2),
    );

    KdSplit {
        plane: best.plane,
        left: KdCell::new(left_aabb, left_indices, left_events),
        right: KdCell::new(right_aabb, right_indices, right_events),
    }
}

pub(crate) fn find_best_split(
    geometries: &[impl Geometry],
    sah: &SahCost,
    cell: &KdCell,
    sides: &mut [EventSide],
) -> Option<KdSplit> {
    debug_assert!(
        !cell.indices.is_empty(),
        "splitting a kd-cell with no geometries only worsens performance"
    );

    let x = sweep_plane(
        sah,
        &cell.boundary,
        cell.indices.len(),
        Axis::X,
        &cell.events.0,
    );
    let y = sweep_plane(
        sah,
        &cell.boundary,
        cell.indices.len(),
        Axis::Y,
        &cell.events.1,
    );
    let z = sweep_plane(
        sah,
        &cell.boundary,
        cell.indices.len(),
        Axis::Z,
        &cell.events.2,
    );
    SahSplit::zip_min(x, SahSplit::zip_min(y, z))
        .map(|best| repartition(geometries, cell, best, sides))
}

#[cfg(test)]
mod tests {
    use geometry::triangle::Triangle;
    use glam::Vec3;

    use crate::event::generate_event_list;

    use super::*;

    #[test]
    fn update_geometry_side_left_only() {
        let geometries = [Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 0.0),
        }];
        let events = generate_event_list(&geometries);
        let best = SahSplit::new_left(Aap::new_x(1.0), 1.0);

        let mut actual = [EventSide::Both];
        update_geometry_side(&[0], &events, &best, &mut actual);

        let expected = [EventSide::LeftOnly];
        assert_eq!(actual, expected);
    }

    #[test]
    fn update_geometry_side_right_only() {
        let geometries = [Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 0.0),
        }];
        let events = generate_event_list(&geometries);
        let best = SahSplit::new_left(Aap::new_x(0.0), 1.0);

        let mut actual = [EventSide::Both];
        update_geometry_side(&[0], &events, &best, &mut actual);

        let expected = [EventSide::RightOnly];
        assert_eq!(actual, expected);
    }

    #[test]
    fn update_geometry_side_both() {
        let geometries = [Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 0.0),
        }];
        let events = generate_event_list(&geometries);
        let best = SahSplit::new_left(Aap::new_x(0.5), 1.0);

        let mut actual = [EventSide::Both];
        update_geometry_side(&[0], &events, &best, &mut actual);

        let expected = [EventSide::Both];
        assert_eq!(actual, expected);
    }

    #[test]
    fn update_geometry_side_planar_best_left() {
        let geometries = [Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 0.0),
        }];
        let events = generate_event_list(&geometries);
        let best = SahSplit::new_left(Aap::new_z(0.0), 1.0);

        let mut actual = [EventSide::Both];
        update_geometry_side(&[0], &events, &best, &mut actual);

        let expected = [EventSide::LeftOnly];
        assert_eq!(actual, expected);
    }

    #[test]
    fn update_geometry_side_planar_best_right() {
        let geometries = [Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 0.0),
        }];
        let events = generate_event_list(&geometries);
        let best = SahSplit::new_right(Aap::new_z(0.0), 1.0);

        let mut actual = [EventSide::Both];
        update_geometry_side(&[0], &events, &best, &mut actual);

        let expected = [EventSide::RightOnly];
        assert_eq!(actual, expected);
    }
}
