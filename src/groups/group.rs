use super::block::Block;
use super::Coord;
use super::UCoord;
use svg::node::element::Rectangle;
use rstar::{RTreeObject, AABB};

#[derive(Debug, Eq)]
pub struct Group {
    pub global_coord: Coord,
    pub block: Block,
}

impl std::cmp::PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.global_coord == other.global_coord && self.block == other.block
    }
}

impl std::cmp::PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.global_coord.partial_cmp(&other.global_coord)
    }
}

impl std::cmp::Ord for Group {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.global_coord.cmp(&other.global_coord)
    }
}

impl RTreeObject for Group {
    type Envelope = AABB<(i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.global_coord.into(), self.top_right().into())
    }
}

impl Group {
    pub fn top_right(&self) -> Coord {
        return Coord {
            x: self.global_coord.x + self.block.x_size as i64 - 1,
            y: self.global_coord.y + self.block.y_size as i64 - 1,
        };
    }

    pub fn intersects(&self, other: &Group) -> bool {
        let self_tr = self.top_right();
        let other_tr = other.top_right();
        if (self.global_coord.x > other_tr.x || self.global_coord.y > other_tr.y)
            || (other.global_coord.x > self_tr.x || other.global_coord.y > self_tr.y)
        {
            return false;
        } else {
            return true;
        }
    }

    pub fn merge(self, other: Group) -> Group {
        let left_bottom = Coord {
            x: std::cmp::min(self.global_coord.x, other.global_coord.x),
            y: std::cmp::min(self.global_coord.y, other.global_coord.y),
        };

        let self_offset = UCoord {
            x: (self.global_coord.x - left_bottom.x) as u32,
            y: (self.global_coord.y - left_bottom.y) as u32,
        };

        let other_offset = UCoord {
            x: (other.global_coord.x - left_bottom.x) as u32,
            y: (other.global_coord.y - left_bottom.y) as u32,
        };

        let mut new_block = Block::new(1, 1);
        new_block.insert(self_offset, &self.block);
        new_block.insert(other_offset, &other.block);

        let new = Group {
            global_coord: left_bottom,
            block: new_block,
        };
        new
    }

    pub fn split(self) -> Option<Vec<Group>> {
        let blocks = match self.block.split() {
            None => return None,
            Some(i) => i,
        };

        let mut groups = Vec::new();
        for i in blocks {
            let piece = Group {
                global_coord: i.1 + self.global_coord,
                block: i.0,
            };
            groups.push(piece);
        }
        Some(groups)
    }

    pub fn step(mut self) -> Option<Vec<Group>> {
        self.block.step();
        self.split()
    }

    pub fn svg_add(&self, mut doc: svg::Document) -> svg::Document {
        let size = 10;
        let group_rect = Rectangle::new()
            .set("x", self.global_coord.x as i64 * size)
            .set("y", self.global_coord.y as i64 * size)
            .set("width", self.block.x_size as i64 * size)
            .set("height", self.block.y_size as i64 * size)
            .set("stroke", "red")
            .set("stroke-width", 0.3)
            .set("fill", "black")
            .set("fill-opacity", "0.01");
        for x in 0..self.block.x_size {
            for y in 0..self.block.y_size {
                if self.block[(x, y)] == 1 {
                    let mut cell = Rectangle::new();
                    cell = cell
                        .set("x", (self.global_coord.x + x as i64) * size)
                        .set("y", (self.global_coord.y + y as i64) * size)
                        .set("width", size)
                        .set("height", size)
                        .set("fill", "green")
                        .set("stroke", "black")
                        .set("stroke-width", 0.3);
                    doc = doc.add(cell);
                }
            }
        }
        doc.add(group_rect)
    }

    pub fn new(global_coord: Coord, block: Block) -> Self {
        let group = Group {
            global_coord,
            block,
        };
        group
    }

