use crate::geometry::{
    aabb::Aabb,
    aap::{Aap, Axis},
    algorithms::intersect_triangle_aabb,
    triangle::Triangle,
};

#[derive(Debug)]
pub struct KdBox {
    pub boundary: Aabb,
    pub triangle_indices: Vec<u32>,
}

#[derive(Debug)]
pub struct KdSplit {
    pub plane: Aap,
    pub left: KdBox,
    pub right: KdBox,
}

pub struct KdTreeInputs {
    pub max_depth: u32,
    pub triangles: Vec<Triangle>,
}

impl KdTreeInputs {
    pub fn potential_split_points(&self, parent: &KdBox, axis: Axis) -> Vec<f32> {
        let min = parent.boundary.min()[axis] + 0.1;
        let max = parent.boundary.max()[axis] - 0.1;
        let mut points = parent
            .triangle_indices
            .iter()
            .flat_map(|i| {
                let triangle = &self.triangles[*i as usize];
                [triangle.min()[axis] - 0.1, triangle.max()[axis] + 0.1]
            })
            .filter(|p| p > &min && p < &max)
            .collect::<Vec<_>>();
        points.sort_by(f32::total_cmp);
        points
    }

    pub fn split_box(&self, parent: &KdBox, plane: Aap) -> KdSplit {
        let (left_aabb, right_aabb) = parent.boundary.split(&plane);
        debug_assert!(
            left_aabb.size()[plane.axis] > 0.1,
            "left_aabb too small {:?} {:?}",
            plane,
            left_aabb
        );
        debug_assert!(
            right_aabb.size()[plane.axis] > 0.1,
            "right_aabb to small {:?} {:?}",
            plane,
            right_aabb
        );
        let mut left_triangle_indices: Vec<u32> = Vec::new();
        let mut right_triangle_indices: Vec<u32> = Vec::new();
        for triangle_index in &parent.triangle_indices {
            let triangle = &self.triangles[*triangle_index as usize];
            let in_left = intersect_triangle_aabb(triangle, &left_aabb);
            let in_right = intersect_triangle_aabb(triangle, &right_aabb);
            debug_assert!(in_left || in_right);
            if in_left {
                left_triangle_indices.push(*triangle_index);
            }
            if in_right {
                right_triangle_indices.push(*triangle_index);
            }
        }
        KdSplit {
            plane,
            left: KdBox {
                boundary: left_aabb,
                triangle_indices: left_triangle_indices,
            },
            right: KdBox {
                boundary: right_aabb,
                triangle_indices: right_triangle_indices,
            },
        }
    }
}
