//! Contains [Field] struct and its methods

use crate::groups::{group::Group, Coord};
use rstar::{RTree, RTreeObject, AABB, Envelope};
use svg::node::element::Rectangle;
use svg::Document;

///Selection function for R-tree that uses [Group::intersects_smart]
struct SmartSelection<'a> {
    data: &'a Group,
}

impl rstar::SelectionFunction<Group> for SmartSelection<'_> {
    fn should_unpack_parent(&self, envelope: &AABB<(i64, i64)>) -> bool {
        self.data.envelope().intersects(envelope)
    }

    fn should_unpack_leaf(&self, _leaf: &Group) -> bool {
        self.data.intersects_smart(_leaf)
    }
}

///Selection function for R-tree that accepts any element. Useful for popping a singular element
///with RTree::remove_with_selection_function()
struct AllSelection;

impl<T: RTreeObject> rstar::SelectionFunction<T> for AllSelection {
      fn should_unpack_parent(&self, _envelope: &T::Envelope) -> bool {
          true
      }
}

///Selection function for R-tree that selects elements which envelope matches the given one 
struct EnvelopeSelection<'a> {
    data: &'a AABB<(i64, i64)>,
}

impl rstar::SelectionFunction<Group> for EnvelopeSelection<'_> {
    fn should_unpack_parent(&self, envelope: &AABB<(i64, i64)>) -> bool {
        self.data.intersects(envelope)
    }

    fn should_unpack_leaf(&self, leaf: &Group) -> bool {
        *self.data == leaf.envelope()
    }
}

pub struct Field {
    pub field: RTree<Group>,
}

impl Field {
    /// Returns a max size AABB. Used to drain all tree contents
    fn full_tree() -> AABB<(i64, i64)> {
        AABB::from_corners((i64::MIN, i64::MIN), (i64::MAX, i64::MAX))
    }

    /// Prepares svg document for groups. Sets max/min coords and background color
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

    /// Returns global coords of bottom left corner of the active field
    pub fn bottom_left(&self) -> Option<Coord> {
        let mut bl = match self.field.iter().next() {
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

    /// Returns global coords of top right corner of the active field
    pub fn top_right(&self) -> Option<Coord> {
        let mut tr = match self.field.iter().next() {
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

    /// Inserts data of every group in Field into svg document
    pub fn svg_draw(&self, mut doc: Document) -> Document {
        for group in &self.field {
            doc = group.svg_add(doc);
        }
        doc
    }

    /// Advances [Field] to next game generation
    pub fn step(mut self) -> Self {
        let mut step_field = Vec::new();
        for group in self.field.drain_in_envelope(Field::full_tree()) {
            match group.step() {
                None => (),
                Some(mut vec) => step_field.append(&mut vec),
            };
        }

        let mut tree = RTree::bulk_load(step_field);
        loop {
            let mut merge_envelope = Option::<AABB<(i64, i64)>>::None;
            for piece in tree.iter() {
                if tree.locate_with_selection_function(SmartSelection{data: &piece}).count() > 1 {
                    merge_envelope = Some(piece.envelope()); 
                    break;
                }
            }
            if merge_envelope == None {
                break;
            }
            for cur in tree.drain_with_selection_function(EnvelopeSelection{data:&merge_envelope.unwrap()}).collect::<Vec<Group>>() {
                let mut merge_group = None;
                for piece in tree.drain_with_selection_function(SmartSelection{data: &cur}) {
                    merge_group = match merge_group {
                        None => Some(piece),
                        Some(val) => Some(val.merge(piece)),
                    };
                }
                merge_group = match merge_group {
                    None => Some(cur),
                    Some(val) => Some(val.merge(cur)),
                };
                tree.insert(merge_group.unwrap());                
            }
        }

        Field { field: tree }
    }

    /// Drains **self** into [Vec]
    pub fn tree_to_vec(mut tree: RTree<Group>) -> Vec::<Group> {
        let mut field = Vec::new();
        for group in tree.drain_in_envelope(Field::full_tree()) {
            field.push(group);
        }
        field
    }
}

#[cfg(test)]
mod test {
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
}
