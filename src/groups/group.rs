use super::block::Block;
use super::Coord;
use super::UCoord;

#[derive(Debug, Eq)]
struct Group {
    global_coord: Coord,
    block: Block,
}

impl std::cmp::PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.global_coord == other.global_coord && self.block == other.block
    }
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

#[test]
fn group_merge() {
    let block_first = Block {
        x_size: 3,
        y_size: 4,
        data: vec![1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    };
    //0 0 0
    //1 1 0
    //1 1 1
    //1 1 0
    let group_first = Group {
        global_coord: Coord { x: 1, y: 1 },
        block: block_first,
    };

    let block_second = Block {
        x_size: 5,
        y_size: 3,
        data: vec![0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0],
    };
    //0 1 1 1 0
    //1 0 1 0 0
    //0 0 1 1 0

    let group_second = Group {
        global_coord: Coord { x: -3, y: -1 },
        block: block_second,
    };

    let result = group_first.merge(group_second);

    let mut block_check = Block::new(7, 6);
    //0 0 0 0 0 0 0
    //0 0 0 0 1 1 0
    //0 0 0 0 1 1 1
    //0 1 1 1 1 1 0
    //1 0 1 0 0 0 0
    //0 0 1 1 0 0 0

    block_check[(2, 0)] = 1;
    block_check[(3, 0)] = 1;
    block_check[(0, 1)] = 1;
    block_check[(2, 1)] = 1;
    block_check[(1, 2)] = 1;
    block_check[(2, 2)] = 1;
    block_check[(3, 2)] = 1;
    block_check[(4, 2)] = 1;
    block_check[(5, 2)] = 1;
    block_check[(4, 3)] = 1;
    block_check[(5, 3)] = 1;
    block_check[(6, 3)] = 1;
    block_check[(4, 4)] = 1;
    block_check[(5, 4)] = 1;

    let check = Group {
        global_coord: Coord { x: -3, y: -1 },
        block: block_check,
    };

    assert_eq!(result, check);
}
