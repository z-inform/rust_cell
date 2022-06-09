use super::*;

#[derive(Eq)]
pub struct Block {
    pub x_size: u32,
    pub y_size: u32,
    pub data: std::vec::Vec<u8>,
}

impl std::ops::Index<UCoord> for Block {
    type Output = u8;

    fn index(&self, index: UCoord) -> &Self::Output {
        let linear_coord = index.y * self.x_size + index.x;
        self.data
            .get(linear_coord as usize)
            .expect("Block indexed on not existing cell")
    }
}

impl std::ops::Index<(u32, u32)> for Block {
    type Output = u8;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let linear_coord = index.1 * self.x_size + index.0;
        self.data
            .get(linear_coord as usize)
            .expect("Block indexed on not existing cell")
    }
}

impl std::ops::IndexMut<UCoord> for Block {
    fn index_mut(&mut self, index: UCoord) -> &mut Self::Output {
        let linear_coord = index.y * self.x_size + index.x;
        self.data
            .get_mut(linear_coord as usize)
            .expect("Block indexed on not existing cell")
    }
}

impl std::ops::IndexMut<(u32, u32)> for Block {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        let linear_coord = index.1 * self.x_size + index.0;
        self.data
            .get_mut(linear_coord as usize)
            .expect("Block indexed on not existing cell")
    }
}

impl std::cmp::PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.x_size == other.x_size && self.y_size == other.y_size && self.data == other.data
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nSize [{} {}]\nData: \n ", self.x_size, self.y_size)?;
        for _x in 0..self.x_size {
            write!(f, " -")?;
        }
        write!(f, "\n")?;
        for y in 1..=self.y_size {
            write!(f, "| ")?;
            for x in 0..self.x_size {
                write!(f, "{} ", self[(x, self.y_size - y)])?;
            }
            write!(f, "|\n")?;
        }
        write!(f, " ")?;
        for _x in 0..self.x_size {
            write!(f, " -")?;
        }
        write!(f, "")
    }
}

impl Block {
    pub fn new(x_size: u32, y_size: u32) -> Block {
        let linear_size = x_size * y_size;
        let mut new_block = Block {
            x_size: x_size,
            y_size: y_size,
            data: std::vec::Vec::with_capacity(linear_size as usize),
        };
        new_block.data.resize(linear_size as usize, 0);
        new_block
    }

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
        let y_start: i8 = if index.y == 0 { 0 } else { -1 };
        let y_end: i8 = if index.y == (self.y_size - 1) { 0 } else { 1 };

        if index.x > 0 {
            if y_start == -1 {
                count += self[(index.x - 1, index.y - 1)];
            }
            if y_end == 1 {
                count += self[(index.x - 1, index.y + 1)];
            }
            count += self[(index.x - 1, index.y)];
        }

        if index.x < (self.x_size - 1) {
            if y_start == -1 {
                count += self[(index.x + 1, index.y - 1)];
            }
            if y_end == 1 {
                count += self[(index.x + 1, index.y + 1)];
            }
            count += self[(index.x + 1, index.y)];
        }

        if y_start == -1 {
            count += self[(index.x, index.y - 1)];
        }

        if y_end == 1 {
            count += self[(index.x, index.y + 1)];
        }

