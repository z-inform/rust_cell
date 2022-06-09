use crate::groups::{group::Group, Coord};
use rstar::{RTree, RTreeObject, AABB};
use svg::node::element::Rectangle;
use svg::Document;

pub struct Field {
    pub field: Vec<Group>,
}

impl Field {
    pub fn prep_svg(&self, mut doc: Document) -> Document {
        let bl = match self.bottom_left() {
            None => Coord { x: -50, y: -50 },
            Some(i) => i,
        };
        let rt = match self.top_right() {
            None => Coord { x: 50, y: 50 },
            Some(i) => i,
        };
        let rec = Rectangle::new()
            .set("x", bl.x * 10)
            .set("y", bl.y * 10)
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "grey");
        doc = doc.add(rec);
        doc.set(
            "viewBox",
            (
                bl.x * 10,
                bl.y * 10,
                (rt.x - bl.x + 1) * 10,
                (rt.y - bl.y + 1) * 10,
            ),
        )
    }

    pub fn bottom_left(&self) -> Option<Coord> {
        let mut bl = match self.field.get(0) {
            None => return None,
            Some(i) => i.global_coord,
        };

        for group in &self.field {
            if group.global_coord.x < bl.x {
                bl.x = group.global_coord.x;
            }
            if group.global_coord.y < bl.y {
                bl.y = group.global_coord.y;
            }
        }
        Some(bl)
    }

    pub fn top_right(&self) -> Option<Coord> {
        let mut tr = match self.field.get(0) {
            None => return None,
            Some(i) => i.top_right(),
        };

        for group in &self.field {
            let coord = group.top_right();
            if coord.x > tr.x {
                tr.x = coord.x;
            }
            if coord.y > tr.y {
                tr.y = coord.y;
            }
        }
        Some(tr)
    }

    pub fn svg_draw(&self, mut doc: Document) -> Document {
        for group in &self.field {
            doc = group.svg_add(doc);
        }
        doc
    }

    pub fn step(&mut self) {
        let mut step_field = Vec::new();
        for group in self.field.drain(..) {
            match group.step() {
                None => (),
                Some(mut vec) => step_field.append(&mut vec),
            };
        }
        loop {
            let mut merged = false;

            'bigger: for i in 0..step_field.len() {
                let cur_group = step_field.get(i).unwrap();
                for j in (i + 1)..step_field.len() {
                    if cur_group.intersects(step_field.get(j).unwrap()) == true {
                        let second = step_field.swap_remove(j);
                        let first = step_field.swap_remove(i);
                        step_field.push(first.merge(second));
                        merged = true;
                        break 'bigger;
                    }
                }
            }

            if merged == false {
                break;
            }
        }
        self.field = step_field;
    }

    pub fn r_tree_step(mut self) -> Self {
        //uses r*-tree to simplify the algorithm to O(n log n)
        let mut step_field = Vec::new();
        for group in self.field.drain(..) {
            match group.step() {
                None => (),
                Some(mut vec) => step_field.append(&mut vec),
            };
        }

        let mut tree = RTree::bulk_load(step_field);
        loop {
            let mut start_env = None;
            for group in tree.iter() {
                if tree
                    .locate_in_envelope_intersecting(&group.envelope())
                    .count()
                    > 1
                {
                    start_env = Some(group.envelope());
                }
            }
            if let Some(val) = start_env {
                let mut new = None;
                for piece in tree.drain_in_envelope_intersecting(val) {
                    new = match new {
                        None => Some(piece),
                        Some(new) => Some(new.merge(piece)),
                    };
                }
                tree.insert(new.unwrap());
            } else {
                break;
            }
        }

        let mut new_field = Vec::new();
        for piece in tree.drain_in_envelope(AABB::from_corners(
            (i64::MIN, i64::MIN),
            (i64::MAX, i64::MAX),
        )) {
            new_field.push(piece);
        }
        Field { field: new_field }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::groups::block::Block;
    fn lidka() -> Block {
        //29126 generations evolution
        let mut block = Block::new(11, 8);
        block[(1, 1)] = 1;
        block[(2, 1)] = 1;
        block[(3, 1)] = 1;
        block[(4, 2)] = 1;
        block[(4, 3)] = 1;
        block[(5, 3)] = 1;
        block[(7, 5)] = 1;
        block[(7, 6)] = 1;
        block[(8, 5)] = 1;
        block[(9, 1)] = 1;
        block[(9, 2)] = 1;
        block[(9, 3)] = 1;
        block[(9, 5)] = 1;
        block
    }

    fn r_pentomino() -> Block {
        //1103 generations evolution
        let mut block = Block::new(5, 5);
        block[(1, 2)] = 1;
        block[(2, 1)] = 1;
        block[(2, 2)] = 1;
        block[(2, 3)] = 1;
        block[(3, 3)] = 1;
        block
    }

    //these won't work without a really strange way of comparing vectors
    #[test]
    #[ignore]
    fn cmp_steps_lidka() {
        let coord = Coord { x: 0, y: 0 };
        let mut f1 = Field { field: Vec::new() };
        f1.field.push(Group::new(coord, lidka()));

        let mut f2 = Field { field: Vec::new() };
        f2.field.push(Group::new(coord, lidka()));

        for _i in 0..29126 {
            f1.step();
            f2 = f2.r_tree_step();
        }

        assert_eq!(f1.field, f2.field);
    }

    #[test]
    #[ignore]
    fn cmp_steps_r_pentomino() {
        let coord = Coord { x: 0, y: 0 };
        let mut f1 = Field { field: Vec::new() };
        f1.field.push(Group::new(coord, r_pentomino()));

        let mut f2 = Field { field: Vec::new() };
        f2.field.push(Group::new(coord, r_pentomino()));

        for _i in 0..1103 {
            f1.step();
            f2 = f2.r_tree_step();
        }

        assert_eq!(f1.field, f2.field);
    }
}
