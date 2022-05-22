mod block;
mod group;

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
pub struct UCoord {
    x: u32,
    y: u32,
}
