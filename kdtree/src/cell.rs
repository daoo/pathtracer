use crate::event::{Event, generate_event_list};
use geometry::{aabb::Aabb, bound::geometries_bounding_box, geometry::Geometry};

#[derive(Debug)]
pub struct KdCell {
    pub(crate) boundary: Aabb,
    pub(crate) indices: Vec<u32>,
    pub(crate) events: (Vec<Event>, Vec<Event>, Vec<Event>),
}

impl KdCell {
    pub(crate) fn new(
        boundary: Aabb,
        indices: Vec<u32>,
        events: (Vec<Event>, Vec<Event>, Vec<Event>),
    ) -> Self {
        debug_assert!(
            boundary.surface_area() != 0.0,
            "empty kd-cell cannot intersect a ray"
        );
        debug_assert!(
            !(boundary.volume() == 0.0 && indices.is_empty()),
            "flat kd-cell without any triangles likely worsens performance"
        );
        Self {
            boundary,
            indices,
            events,
        }
    }

    pub(crate) fn generate_initial(geometries: &[impl Geometry]) -> Self {
        Self::new(
            geometries_bounding_box(geometries),
            (0u32..geometries.len() as u32).collect(),
            generate_event_list(geometries),
        )
    }
}