    pub fn reverse_y(&mut self) {
        let mut rev_block = Block::new(self.block.x_size, self.block.y_size);
        for x in 0..self.block.x_size {
            for y in 0..self.block.y_size {
                rev_block[(x, y)] = self.block[(x, self.block.y_size - y - 1)];
            }
        }
        self.block = rev_block;
        self.global_coord = Coord {
            x: self.global_coord.x,
            y: -self.global_coord.y - self.block.y_size as i64 + 1,
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn group_merge() {
        let mut block_first = Block::new(6, 6);
        block_first[(1, 2)] = 1;
        block_first[(1, 3)] = 1;
        block_first[(1, 4)] = 1;
        block_first[(2, 1)] = 1;
        block_first[(3, 1)] = 1;
        block_first[(3, 4)] = 1;
        block_first[(4, 2)] = 1;
        block_first[(4, 3)] = 1;
        //0 0 0 0 0 0
        //0 1 0 1 0 0
        //0 1 0 0 1 0
        //0 1 0 0 1 0
        //0 0 1 1 0 0
        //0 0 0 0 0 0
        let group_first = Group {
            global_coord: Coord { x: 1, y: 1 },
            block: block_first,
        };

        let mut block_second = Block::new(6, 6);
        block_second[(1, 2)] = 1;
        block_second[(1, 3)] = 1;
        block_second[(2, 4)] = 1;
        block_second[(2, 1)] = 1;
        block_second[(3, 4)] = 1;
        block_second[(4, 1)] = 1;
        block_second[(4, 2)] = 1;
        //0 0 0 0 0 0
        //0 0 1 1 0 0
        //0 1 0 0 0 0
        //0 1 0 0 1 0
        //0 0 1 0 1 0
        //0 0 0 0 0 0

        let group_second = Group {
            global_coord: Coord { x: 5, y: 1 },
            block: block_second,
        };

        let result = group_first.merge(group_second);

        let mut block_check = Block::new(10, 6);
        //0 0 0 0 0 0 0 0 0 0
        //0 1 0 1 0 0 1 1 0 0
        //0 1 0 0 1 1 0 0 0 0
        //0 1 0 0 1 1 0 0 1 0
        //0 0 1 1 0 0 1 0 1 0
        //0 0 0 0 0 0 0 0 0 0

        block_check[(1, 2)] = 1;
        block_check[(1, 3)] = 1;
        block_check[(1, 4)] = 1;
        block_check[(2, 1)] = 1;
        block_check[(3, 1)] = 1;
        block_check[(3, 4)] = 1;
        block_check[(4, 2)] = 1;
        block_check[(4, 3)] = 1;
        block_check[(5, 2)] = 1;
        block_check[(5, 3)] = 1;
        block_check[(6, 1)] = 1;
        block_check[(6, 4)] = 1;
        block_check[(7, 4)] = 1;
        block_check[(8, 1)] = 1;
        block_check[(8, 2)] = 1;

        let check = Group {
            global_coord: Coord { x: 1, y: 1 },
            block: block_check,
        };
        assert_eq!(result, check);
    }

    #[test]
    fn group_top_right() {
        let mut block = Block::new(5, 5);
        //0 0 0 0 0
        //0 0 1 0 0
        //0 0 0 1 0
        //0 1 1 1 0
        //0 0 0 0 0
        block[(1, 1)] = 1;
        block[(2, 1)] = 1;
        block[(3, 1)] = 1;
        block[(3, 2)] = 1;
        block[(2, 3)] = 1;

        let group = Group {
            global_coord: Coord { x: 5, y: -10 },
            block,
        };

        assert_eq!(group.top_right(), Coord { x: 9, y: -6 });
    }

    #[test]
    fn group_split() {
        let mut block = Block::new(1, 1);
        let mut b1 = Block::new(5, 6);
        //0 0 0 0 0
        //0 0 1 0 0
        //0 1 0 1 0
        //0 1 0 1 0
        //0 0 1 0 0
        //0 0 0 0 0
        b1[(1, 2)] = 1;
        b1[(1, 3)] = 1;
        b1[(2, 1)] = 1;
        b1[(2, 4)] = 1;
        b1[(3, 2)] = 1;
        b1[(3, 3)] = 1;

        let mut b2 = Block::new(5, 5);
        //0 0 0 0 0
        //0 0 1 0 0
        //0 1 0 1 0
        //0 0 1 1 0
        //0 0 0 0 0
        b2[(1, 2)] = 1;
        b2[(2, 1)] = 1;
        b2[(2, 3)] = 1;
        b2[(3, 1)] = 1;
        b2[(3, 2)] = 1;

        block.insert(UCoord { x: 0, y: 16 }, &b1);
        block.insert(UCoord { x: 3, y: 0 }, &b2);
        let group = Group {
            global_coord: Coord { x: 0, y: 0 },
            block,
        };
        let g1 = Group {
            global_coord: Coord {x: 0, y: 16},
            block: b1,
        };
        let g2 = Group {
            global_coord: Coord {x: 3, y: 0},
            block: b2,
        };
        let new = group.split().unwrap();
        assert_eq!(new[1], g2);
        assert_eq!(new[0], g1);
        assert_eq!(new.len(), 2);
    }
}
