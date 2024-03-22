use crate::geometry::{aap::Axis, algorithms::clip_triangle_aabb, triangle::Triangle};

use super::build::KdBox;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SplitKind {
    Start,
    End,
}

#[derive(Debug, PartialEq)]
pub struct PerfectSplit {
    pub axis: Axis,
    pub kind: SplitKind,
    pub distance: f32,
}

impl PerfectSplit {
    fn new_x(kind: SplitKind, distance: f32) -> Self {
        PerfectSplit {
            axis: Axis::X,
            kind,
            distance,
        }
    }

    fn new_y(kind: SplitKind, distance: f32) -> Self {
        PerfectSplit {
            axis: Axis::Y,
            kind,
            distance,
        }
    }

    fn new_z(kind: SplitKind, distance: f32) -> Self {
        PerfectSplit {
            axis: Axis::Z,
            kind,
            distance,
        }
    }

    fn total_cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.axis
            .cmp(&other.axis)
            .then(self.kind.cmp(&other.kind))
            .then(f32::total_cmp(&self.distance, &other.distance))
    }
}

pub fn perfect_splits(triangles: &[Triangle], parent: &KdBox) -> Vec<PerfectSplit> {
    let mut points = parent
        .triangle_indices
        .iter()
        .flat_map(|i| {
            let triangle = &triangles[*i as usize];
            let clipped = clip_triangle_aabb(&triangle, &parent.boundary);
            if clipped.is_empty() {
                return Vec::new();
            }
            let start = (clipped[0], clipped[0]);
            let (min, max) = clipped
                .iter()
                .fold(start, |(min, max), b| (min.inf(b), max.sup(b)));
            vec![
                PerfectSplit::new_x(SplitKind::Start, min.x),
                PerfectSplit::new_x(SplitKind::End, max.x),
                PerfectSplit::new_y(SplitKind::Start, min.y),
                PerfectSplit::new_y(SplitKind::End, max.y),
                PerfectSplit::new_z(SplitKind::Start, min.z),
                PerfectSplit::new_z(SplitKind::End, max.z),
            ]
        })
        .collect::<Vec<_>>();
    points.sort_unstable_by(PerfectSplit::total_cmp);
    points
}

#[cfg(test)]
mod tests {
    use nalgebra::vector;

    use crate::geometry::aabb::Aabb;

    use super::*;

    #[test]
    fn test() {
        let triangle = Triangle {
            v0: vector![0.0, 0.0, 0.0],
            v1: vector![1.0, 0.0, 0.0],
            v2: vector![1.0, 1.0, 0.0],
        };
        let parent = KdBox {
            boundary: Aabb::from_extents(&vector![0.0, 0.0, 0.0], &vector![2.0, 1.0, 1.0]),
            triangle_indices: vec![0],
        };

        let actual = perfect_splits(&[triangle], &parent);

        let expected = vec![
            PerfectSplit::new_x(SplitKind::Start, 0.0),
            PerfectSplit::new_x(SplitKind::End, 1.0),
            PerfectSplit::new_y(SplitKind::Start, 0.0),
            PerfectSplit::new_y(SplitKind::End, 1.0),
            PerfectSplit::new_z(SplitKind::Start, 0.0),
            PerfectSplit::new_z(SplitKind::End, 0.0),
        ];
        assert_eq!(actual, expected);
    }
}
