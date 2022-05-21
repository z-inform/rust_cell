use super::*;

#[derive(Eq)]
#[derive(Debug)]
pub struct Block {
    x_size: u32,
    y_size: u32,
    data: std::vec::Vec<u8>,
}

impl std::ops::Index<UCoord> for Block {
    type Output = u8;

    fn index(&self, index: UCoord) -> &Self::Output {
        let linear_coord = index.x * self.x_size + index.y;
        self.data.get(linear_coord as usize).unwrap()
    }
}

impl std::ops::Index<(u32, u32)> for Block {
    type Output = u8;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let linear_coord = index.0 * self.x_size + index.1;
        self.data.get(linear_coord as usize).unwrap()
    }
}

impl std::ops::IndexMut<UCoord> for Block {
    fn index_mut(&mut self, index: UCoord) -> &mut Self::Output {
        let linear_coord = index.x * self.x_size + index.y;
        self.data.get_mut(linear_coord as usize).unwrap()
    }
}

impl std::ops::IndexMut<(u32, u32)> for Block {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        let linear_coord = index.0 * self.x_size + index.1;
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
        let linear_coord = index.x * self.x_size + index.y;
        self.data.get(linear_coord as usize)
    }

    pub fn dump_data(&self) {
        for y in self.y_size..0 {
            for x in 0..self.x_size {
                print!("{} ", self[(x, y)]);
            }
        }
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
