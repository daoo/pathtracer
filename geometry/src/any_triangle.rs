use crate::{axial_triangle::AxiallyAlignedTriangle, triangle::Triangle};

#[derive(Clone, Debug, PartialEq)]
pub enum AnyTriangle {
    Triangle(Triangle),
    AxiallyAlignedTriangle(AxiallyAlignedTriangle),
}

impl From<Triangle> for AnyTriangle {
    fn from(triangle: Triangle) -> Self {
        triangle
            .as_axially_aligned()
            .map(AnyTriangle::AxiallyAlignedTriangle)
            .unwrap_or_else(|| AnyTriangle::Triangle(triangle))
    }
}
