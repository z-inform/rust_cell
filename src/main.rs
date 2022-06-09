use rust_cell::groups::{block::Block, group::Group, Coord, field::Field};
use svg::node::element::Rectangle;

fn lidka() -> Block { //29126 generations evolution
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

fn r_pentomino() -> Block { //1103 generations evolution
    let mut block = Block::new(5, 5);
    block[(1, 2)] = 1;
    block[(2, 1)] = 1;
    block[(2, 2)] = 1;
    block[(2, 3)] = 1;
    block[(3, 3)] = 1;
    block
}

fn main() {
    let coord = Coord { x: 0, y: 0 };

    let mut group = Group::new(coord, lidka());
    group.reverse_y();
    let mut test_field = Field {
        field: Vec::new(),
    };
    test_field.field.push(group);
    for _i in 0..29126 {
         test_field = test_field.r_tree_step();
    }
    let mut doc = svg::Document::new();
    doc = test_field.prep_svg(doc);
    doc = test_field.svg_draw(doc);
    let start = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("height", 10)
        .set("width", 10)
        .set("fill", "blue");
    doc = doc.add(start);
    svg::save("life.svg", &doc).unwrap();
    println!("{}", test_field.field.len());
}
