mod block;
mod group;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UCoord {
    x: u32,
    y: u32,
}
