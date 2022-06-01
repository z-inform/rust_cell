use rust_cell::groups::{block::Block, group::Group, Coord};

fn main() {
    let mut block = Block::new(3, 3);
    block[(1, 0)] = 1;
    block[(2, 1)] = 1;
    block[(0, 2)] = 1;
    block[(1, 2)] = 1;
    block[(2, 2)] = 1;

    let coord = Coord { x: 0, y: 0 };

    let group = Group::new(coord, block);
    let mut doc = svg::Document::new()
        .set("viewBox", (0, 0, 30, 30));
    doc = group.svg_add(doc);
    svg::save("life.svg", &doc, ).unwrap();
}
