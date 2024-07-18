#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointIntersection {
    pub u: f32,
    pub v: f32,
}

impl PointIntersection {
    pub fn new(u: f32, v: f32) -> PointIntersection {
        PointIntersection { u, v }
    }

    pub fn with_ray_param(&self, t: f32) -> RayIntersection {
        RayIntersection {
            t,
            u: self.u,
            v: self.v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayIntersection {
    pub t: f32,
    pub u: f32,
    pub v: f32,
}

impl RayIntersection {
    pub fn new(t: f32, u: f32, v: f32) -> Self {
        RayIntersection { t, u, v }
    }
}
