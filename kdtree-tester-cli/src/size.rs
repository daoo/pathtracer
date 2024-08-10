use std::{fmt::Display, str::FromStr};

use glam::UVec2;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Size {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl Size {
    pub(crate) fn new(x: u32, y: u32) -> Self {
        Size { x, y }
    }

    pub(crate) fn as_uvec2(self) -> UVec2 {
        UVec2::new(self.x, self.y)
    }
}

impl FromStr for Size {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s.find('x').expect("Could not parse");
        Ok(Size {
            x: s[0..pos].parse()?,
            y: s[pos + 1..].parse()?,
        })
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}
