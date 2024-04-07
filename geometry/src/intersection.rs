#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub u: f32,
    pub v: f32,
}

impl Intersection {
    pub fn new(u: f32, v: f32) -> Intersection {
        Intersection { u, v }
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
