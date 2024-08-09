use geometry::{aabb::Aabb, aap::Aap};

fn partition_clipped_geometries(
    clipped: &[(u32, Aabb)],
    plane: &Aap,
) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut left_indices: Vec<u32> = Vec::new();
    let mut middle_indices: Vec<u32> = Vec::new();
    let mut right_indices: Vec<u32> = Vec::new();
    left_indices.reserve(clipped.len());
    right_indices.reserve(clipped.len());
    for (index, boundary) in clipped {
        let planar = boundary.min()[plane.axis] == plane.distance
            && boundary.max()[plane.axis] == plane.distance;
        let left = boundary.min()[plane.axis] < plane.distance;
        let right = boundary.max()[plane.axis] > plane.distance;
        if left {
            left_indices.push(*index);
        }
        if planar {
            middle_indices.push(*index);
        }
        if right {
            right_indices.push(*index);
        }
    }
    (left_indices, middle_indices, right_indices)
}

#[derive(Debug)]
pub(crate) struct KdPartitioning {
    pub(crate) plane: Aap,
    pub(crate) parent_aabb: Aabb,
    pub(crate) left_aabb: Aabb,
    pub(crate) right_aabb: Aabb,
    pub(crate) left_indices: Vec<u32>,
    pub(crate) middle_indices: Vec<u32>,
    pub(crate) right_indices: Vec<u32>,
}

pub(crate) fn split_and_partition_clipped_geometries(
    clipped: &[(u32, Aabb)],
    parent_aabb: Aabb,
    plane: Aap,
) -> KdPartitioning {
    let (left_aabb, right_aabb) = parent_aabb.split(&plane);
    let (left_indices, middle_indices, right_indices) =
        partition_clipped_geometries(clipped, &plane);
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

        let actual = partition_clipped_geometries(clipped.as_slice(), &plane);

        assert_eq!(actual, (vec![0], vec![1], vec![2]));
    }
}
