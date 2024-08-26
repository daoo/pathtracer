use std::cmp::Ordering;

use geometry::{aabb::Aabb, axis::Axis};

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
        f32::total_cmp(&self.distance, &other.distance).then(self.kind.cmp(&other.kind))
    }
}

pub(crate) fn generate_event_list(clipped: &[(u32, Aabb)], axis: Axis) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::with_capacity(clipped.len() * 2);
    for c in clipped {
        if c.1.min()[axis] == c.1.max()[axis] {
            events.push(Event::new_planar(c.1.min()[axis]));
        } else {
            events.push(Event::new_end(c.1.max()[axis]));
            events.push(Event::new_start(c.1.min()[axis]));
        }
    }
    events.sort_unstable_by(Event::total_cmp);
    events
}

#[cfg(test)]
mod tests {
    use crate::event::Event;
    use glam::Vec3;

    use super::*;

    #[test]
    fn generate_event_list_single_triangle() {
        let triangle = Aabb::from_extents(Vec3::ZERO, Vec3::ONE);
        let clipped = [(0, triangle)];

        let actual = generate_event_list(&clipped, Axis::X);

        let expected = vec![Event::new_start(0.0), Event::new_end(1.0)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn generate_event_list_planar_triangle() {
        let triangle = Aabb::from_extents(Vec3::ZERO, Vec3::new(0.0, 1.0, 1.0));
        let clipped = [(0, triangle)];

        let actual = generate_event_list(&clipped, Axis::X);

        let expected = vec![Event::new_planar(0.0)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn generate_event_list_multiple_overlapping_triangles() {
        let triangle1 = Aabb::from_extents(Vec3::ZERO, Vec3::ONE);
        let triangle2 = Aabb::from_extents(Vec3::ONE, Vec3::new(2.0, 2.0, 2.0));
        let triangle3 = Aabb::from_extents(Vec3::ONE, Vec3::new(1.0, 2.0, 2.0));
        let clipped = [(0, triangle1), (1, triangle2), (2, triangle3)];

        let actual = generate_event_list(&clipped, Axis::X);

        let expected = vec![
            Event::new_start(0.0),
            Event::new_end(1.0),
            Event::new_planar(1.0),
            Event::new_start(1.0),
            Event::new_end(2.0),
        ];
        assert_eq!(actual, expected);
    }
}
