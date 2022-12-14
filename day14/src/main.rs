use std::fs;

use crate::sand::parse;

mod sand;

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    let mut cave_block = parse(&filecontents);
    println!(
        "Number of grains of sand: {}",
        cave_block.drop_grains_until_done()
    );
}
