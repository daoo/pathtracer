use geometry::{aabb::Aabb, aap::Aap};

use crate::cell::KdCell;

#[derive(Debug)]
pub(crate) struct KdSplit {
    pub(crate) plane: Aap,
    pub(crate) left: KdCell,
    pub(crate) right: KdCell,
}

fn partition_triangles(
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

pub(crate) struct KdPartitioning {
    pub(crate) plane: Aap,
    pub(crate) parent_aabb: Aabb,
    pub(crate) left_aabb: Aabb,
    pub(crate) right_aabb: Aabb,
    pub(crate) left_indices: Vec<u32>,
    pub(crate) middle_indices: Vec<u32>,
    pub(crate) right_indices: Vec<u32>,
}

pub(crate) fn partition_clipped_geometries(
    clipped: &[(u32, Aabb)],
    parent_aabb: Aabb,
    plane: Aap,
) -> KdPartitioning {
    let (left_aabb, right_aabb) = parent_aabb.split(&plane);
    let (left_indices, middle_indices, right_indices) = partition_triangles(clipped, &plane);
    KdPartitioning {
        plane,
        parent_aabb,
        left_aabb,
        right_aabb,
        left_indices,
        middle_indices,
        right_indices,
    }
}

#[cfg(test)]
mod tests {
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
