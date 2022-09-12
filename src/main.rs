use rstar::RTree;
use rust_cell::groups::{block::Block, field::Field, group::Group, Coord};
use svg::node::element::Rectangle;
use clap::Parser;

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

#[derive(Parser)]
struct Cli {
    /// Enables parallel calculations
    #[clap(short, long, action)]
    parallel: bool,
    
    /// Number of threads to use
    #[clap(short, long, value_parser, default_value_t = 8)]
    jobs: u8,

    /// Number of generations to be run
    #[clap(short, long, value_parser, required_unless_present = "pattern")]
    generations: Option<u32>,

    /// Path to output SVG file to
    #[clap(short, long, value_parser, value_name = "FILE", default_value_t = String::from("life.svg"))]
    output_file: String,

    /// Pattern to run. Available "lidka" and "r-pentomino". When not specified RLE is parsed 
    /// from stdin
    #[clap(value_parser)]
    pattern: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let coord = Coord { x: 0, y: 0 };
    let mut age = 0;

    let mut group = match &cli.pattern {
        Some(pattern) => {
            if pattern == "r-pentomino" {
                age = 1103;
                Group::new(coord, r_pentomino())
            } else if pattern == "lidka" {
                age = 29126;
                Group::new(coord, lidka())
            } else {
                panic!("Unknown pattern specified\n");
            }
        }
        None => {
			let mut buf = "".to_string();
				loop {
				match std::io::stdin().read_line(&mut buf) {
					Ok(0) => break,
					_ => (),
				}
			}
            Group::new(coord, Block::rle_import(&buf).expect("Cannot parse pattern\n"))
        }
    };

    group.reverse_y();
    let mut test_field = Field::new(RTree::new());
    test_field.field.insert(group);

    age = match &cli.generations {
        Some(val) => *val,
        None => age,
    };

    test_field.request_parallelizm(cli.jobs);

    for _i in 0..age {
        if cli.parallel {
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
    svg::save(&cli.output_file, &doc).unwrap();
}
