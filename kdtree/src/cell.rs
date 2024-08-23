use geometry::{aabb::Aabb, bound::geometries_bounding_box, geometry::Geometry};

#[derive(Debug)]
pub(crate) struct KdCell {
    pub(crate) boundary: Aabb,
    pub(crate) indices: Vec<u32>,
}

impl KdCell {
    pub(crate) fn new(boundary: Aabb, indices: Vec<u32>) -> Self {
        debug_assert!(
            boundary.surface_area() != 0.0,
            "empty kd-cell cannot intersect a ray"
        );
        debug_assert!(
            !(boundary.volume() == 0.0 && indices.is_empty()),
            "flat kd-cell without any triangles likely worsens performance"
        );
        KdCell { boundary, indices }
    }

    pub(crate) fn clip_geometries(&self, geometries: &[Geometry]) -> Vec<(u32, Aabb)> {
        self.indices
            .iter()
            .filter_map(|i| {
                geometries[*i as usize]
                    .clip_aabb(&self.boundary)
                    .map(|aabb| (*i, aabb))
            })
            .collect::<Vec<_>>()
    }

    pub(crate) fn generate_initial(geometries: &[Geometry]) -> Self {
        KdCell::new(
            geometries_bounding_box(geometries),
            (0u32..geometries.len() as u32).collect(),
        )
    }
}
