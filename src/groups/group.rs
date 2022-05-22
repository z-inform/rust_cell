use super::block::Block;
use super::Coord;
use super::UCoord;

#[derive(Debug)]
struct Group {
    global_coord: Coord,
    block: Block,
}

impl Group {
    pub fn top_right(&self) -> Coord {
        return Coord {
            x: self.global_coord.x + self.block.x_size as i64,
            y: self.global_coord.y + self.block.y_size as i64,
        };
    }

    pub fn intersects(&self, other: &Group) -> bool {
        let self_tr = self.top_right();
        let other_tr = other.top_right();
        if (self.global_coord.x > other_tr.x || self.global_coord.y > other_tr.y)
            && (other.global_coord.x > self_tr.x || other.global_coord.y > self_tr.y)
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
}
