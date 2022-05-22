use super::Coord;
use super::block::Block;

#[derive(Debug)]
struct Group {
    global_coord: Coord,
    block: Block,
}

impl Group {
    pub fn top_right(&self) -> Coord {
        return Coord {x : self.global_coord.x + self.block.x_size as i64, 
                      y : self.global_coord.y + self.block.y_size as i64}
    }

    pub fn intersects(&self, other: &Group) -> bool {
        let self_tr = self.top_right();
        let other_tr = other.top_right();
        if (self.global_coord.x > other_tr.x || self.global_coord.y > other_tr.y) &&
           (other.global_coord.x > self_tr.x || other.global_coord.y > self_tr.y) {
            return false
        } else {
            return true
        }
    }

}
