pub mod block;
pub mod group;
pub mod field;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::convert::From<UCoord> for Coord {
    fn from(other: UCoord) -> Self {
        Coord {
            x: other.x as i64,
            y: other.y as i64,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UCoord {
    pub x: u32,
    pub y: u32,
}

impl std::ops::Add for UCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
