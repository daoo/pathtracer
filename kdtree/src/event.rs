use geometry::axis::Axis;
use geometry::shape::Shape;
use glam::Vec3;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum EventKind {
    End,
    Planar,
    Start,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub(crate) index: u32,
    pub(crate) distance: f32,
    pub(crate) kind: EventKind,
}

impl Event {
    pub(crate) const fn new_end(index: u32, distance: f32) -> Self {
        Self {
            index,
            distance,
            kind: EventKind::End,
        }
    }

    pub(crate) const fn new_planar(index: u32, distance: f32) -> Self {
        Self {
            index,
            distance,
            kind: EventKind::Planar,
        }
    }

    pub(crate) const fn new_start(index: u32, distance: f32) -> Self {
        Self {
            index,
            distance,
            kind: EventKind::Start,
        }
    }

    pub(crate) fn total_cmp(&self, other: &Self) -> Ordering {
        let cmp_distance = f32::total_cmp(&self.distance, &other.distance);
        let cmp_kind = self.kind.cmp(&other.kind);
        cmp_distance.then(cmp_kind)
    }

    pub(crate) fn le(&self, other: &Self) -> bool {
        self.total_cmp(other).is_le()
    }
}

fn extend_vec_with_events_for_axis(
    vec: &mut Vec<Event>,
    index: u32,
    min: &Vec3,
    max: &Vec3,
    axis: Axis,
) {
    if min[axis] == max[axis] {
        vec.push(Event::new_planar(index, min[axis]));
    } else {
        vec.push(Event::new_start(index, min[axis]));
        vec.push(Event::new_end(index, max[axis]));
    }
}

pub fn extend_vec_with_events(
    events: &mut (Vec<Event>, Vec<Event>, Vec<Event>),
    index: u32,
    min: &Vec3,
    max: &Vec3,
) {
    extend_vec_with_events_for_axis(&mut events.0, index, min, max, Axis::X);
    extend_vec_with_events_for_axis(&mut events.1, index, min, max, Axis::Y);
    extend_vec_with_events_for_axis(&mut events.2, index, min, max, Axis::Z);
}

pub fn generate_event_list(geometries: &[Shape]) -> (Vec<Event>, Vec<Event>, Vec<Event>) {
    let mut events = (
        Vec::with_capacity(geometries.len() * 2),
        Vec::with_capacity(geometries.len() * 2),
        Vec::with_capacity(geometries.len() * 2),
    );
    for (index, geometry) in geometries.iter().enumerate() {
        extend_vec_with_events(&mut events, index as u32, &geometry.min(), &geometry.max());
    }
    events.0.sort_unstable_by(Event::total_cmp);
    events.1.sort_unstable_by(Event::total_cmp);
    events.2.sort_unstable_by(Event::total_cmp);
    events
}

#[cfg(test)]
mod tests {
    use geometry::triangle::Triangle;
    use glam::Vec3;

    use super::*;

    #[test]
    fn generate_event_list_single_triangle() {
        let triangle = Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 1.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 1.0),
        };
        let geometries = [triangle].map(Shape::from);

        let actual = generate_event_list(&geometries);

        let expected = (
            vec![Event::new_start(0, 0.0), Event::new_end(0, 1.0)],
            vec![Event::new_start(0, 0.0), Event::new_end(0, 1.0)],
            vec![Event::new_start(0, 0.0), Event::new_end(0, 1.0)],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn generate_event_list_planar_triangle() {
        let triangle = Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(0.0, 1.0, 0.0),
            v2: Vec3::new(0.0, 1.0, 1.0),
        };
        let geometries = [triangle].map(Shape::from);

        let actual = generate_event_list(&geometries);

        let expected = (
            vec![Event::new_planar(0, 0.0)],
            vec![Event::new_start(0, 0.0), Event::new_end(0, 1.0)],
            vec![Event::new_start(0, 0.0), Event::new_end(0, 1.0)],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn generate_event_list_multiple_overlapping_triangles() {
        let triangle1 = Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 1.0, 0.0),
            v2: Vec3::new(1.0, 1.0, 1.0),
        };
        let triangle2 = Triangle {
            v0: Vec3::new(1.0, 1.0, 1.0),
            v1: Vec3::new(1.0, 2.0, 1.0),
            v2: Vec3::new(1.0, 2.0, 2.0),
        };
        let triangle3 = Triangle {
            v0: Vec3::new(1.0, 1.0, 1.0),
            v1: Vec3::new(2.0, 2.0, 1.0),
            v2: Vec3::new(2.0, 2.0, 2.0),
        };
        let geometries = [triangle1, triangle2, triangle3].map(Shape::from);

        let actual = generate_event_list(&geometries);

        let expected = (
            vec![
                Event::new_start(0, 0.0),
                Event::new_end(0, 1.0),
                Event::new_planar(1, 1.0),
                Event::new_start(2, 1.0),
                Event::new_end(2, 2.0),
            ],
            vec![
                Event::new_start(0, 0.0),
                Event::new_end(0, 1.0),
                Event::new_start(1, 1.0),
                Event::new_start(2, 1.0),
                Event::new_end(1, 2.0),
                Event::new_end(2, 2.0),
            ],
            vec![
                Event::new_start(0, 0.0),
                Event::new_end(0, 1.0),
                Event::new_start(1, 1.0),
                Event::new_start(2, 1.0),
                Event::new_end(1, 2.0),
                Event::new_end(2, 2.0),
            ],
        );
        assert_eq!(actual, expected);
    }
}
