use crate::{
    cell::KdCell,
    partitioning::{split_and_partition_clipped_geometries, KdPartitioning},
};
use geometry::{aap::Aap, geometry::Geometry};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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

    fn calculate_for_split(&self, partitioning: &KdPartitioning) -> f32 {
        let surface_area = partitioning.parent_aabb.surface_area();
        let probability_left = partitioning.left_aabb.surface_area() / surface_area;
        let probability_right = partitioning.right_aabb.surface_area() / surface_area;
        let probability = (probability_left, probability_right);
        let counts = (
            partitioning.left_indices.len() + partitioning.middle_indices.len(),
            partitioning.right_indices.len() + partitioning.middle_indices.len(),
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

#[derive(Debug)]
pub(crate) struct KdSplit {
    pub(crate) plane: Aap,
    pub(crate) left: KdCell,
    pub(crate) right: KdCell,
}

fn calculate_cost(cost: &SahCost, partitioning: KdPartitioning) -> Option<(KdSplit, f32)> {
    // TODO: Place planes to the left or to the right depending on what gives best cost.
    if (partitioning.left_aabb.volume() == 0.0 || partitioning.right_aabb.volume() == 0.0)
        && partitioning.middle_indices.is_empty()
    {
        return None;
    }
    let cost = cost.calculate_for_split(&partitioning);
    let mut left_indices = partitioning.left_indices;
    let mut right_indices = partitioning.right_indices;
    left_indices.extend(&partitioning.middle_indices);
    right_indices.extend(partitioning.middle_indices);
    let left = KdCell::new(partitioning.left_aabb, left_indices);
    let right = KdCell::new(partitioning.right_aabb, right_indices);
    let plane = partitioning.plane;
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
    if splits.len() <= 100 {
        splits
            .into_iter()
            .filter_map(|plane| {
                calculate_cost(
                    cost,
                    split_and_partition_clipped_geometries(&clipped, cell.boundary.clone(), plane),
                )
            })
            .reduce(min_by_snd)
            .map(|a| a.0)
    } else {
        splits
            .into_par_iter()
            .filter_map(|plane| {
                calculate_cost(
                    cost,
                    split_and_partition_clipped_geometries(&clipped, cell.boundary.clone(), plane),
                )
            })
            .reduce_with(min_by_snd)
            .map(|a| a.0)
    }
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
