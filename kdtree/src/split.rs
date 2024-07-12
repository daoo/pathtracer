use geometry::{aabb::Aabb, aap::Aap, geometric::Geometric};

use crate::build::KdCell;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub(crate) fn clip_geometries(geometries: &[Geometric], cell: &KdCell) -> Vec<(u32, Aabb)> {
    cell.indices()
        .into_par_iter()
        .filter_map(|i| {
            geometries[*i as usize]
                .clip_aabb(cell.boundary())
                .map(|aabb| (*i, aabb))
        })
        .collect::<Vec<_>>()
}

pub fn partition_triangles(
    clipped_triangles: &[(u32, Aabb)],
    plane: &Aap,
) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut left_triangles: Vec<u32> = Vec::new();
    let mut middle_triangles: Vec<u32> = Vec::new();
    let mut right_triangles: Vec<u32> = Vec::new();
    left_triangles.reserve(clipped_triangles.len());
    right_triangles.reserve(clipped_triangles.len());
    for (index, boundary) in clipped_triangles {
        let planar = boundary.min()[plane.axis] == plane.distance
            && boundary.max()[plane.axis] == plane.distance;
        let left = boundary.min()[plane.axis] < plane.distance;
        let right = boundary.max()[plane.axis] > plane.distance;
        if left {
            left_triangles.push(*index);
        }
        if planar {
            middle_triangles.push(*index);
        }
        if right {
            right_triangles.push(*index);
        }
    }
    (left_triangles, middle_triangles, right_triangles)
}

pub struct SplitPartitioning {
    pub left_aabb: Aabb,
    pub right_aabb: Aabb,
    pub left_indices: Vec<u32>,
    pub middle_indices: Vec<u32>,
    pub right_indices: Vec<u32>,
}

pub fn split_and_partition(clipped: &[(u32, Aabb)], aabb: &Aabb, plane: Aap) -> SplitPartitioning {
    let (left_aabb, right_aabb) = aabb.split(&plane);
    let (left_indices, middle_indices, right_indices) = partition_triangles(clipped, &plane);
    SplitPartitioning {
        left_aabb,
        right_aabb,
        left_indices,
        middle_indices,
        right_indices,
    }
}

#[cfg(test)]
mod partition_triangles_tests {
    use geometry::axis::Axis;
    use glam::Vec3;

    use super::*;

    #[test]
    fn test() {
        let triangle0 = Aabb::from_extents(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
        let triangle1 = Aabb::from_extents(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let triangle2 = Aabb::from_extents(Vec3::new(1.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 0.0));
        let clipped = [(0, triangle0), (1, triangle1), (2, triangle2)];
        let plane = Aap {
            axis: Axis::X,
            distance: 1.0,
        };

        let actual = partition_triangles(clipped.as_slice(), &plane);

        assert_eq!(actual, (vec![0], vec![1], vec![2]));
    }
}
