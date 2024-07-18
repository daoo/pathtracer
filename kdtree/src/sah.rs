use crate::{
    cell::KdCell,
    split::{partition_clipped_geometries, KdPartitioning, KdSplit},
};
use geometry::{aap::Aap, geometry::Geometry};

pub struct SahCost {
    pub traverse_cost: f32,
    pub intersect_cost: f32,
    pub empty_factor: f32,
}

impl SahCost {
    fn calculate(&self, probability: (f32, f32), counts: (usize, usize)) -> f32 {
        debug_assert!((0.0..=1.0).contains(&probability.0) && (0.0..=1.0).contains(&probability.1));
        debug_assert!(probability.0 > 0.0 || probability.1 > 0.0);
        let empty_factor = if counts.0 == 0 || counts.1 == 0 {
            self.empty_factor
        } else {
            1.0
        };
        let intersect_cost = self.intersect_cost
            * (probability.0 * counts.0 as f32 + probability.1 * counts.1 as f32);
        empty_factor * (self.traverse_cost + intersect_cost)
    }

    fn calculate_for_split(&self, split: &KdPartitioning) -> f32 {
        let surface_area = split.parent_aabb.surface_area();
        let probability_left = split.left_aabb.surface_area() / surface_area;
        let probability_right = split.right_aabb.surface_area() / surface_area;
        let probability = (probability_left, probability_right);
        let counts = (
            split.left_indices.len() + split.middle_indices.len(),
            split.right_indices.len() + split.middle_indices.len(),
        );
        self.calculate(probability, counts)
    }
}

impl Default for SahCost {
    fn default() -> Self {
        SahCost {
            traverse_cost: 2.0,
            intersect_cost: 1.0,
            empty_factor: 0.8,
        }
    }
}

fn select_best_split_based_on_cost(
    cost: &SahCost,
    split: KdPartitioning,
) -> Option<(KdSplit, f32)> {
    // TODO: Place planes to the left or to the right depending on what gives best cost.
    if (split.left_aabb.volume() == 0.0 || split.right_aabb.volume() == 0.0)
        && split.middle_indices.is_empty()
    {
        return None;
    }
    let cost = cost.calculate_for_split(&split);
    let mut left_indices = split.left_indices;
    let mut right_indices = split.right_indices;
    left_indices.extend(&split.middle_indices);
    right_indices.extend(split.middle_indices);
    let left = KdCell::new(split.left_aabb, left_indices);
    let right = KdCell::new(split.right_aabb, right_indices);
    let plane = split.plane;
    Some((KdSplit { plane, left, right }, cost))
}

pub(crate) fn find_best_split(
    geometries: &[Geometry],
    cost: &SahCost,
    cell: &KdCell,
) -> Option<KdSplit> {
    debug_assert!(
        !cell.indices.is_empty(),
        "splitting a kd-cell with no geometries only worsens performance"
    );

    let min_by_snd = |a: (_, f32), b: (_, f32)| if a.1 <= b.1 { a } else { b };

    let clipped = cell.clip_geometries(geometries);
    let mut splits = clipped
        .iter()
        .flat_map(|(_, aabb)| aabb.sides())
        .collect::<Vec<_>>();
    splits.sort_unstable_by(Aap::total_cmp);
    splits.dedup();
    splits
        .into_iter()
        .filter_map(|plane| {
            select_best_split_based_on_cost(
                cost,
                partition_clipped_geometries(&clipped, cell.boundary, plane),
            )
        })
        .reduce(min_by_snd)
        .map(|a| a.0)
}

pub(crate) fn should_terminate(cost: &SahCost, cell: &KdCell, split: &KdSplit) -> bool {
    let surface_area = cell.boundary.surface_area();
    let probability_left = split.left.boundary.surface_area() / surface_area;
    let probability_right = split.right.boundary.surface_area() / surface_area;
    let probability = (probability_left, probability_right);
    let counts = (split.left.indices.len(), split.right.indices.len());
    let split_cost = cost.calculate(probability, counts);
    let intersect_cost = cost.intersect_cost * cell.indices.len() as f32;
    split_cost >= intersect_cost
}
