mod block;
mod group;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coord {
    x: i64,
    y: i64,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UCoord {
    x: u32,
    y: u32,
}
