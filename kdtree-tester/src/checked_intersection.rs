use geometry::{intersection::RayIntersection, ray::Ray};

#[derive(Debug, Clone, Copy)]
pub struct CheckedIntersection {
    pub ray: Ray,
    pub reference: Option<(u32, RayIntersection)>,
    pub kdtree: Option<(u32, RayIntersection)>,
}

impl CheckedIntersection {
    pub fn is_valid(&self) -> bool {
        const T_TOLERANCE: f32 = 0.000001;
        const UV_TOLERANCE: f32 = 0.00001;
        match (self.reference, self.kdtree) {
            (None, None) => true,
            (Some((t1, i1)), Some((t2, i2))) => {
                t1 == t2
                    && (i1.t - i2.t).abs() < T_TOLERANCE
                    && (i1.u - i2.u).abs() < UV_TOLERANCE
                    && (i1.v - i2.v).abs() < UV_TOLERANCE
            }
            _ => false,
        }
    }

    pub fn as_bytes(&self, iteration: u16) -> [u8; 50] {
        let mut bytes = [0u8; 50];
        let ray = if let Some(kdtree) = self.kdtree {
            self.ray.extended(kdtree.1.t)
        } else if let Some(reference) = self.reference {
            self.ray.extended(reference.1.t)
        } else {
            self.ray
        };
        let correct_point = self.ray.param(self.reference.unwrap().1.t);
        let actual_point = if let Some(kdtree) = self.kdtree {
            self.ray.param(kdtree.1.t)
        } else {
            [0.0, 0.0, 0.0].into()
        };
        bytes[0..2].copy_from_slice(&iteration.to_le_bytes());
        bytes[2..6].copy_from_slice(&ray.origin.x.to_le_bytes());
        bytes[6..10].copy_from_slice(&ray.origin.y.to_le_bytes());
        bytes[10..14].copy_from_slice(&ray.origin.z.to_le_bytes());
        bytes[14..18].copy_from_slice(&ray.direction.x.to_le_bytes());
        bytes[18..22].copy_from_slice(&ray.direction.y.to_le_bytes());
        bytes[22..26].copy_from_slice(&ray.direction.z.to_le_bytes());
        bytes[26..30].copy_from_slice(&correct_point.x.to_le_bytes());
        bytes[30..34].copy_from_slice(&correct_point.y.to_le_bytes());
        bytes[34..38].copy_from_slice(&correct_point.z.to_le_bytes());
        bytes[38..42].copy_from_slice(&actual_point.x.to_le_bytes());
        bytes[42..46].copy_from_slice(&actual_point.y.to_le_bytes());
        bytes[46..50].copy_from_slice(&actual_point.z.to_le_bytes());
        bytes
    }
}
