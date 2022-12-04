#[macro_use]
extern crate pest_derive;

mod parser;

use crate::parser::{parse_entry, AssignmentPair};
use std::fs;

fn main() {
    let filecontents = fs::read_to_string("input.txt").unwrap();

    println!(
        "num containments found: {}",
        solve(parse_entry(&filecontents))
    );
}

fn solve(assignment_pairs: Vec<AssignmentPair>) -> i32 {
    assignment_pairs
        .iter()
        .map(|&p| {
            if contains(p) || contains((p.1, p.0)) {
                1
            } else {
                0
            }
        })
        .sum::<i32>()
}

fn contains((le, ri): AssignmentPair) -> bool {
    le.start <= ri.start && le.end >= ri.end
}

#[test]
pub fn test_solve() {
    assert_eq!(
        2,
        solve(parse_entry(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
        ))
    );
}
