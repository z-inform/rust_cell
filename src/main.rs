use rstar::RTree;
use rust_cell::groups::{block::Block, field::Field, group::Group, Coord};
use svg::node::element::Rectangle;

/// Returns a block with Lidka predecessor (29126 generations lifespan)
fn lidka() -> Block {
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

/// Returns a block with a r pentomino (1103 generations lifespan)
fn r_pentomino() -> Block {
    let mut block = Block::new(5, 5);
    block[(1, 2)] = 1;
    block[(2, 1)] = 1;
    block[(2, 2)] = 1;
    block[(2, 3)] = 1;
    block[(3, 3)] = 1;
    block
}

fn main() {
    let mut block;
    let age;
    let args: Vec<String> = std::env::args().collect();
    let mut parallel_flag: bool = false;

    if args.len() < 2 {
        println!("Not enough arguments");
        return;
    }
    if args[1] == "help" {
        println!("{}{}{}", "Use \"r-pentomino\" or \"lidka\" to run full length of that pattern. Optionally specify generation to stop at.\n",
                         "If no pattern name is provided, max generation is expected and RLE-formatted pattern will be read from stdin\n",
                         "Specify \"parallel\" as first argument to run in multiple threads");
        return;
    }

    let mut arg_count = 1;

    if args[1] == "parallel" {
        parallel_flag = true;
        arg_count = 2;
    }

    if args[arg_count] == "r-pentomino" {
        block = r_pentomino();
        age = match args.get(arg_count + 1) {
            None => 1103,
            Some(val) => val.parse().unwrap(),
        };
    } else if args[arg_count] == "lidka" {
        block = lidka();
        age = match args.get(arg_count + 1) {
            None => 29126,
            Some(val) => val.parse().unwrap(),
        };
    } else if let Ok(val) = args[arg_count].parse() {
        age = val;
        println!("Enter RLE pattern");
        let mut buf = "".to_string();
        loop {
            match std::io::stdin().read_line(&mut buf) {
                Ok(0) => break,
                _ => (),
            }
        }
        block = Block::rle_import(&buf).unwrap();
        block.resize();
    } else {
        println!("Incorrect arguments\n");
        return;
    }

    let coord = Coord { x: 0, y: 0 };

    let mut group = Group::new(coord, block);
    group.reverse_y();
    let mut test_field = Field::new(RTree::new());
    test_field.field.insert(group);
    for _i in 0..age {
        if parallel_flag {
            test_field.step_parallel();
        } else {
            test_field.step();
        }
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
}
