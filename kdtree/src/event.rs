use geometry::{aabb::Aabb, axis::Axis};
use glam::Vec3;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum EventKind {
    End,
    Planar,
    Start,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Event {
    pub(crate) distance: f32,
    pub(crate) kind: EventKind,
}

impl Event {
    fn new_end(distance: f32) -> Self {
        Self {
            distance,
            kind: EventKind::End,
        }
    }

    fn new_planar(distance: f32) -> Self {
        Self {
            distance,
            kind: EventKind::Planar,
        }
    }

    fn new_start(distance: f32) -> Self {
        Self {
            distance,
            kind: EventKind::Start,
        }
    }

    fn total_cmp(&self, other: &Self) -> Ordering {
        let cmp_distance = f32::total_cmp(&self.distance, &other.distance);
        let cmp_kind = self.kind.cmp(&other.kind);
        cmp_distance.then(cmp_kind)
    }
}

fn extend_vec_with_events(vec: &mut Vec<Event>, min: &Vec3, max: &Vec3, axis: Axis) {
    if min[axis] == max[axis] {
        vec.push(Event::new_planar(min[axis]));
    } else {
        vec.push(Event::new_start(min[axis]));
        vec.push(Event::new_end(max[axis]));
    }
}

pub(crate) fn generate_event_list(clipped: &[(u32, Aabb)]) -> [(Axis, Vec<Event>); 3] {
    let mut events = [
        (Axis::X, Vec::with_capacity(clipped.len() * 2)),
        (Axis::Y, Vec::with_capacity(clipped.len() * 2)),
        (Axis::Z, Vec::with_capacity(clipped.len() * 2)),
    ];
    for (_, boundary) in clipped {
        let (min, max) = (&boundary.min(), &boundary.max());
        extend_vec_with_events(&mut events[0].1, min, max, Axis::X);
        extend_vec_with_events(&mut events[1].1, min, max, Axis::Y);
        extend_vec_with_events(&mut events[2].1, min, max, Axis::Z);
    }
    events[0].1.sort_unstable_by(Event::total_cmp);
    events[1].1.sort_unstable_by(Event::total_cmp);
    events[2].1.sort_unstable_by(Event::total_cmp);
    events
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;

    #[test]
    fn generate_event_list_single_triangle() {
        let triangle = Aabb::from_extents(Vec3::ZERO, Vec3::ONE);
        let clipped = [(0, triangle)];

        let actual = generate_event_list(&clipped);

        let expected = [
            (Axis::X, vec![Event::new_start(0.0), Event::new_end(1.0)]),
            (Axis::Y, vec![Event::new_start(0.0), Event::new_end(1.0)]),
            (Axis::Z, vec![Event::new_start(0.0), Event::new_end(1.0)]),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn generate_event_list_planar_triangle() {
        let triangle = Aabb::from_extents(Vec3::ZERO, Vec3::new(0.0, 1.0, 1.0));
        let clipped = [(0, triangle)];

        let actual = generate_event_list(&clipped);

        let expected = [
            (Axis::X, vec![Event::new_planar(0.0)]),
            (Axis::Y, vec![Event::new_start(0.0), Event::new_end(1.0)]),
            (Axis::Z, vec![Event::new_start(0.0), Event::new_end(1.0)]),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn generate_event_list_multiple_overlapping_triangles() {
        let triangle1 = Aabb::from_extents(Vec3::ZERO, Vec3::ONE);
        let triangle2 = Aabb::from_extents(Vec3::ONE, Vec3::new(2.0, 2.0, 2.0));
        let triangle3 = Aabb::from_extents(Vec3::ONE, Vec3::new(1.0, 2.0, 2.0));
        let clipped = [(0, triangle1), (1, triangle2), (2, triangle3)];

        let actual = generate_event_list(&clipped);

        let expected = [
            (
                Axis::X,
                vec![
                    Event::new_start(0.0),
                    Event::new_end(1.0),
                    Event::new_planar(1.0),
                    Event::new_start(1.0),
                    Event::new_end(2.0),
                ],
            ),
            (
                Axis::Y,
                vec![
                    Event::new_start(0.0),
                    Event::new_end(1.0),
                    Event::new_start(1.0),
                    Event::new_start(1.0),
                    Event::new_end(2.0),
                    Event::new_end(2.0),
                ],
            ),
            (
                Axis::Z,
                vec![
                    Event::new_start(0.0),
                    Event::new_end(1.0),
                    Event::new_start(1.0),
                    Event::new_start(1.0),
                    Event::new_end(2.0),
                    Event::new_end(2.0),
                ],
            ),
        ];
        assert_eq!(actual, expected);
    }
}
