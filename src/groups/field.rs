use crate::groups::{group::Group, Coord};
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
}
