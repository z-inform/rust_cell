use super::*;

#[derive(Eq)]
#[derive(Debug)]
pub struct Block {
    pub x_size: u32,
    pub y_size: u32,
    pub data: std::vec::Vec<u8>,
}

impl std::ops::Index<UCoord> for Block {
    type Output = u8;

    fn index(&self, index: UCoord) -> &Self::Output {
        let linear_coord = index.y * self.x_size + index.x;
        self.data.get(linear_coord as usize).unwrap()
    }
}

impl std::ops::Index<(u32, u32)> for Block {
    type Output = u8;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let linear_coord = index.1 * self.x_size + index.0;
        self.data.get(linear_coord as usize).unwrap()
    }
}

impl std::ops::IndexMut<UCoord> for Block {
    fn index_mut(&mut self, index: UCoord) -> &mut Self::Output {
        let linear_coord = index.y * self.x_size + index.x;
        self.data.get_mut(linear_coord as usize).unwrap()
    }
}

impl std::ops::IndexMut<(u32, u32)> for Block {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        let linear_coord = index.1 * self.x_size + index.0;
        self.data.get_mut(linear_coord as usize).unwrap()
    }
}

impl std::cmp::PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.x_size == other.x_size && self.y_size == other.y_size && self.data == other.data        
    }
}

impl Block {

    pub fn get(&self, index: UCoord) -> Option<&u8> {
        let linear_coord = index.y * self.x_size + index.x;
        self.data.get(linear_coord as usize)
    }

    pub fn dump_data(&self) {
        for y in self.y_size..0 {
            for x in 0..self.x_size {
                print!("{} ", self[(x, y)]);
            }
        }
    }

    pub fn neighbour_count(&self, index: UCoord) -> u8 {
        let mut count: u8 = 0;
        let y_start: i8 = if index.y == 0 {0} else {-1};
        let y_end: i8 = if index.y == (self.y_size - 1) {0} else {1};

        if index.x > 0 {
            for y in y_start..=y_end {
                count += self[(index.x - 1, (index.y as i8 + y) as u32)];
            }
        }

        if index.x < (self.x_size - 1) {
            for y in y_start..=y_end {
                count += self[(index.x + 1, (index.y as i8 + y) as u32)];
            }
        }

        if y_start == -1 {
                count += self[(index.x, index.y - 1)];
        }

        if y_end == 1 {
                count += self[(index.x, index.y + 1)];
        }

        count 
    }

    pub fn block_step(&mut self) {
        let length = self.x_size * self.y_size;
        let mut new_block = Block {data : std::vec::Vec::with_capacity(length as usize), ..*self};
        for _i in 0..length {
            new_block.data.push(0);
        }

        for x in 0..self.x_size {
            for y in 0..self.y_size {
                let coord = UCoord {x : x, y : y};
                match self.neighbour_count(coord) {
                    2 => new_block[coord] = self[coord],
                    3 => new_block[coord] = 1,
                    _ => new_block[coord] = 0
                }
            }
        }
        *self = new_block;
    }

    fn row_alive(&self, index_y : u32) -> u32 {
        let mut count: u32 = 0;
        for x in 0..self.x_size {
            count += self[(x, index_y)] as u32;
        }
        count
    }

    fn column_alive(&self, index_x : u32) -> u32 {
        let mut count: u32 = 0;
        for y in 0..self.y_size {
            count += self[(index_x, y)] as u32;
        }
        count
    }

    pub fn need_expand(&self) -> bool {
        if self.column_alive(0) > 0 || self.column_alive(self.x_size - 1) > 0 {
            return true
        }

        if self.row_alive(0) > 0 || self.row_alive(self.y_size - 1) > 0 {
            return true
        }

        return false
    }
}

#[test]
fn block_indexing() {
    let block = Block {x_size : 2, y_size : 2, data : vec![0, 1, 1, 0]};
    assert_eq!(block[(0, 0)], 0);
    assert_eq!(block[(0, 1)], 1);
    assert_eq!(block[(1, 0)], 1);
    assert_eq!(block[(1, 1)], 0);
}

#[test]
fn block_mutability() {
    let mut block = Block {x_size : 2, y_size : 2, data : vec![0, 1, 1, 0]};
    block[(0, 1)] = 1;
    assert_eq!(block[(0, 1)], 1);
}

#[test]
fn count_neighbours() {
    let block = Block {x_size : 3, y_size : 3, data : vec![1, 0, 1, 0, 0, 1, 1, 1, 0]};
    //1 1 0
    //0 0 1
    //1 0 1

    assert_eq!(block.neighbour_count(UCoord {x : 0, y : 0}), 0, "Coord 0;0");
    assert_eq!(block.neighbour_count(UCoord {x : 1, y : 0}), 3, "Coord 1;0");
    assert_eq!(block.neighbour_count(UCoord {x : 2, y : 0}), 1, "Coord 2;0");
    assert_eq!(block.neighbour_count(UCoord {x : 0, y : 1}), 3, "Coord 0;1");
    assert_eq!(block.neighbour_count(UCoord {x : 1, y : 1}), 5, "Coord 2;1");
    assert_eq!(block.neighbour_count(UCoord {x : 2, y : 1}), 2, "Coord 3;1");
    assert_eq!(block.neighbour_count(UCoord {x : 0, y : 2}), 1, "Coord 0;2");
    assert_eq!(block.neighbour_count(UCoord {x : 1, y : 2}), 2, "Coord 1;2");
    assert_eq!(block.neighbour_count(UCoord {x : 2, y : 2}), 2, "Coord 2;2");

}

#[test]
fn block_step() {
    let mut block = Block {x_size : 3, y_size : 3, data : vec![1, 0, 1, 0, 0, 1, 1, 1, 0]};
    //1 1 0
    //0 0 1
    //1 0 1
    assert_eq!(block.neighbour_count(UCoord {x : 1, y : 0}), 3);
    block.block_step();
    let next_block = Block {x_size : 3, y_size : 3, data : vec![0, 1, 0, 1, 0, 1, 0, 1, 0]};
    assert_eq!(block, next_block);
}

#[test]
fn line_count() {
    let block = Block {x_size : 3, y_size : 3, data : vec![1, 0, 1, 1, 0, 1, 1, 1, 0]};
    //1 1 0
    //1 0 1
    //1 0 1
    assert_eq!(block.row_alive(0), 2);
    assert_eq!(block.row_alive(1), 2);
    assert_eq!(block.row_alive(2), 2);

    assert_eq!(block.column_alive(0), 3);
    assert_eq!(block.column_alive(1), 1);
    assert_eq!(block.column_alive(2), 2);

}

#[test]
fn need_expand() {
    let block = Block {x_size : 3, y_size : 3, data : vec![1, 0, 1, 1, 0, 1, 1, 1, 0]};
    //1 1 0
    //1 0 1
    //1 0 1
    assert_eq!(block.need_expand(), true);

    let block_no_expand = Block {x_size : 3, y_size : 3, data : vec![0, 0, 0, 0, 1, 0, 0, 0, 0]};
    assert_eq!(block_no_expand.need_expand(), false)
}