        count
    }

    pub fn step(&mut self) {
        let length = self.x_size * self.y_size;
        let mut new_block = Block {
            data: std::vec::Vec::with_capacity(length as usize),
            ..*self
        };
        for _i in 0..length {
            new_block.data.push(0);
        }

        for x in 0..self.x_size {
            for y in 0..self.y_size {
                let coord = UCoord { x: x, y: y };
                match self.neighbour_count(coord) {
                    2 => new_block[coord] = self[coord],
                    3 => new_block[coord] = 1,
                    _ => new_block[coord] = 0,
                }
            }
        }
        *self = new_block;
    }

    fn row_alive(&self, index_y: u32) -> u32 {
        let mut count: u32 = 0;
        for x in 0..self.x_size {
            count += self[(x, index_y)] as u32;
        }
        count
    }

    fn column_alive(&self, index_x: u32) -> u32 {
        let mut count: u32 = 0;
        for y in 0..self.y_size {
            count += self[(index_x, y)] as u32;
        }
        count
    }

    pub fn need_expand(&self) -> bool {
        if self.column_alive(0) > 0 || self.column_alive(self.x_size - 1) > 0 {
            return true;
        }

        if self.row_alive(0) > 0 || self.row_alive(self.y_size - 1) > 0 {
            return true;
        }

        return false;
    }

    pub fn insert(&mut self, place: UCoord, other: &Block) {
        let mut new_x = place.x + other.x_size;
        new_x = if new_x < self.x_size {
            self.x_size
        } else {
            new_x
        };

        let mut new_y = place.y + other.y_size;
        new_y = if new_y < self.y_size {
            self.y_size
        } else {
            new_y
        };

        let mut new = Block::new(new_x, new_y);
        for x in 0..self.x_size {
            for y in 0..self.y_size {
                new[(x, y)] = self[(x, y)];
            }
        }

        for x in 0..other.x_size {
            for y in 0..other.y_size {
                new[(x + place.x, y + place.y)] = match other[(x, y)] {
                    1 => 1,
                    _ => new[(x + place.x, y + place.y)],
                }
            }
        }
        *self = new;
    }

    pub fn cut_empty(&mut self) -> Option<UCoord> {
        let mut x_offset: u32 = 0;
        let mut y_offset: u32 = 0;
        let mut right_x_offset: u32 = 0;
        let mut top_y_offset: u32 = 0;

        loop {
            //count empty columns from the left
            if self.column_alive(x_offset) != 0 {
				if x_offset != 0 {
                    x_offset -= 1; //reserve one empty column for borders
                }
				break;
            }
            if x_offset == self.x_size - 1 {
                //return if no alive cells in block
                return Option::None;
            }
            x_offset += 1;
        }

        loop {
            //count empty rows from the bottom
            if self.row_alive(y_offset) != 0 {
				if y_offset != 0 {
                    y_offset -= 1; //reserve one empty row for borders
                }
                break;
            }
            y_offset += 1;
        }

        loop {
            if self.column_alive(self.x_size - 1 - right_x_offset) != 0 {
				if right_x_offset != 0 {
                    right_x_offset -= 1;
                }
                break;
            }
            right_x_offset += 1;
        }

        loop {
            if self.row_alive(self.y_size - 1 - top_y_offset) != 0 {
				if top_y_offset != 0 {
                    top_y_offset -= 1;
                }
                break;
            }
            top_y_offset += 1;
        }

        let new_x_size = self.x_size - x_offset - right_x_offset;
        let new_y_size = self.y_size - y_offset - top_y_offset;

        let mut new_block = Block::new(new_x_size, new_y_size);
        for x in x_offset..self.x_size - right_x_offset {
            for y in y_offset..self.y_size - top_y_offset {
                new_block[(x - x_offset, y - y_offset)] = self[(x, y)];
            }
        }

        *self = new_block;
        Some(UCoord {
            x: x_offset,
            y: y_offset,
        })
    }

    fn add_border(&mut self) -> Coord {
        let left = match self.column_alive(0) {
            0 => 0,
            _ => 1,
        };

        let bottom = match self.row_alive(0) {
            0 => 0,
            _ => 1,
        };

        let right = match self.column_alive(self.x_size - 1) {
            0 => 0,
            _ => 1,
        };

        let top = match self.row_alive(self.y_size - 1) {
            0 => 0,
            _ => 1,
        };

        let mut new_block = Block::new(self.x_size + left + right, self.y_size + bottom + top);
        for x in 0..self.x_size {
            for y in 0..self.y_size {
                new_block[(x + left, y + bottom)] = self[(x, y)];
            }
        }

        *self = new_block;
        Coord {
            x: 0 - left as i64,
            y: 0 - bottom as i64,
        }
    }

    pub fn resize(&mut self) -> Option<Coord> {
        let offset = match self.cut_empty() {
            None => return None,
            Some(val) => Coord {
                x: val.x as i64,
                y: val.y as i64,
            },
        };

        Option::Some(offset + self.add_border())
    }

    pub fn split(mut self) -> Option<Vec<(Block, Coord)>> {
        let resize_offset = match self.resize() {
            None => return Option::None,
            Some(i) => i,
        };

        let mut vert_splits: Vec<u32> = Vec::new();
        vert_splits.push(0);

        for x in 1..self.x_size - 1 {
            if self.column_alive(x) == 0 && self.column_alive(x - 1) == 0 {
                vert_splits.push(x);
            }
        }

        let mut pieces = Vec::new();


        for x in vert_splits.iter().rev() {
            let mut piece_column = self.cut_block_right(*x);
            let mut horiz_splits = Vec::new();

            for y in 1..self.y_size - 1 {
                if piece_column.row_alive(y) == 0 && piece_column.row_alive(y - 1) == 0 {
                    horiz_splits.push(y);
                }
            }

            for y in horiz_splits.iter().rev() {
                let mut piece = piece_column.cut_block_top(*y);
                let fin_offset = match piece.resize(){
                    None => continue,
                    Some(i) => i + resize_offset + UCoord { x: *x, y: *y }.into()
                };
                pieces.push((piece, fin_offset));
            }
            
            let fin_offset = match piece_column.resize() {
                None => continue,
                Some(i) => i + resize_offset + UCoord { x: *x, y: 0 }.into()
            };
            pieces.push((piece_column, fin_offset));
        }

        Some(pieces)
    }

    fn cut_block_top(&mut self, cut_line: u32) -> Block {
        let mut piece = Block::new(self.x_size, self.y_size - cut_line);
        for x in 0..piece.x_size {
            for y in 0..piece.y_size {
                piece[(x, y)] = self[(x, y + cut_line)];
            }
        }
        self.y_size = cut_line;
        self.data.truncate((self.x_size * self.y_size) as usize);
        piece
    }

    fn cut_block_right(&mut self, cut_line: u32) -> Block {
        let mut piece = Block::new(self.x_size - cut_line, self.y_size);
        for x in 0..piece.x_size {
            for y in 0..piece.y_size {
                piece[(x, y)] = self[(x + cut_line, y)];
            }
        }
        if cut_line == 0 {
            return piece;
        }
        let mut temp_block = Block::new(cut_line, self.y_size);
        for x in 0..temp_block.x_size {
            for y in 0..temp_block.y_size {
                temp_block[(x, y)] = self[(x, y)];
            }
        }
        *self = temp_block;
        piece
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn block_indexing() {
        let block = Block {
            x_size: 2,
            y_size: 2,
            data: vec![0, 1, 1, 0],
        };
        assert_eq!(block[(0, 0)], 0);
        assert_eq!(block[(0, 1)], 1);
        assert_eq!(block[(1, 0)], 1);
        assert_eq!(block[(1, 1)], 0);
    }

    #[test]
    fn block_mutability() {
        let mut block = Block {
            x_size: 2,
            y_size: 2,
            data: vec![0, 1, 1, 0],
        };
        block[(0, 1)] = 1;
        assert_eq!(block[(0, 1)], 1);
    }

    #[test]
    fn block_count_neighbours() {
        let block = Block {
            x_size: 3,
            y_size: 3,
            data: vec![1, 0, 1, 0, 0, 1, 1, 1, 0],
        };
        //1 1 0
        //0 0 1
        //1 0 1

        assert_eq!(block.neighbour_count(UCoord { x: 0, y: 0 }), 0, "Coord 0;0");
        assert_eq!(block.neighbour_count(UCoord { x: 1, y: 0 }), 3, "Coord 1;0");
        assert_eq!(block.neighbour_count(UCoord { x: 2, y: 0 }), 1, "Coord 2;0");
        assert_eq!(block.neighbour_count(UCoord { x: 0, y: 1 }), 3, "Coord 0;1");
        assert_eq!(block.neighbour_count(UCoord { x: 1, y: 1 }), 5, "Coord 2;1");
        assert_eq!(block.neighbour_count(UCoord { x: 2, y: 1 }), 2, "Coord 3;1");
        assert_eq!(block.neighbour_count(UCoord { x: 0, y: 2 }), 1, "Coord 0;2");
        assert_eq!(block.neighbour_count(UCoord { x: 1, y: 2 }), 2, "Coord 1;2");
        assert_eq!(block.neighbour_count(UCoord { x: 2, y: 2 }), 2, "Coord 2;2");
    }

    #[test]
    fn block_step() {
        let mut block = Block {
            x_size: 3,
            y_size: 3,
            data: vec![1, 0, 1, 0, 0, 1, 1, 1, 0],
        };
        //1 1 0
        //0 0 1
        //1 0 1
        assert_eq!(block.neighbour_count(UCoord { x: 1, y: 0 }), 3);
        block.step();
        let next_block = Block {
            x_size: 3,
            y_size: 3,
            data: vec![0, 1, 0, 1, 0, 1, 0, 1, 0],
        };
        assert_eq!(block, next_block);
    }

    #[test]
    fn block_line_count() {
        let block = Block {
            x_size: 3,
            y_size: 3,
            data: vec![1, 0, 1, 1, 0, 1, 1, 1, 0],
        };
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
    fn block_need_expand() {
        let block = Block {
            x_size: 3,
            y_size: 3,
            data: vec![1, 0, 1, 1, 0, 1, 1, 1, 0],
        };
        //1 1 0
        //1 0 1
        //1 0 1
        assert_eq!(block.need_expand(), true);

        let block_no_expand = Block {
            x_size: 3,
            y_size: 3,
            data: vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
        };
        assert_eq!(block_no_expand.need_expand(), false)
    }

    #[test]
    fn block_new() {
        let mut block = Block::new(3, 3);
        assert_eq!(
            block,
            Block {
                x_size: 3,
                y_size: 3,
                data: vec![0; 9]
            }
        );
        block[(1, 1)] = 1;
        assert_eq!(
            block,
            Block {
                x_size: 3,
                y_size: 3,
                data: vec![0, 0, 0, 0, 1, 0, 0, 0, 0]
            }
        );
    }

    #[test]
    fn block_insert() {
        let mut block = Block::new(5, 5);
        let insert = Block {
            x_size: 3,
            y_size: 4,
            data: vec![0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1],
        };
        //0 1 1
        //0 0 0
        //1 1 0
        //0 1 0
        let place = UCoord { x: 1, y: 0 };
        block.insert(place, &insert);

        let mut result = Block::new(5, 5);
        result[(2, 0)] = 1;
        result[(1, 1)] = 1;
        result[(2, 1)] = 1;
        result[(2, 3)] = 1;
        result[(3, 3)] = 1;
        //0 0 0 0 0
        //0 0 1 1 0
        //0 0 0 0 0
        //0 1 1 0 0
        //0 0 1 0 0
        assert_eq!(block, result);

        block.insert(UCoord { x: 5, y: 2 }, &insert);
        result = Block::new(8, 6);
        //0 0 0 0 0 0 1 1
        //0 0 0 0 0 0 0 0
        //0 0 1 1 0 1 1 0
        //0 0 0 0 0 0 1 0
        //0 1 1 0 0 0 0 0
        //0 0 1 0 0 0 0 0
        result[(2, 0)] = 1;
        result[(1, 1)] = 1;
        result[(2, 1)] = 1;
        result[(2, 3)] = 1;
        result[(3, 3)] = 1;

        result[(5, 3)] = 1;
        result[(6, 2)] = 1;
        result[(6, 3)] = 1;
        result[(6, 5)] = 1;
        result[(7, 5)] = 1;

        assert_eq!(block, result);
    }

    #[test]
    fn block_cut_empty_empty() {
        let mut block = Block::new(5, 5);
        let result = block.cut_empty();
        assert_eq!(result, Option::None);
        assert_eq!(block, block);
    }

    #[test]
    fn block_cut_empty() {
        let mut block = Block::new(8, 4);
        block[(3, 0)] = 1;
        block[(2, 1)] = 1;
        block[(4, 1)] = 1;
        block[(4, 3)] = 1;
        //0 0 0 0 1 0 0 0
        //0 0 0 0 0 0 0 0
        //0 0 1 0 1 0 0 0
        //0 0 0 1 0 0 0 0

        let mut result_block = Block::new(5, 4);
        // 0 0 0 1 0
        // 0 0 0 0 0
        // 0 1 0 1 0 
        // 0 0 1 0 0
        result_block[(2, 0)] = 1;
        result_block[(1, 1)] = 1;
        result_block[(3, 1)] = 1;
        result_block[(3, 3)] = 1;

        assert_eq!(block.cut_empty(), Option::Some(UCoord { x: 1, y: 0 }));
        assert_eq!(block, result_block);
    }

    #[test]
    fn block_borders() {
        let mut block = Block::new(3, 4);
        //0 0 0
        //0 0 0
        //1 0 0
        //1 1 0
        block[(0, 0)] = 1;
        block[(1, 0)] = 1;
        block[(0, 1)] = 1;

        let result_offset = Coord { x: -1, y: -1 };
        let mut result_block = Block::new(4, 5);
        //0 0 0 0
        //0 0 0 0
        //0 1 0 0
        //0 1 1 0
        //0 0 0 0
        result_block[(1, 1)] = 1;
        result_block[(2, 1)] = 1;
        result_block[(1, 2)] = 1;

        assert_eq!(block.add_border(), result_offset);
        assert_eq!(block, result_block);
    }

    #[test]
    fn block_resize() {
        let mut block = Block::new(8, 4);
        block[(3, 0)] = 1;
        block[(2, 1)] = 1;
        block[(4, 1)] = 1;
        block[(4, 3)] = 1;
        //0 0 0 0 1 0 0 0
        //0 0 0 0 0 0 0 0
        //0 0 1 0 1 0 0 0
        //0 0 0 1 0 0 0 0

        let mut result_block = Block::new(5, 6);
        // 0 0 0 0 0
        // 0 0 0 1 0
        // 0 0 0 0 0
        // 0 1 0 1 0
        // 0 0 1 0 0
        // 0 0 0 0 0
        result_block[(2, 1)] = 1;
        result_block[(1, 2)] = 1;
        result_block[(3, 2)] = 1;
        result_block[(3, 4)] = 1;

        let coord = Coord { x: 1, y: -1 };
        assert_eq!(block.resize(), Option::Some(coord));
        assert_eq!(block, result_block);
    }

    #[test]
    fn block_cut_top() {
        let mut block = Block::new(3, 5);
        //1 1 1
        //0 1 0
        //1 0 0
        //1 0 0
        //0 0 0
        block[(0, 1)] = 1;
        block[(0, 2)] = 1;
        block[(0, 4)] = 1;
        block[(1, 3)] = 1;
        block[(1, 4)] = 1;
        block[(2, 4)] = 1;

        let mut piece_result = Block::new(3, 2);
        piece_result[(1, 0)] = 1;
        piece_result[(0, 1)] = 1;
        piece_result[(1, 1)] = 1;
        piece_result[(2, 1)] = 1;

        let mut block_result = Block::new(3, 3);
        block_result[(0, 1)] = 1;
        block_result[(0, 2)] = 1;

        assert_eq!(block.cut_block_top(3), piece_result);
        assert_eq!(block, block_result);
    }

    #[test]
    fn block_split() {
        let mut block = Block::new(6, 5);
        //0 0 0 0 1 1
        //1 0 0 0 1 0
        //1 0 0 0 0 0
        //1 0 0 0 0 0
        //0 0 0 0 1 1
        block[(4, 0)] = 1;
        block[(5, 0)] = 1;
        block[(0, 1)] = 1;
        block[(0, 2)] = 1;
        block[(0, 3)] = 1;
        block[(4, 3)] = 1;
        block[(4, 4)] = 1;
        block[(5, 4)] = 1;

        let mut b1 = Block::new(3, 5);
        //0 0 0 
        //0 1 0 
        //0 1 0
        //0 1 0
        //0 0 0
        b1[(1, 1)] = 1;
        b1[(1, 2)] = 1;
        b1[(1, 3)] = 1;
        let c1 = Coord { x: -1, y: 0 };

        let mut b2 = Block::new(4, 3);
        //0 0 0 0
        //0 1 1 0
        //0 0 0 0
        b2[(1, 1)] = 1;
        b2[(2, 1)] = 1;
        let c2 = Coord { x: 3, y: -1 };

        let mut b3 = Block::new(4, 4);
        //0 0 0 0
        //0 1 1 0
        //0 1 0 0
        //0 0 0 0
        b3[(1, 1)] = 1;
        b3[(1, 2)] = 1;
        b3[(2, 2)] = 1;
        let c3 = Coord { x: 3, y: 2};

        let pieces = block.split().unwrap();
        assert_eq!(pieces[0], (b3, c3));
        assert_eq!(pieces[1], (b2, c2));
        assert_eq!(pieces[2], (b1, c1));

        let mut block = Block::new(2, 5);
        //1 1
        //1 0
        //0 0
        //0 0
        //1 1
        block[(0, 0)] = 1;
        block[(0, 3)] = 1;
        block[(0, 4)] = 1;
        block[(1, 0)] = 1;
        block[(1, 4)] = 1;

        let mut b2 = Block::new(4, 3);
        //0 0 0 0
        //0 1 1 0
        //0 0 0 0
        b2[(1, 1)] = 1;
        b2[(2, 1)] = 1;
        let c2 = Coord { x: -1, y: -1 };

        let mut b3 = Block::new(4, 4);
        //0 0 0 0
        //0 1 1 0
        //0 1 0 0
        //0 0 0 0
        b3[(1, 1)] = 1;
        b3[(1, 2)] = 1;
        b3[(2, 2)] = 1;
        let c3 = Coord { x: -1, y: 2};
        let pieces = block.split().unwrap();
        assert_eq!(pieces[0], (b3, c3));
        assert_eq!(pieces[1], (b2, c2));
        assert_eq!(pieces.len(), 2);

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

        block.insert(UCoord {x: 0, y: 16}, &b1);
        block.insert(UCoord {x: 3, y: 0}, &b2);
        let pieces = block.split().unwrap();
        assert_eq!(pieces[1], (b2, Coord {x: 3, y: 0}));
        assert_eq!(pieces[0], (b1, Coord {x: 0, y: 16}));
        assert_eq!(pieces.len(), 2);

    }
}
