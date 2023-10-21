#[macro_use]
extern crate pest_derive;

mod parser;

use crate::parser::{parse_entry, AssignmentPair};

fn main() {
    let filecontents = get_input();

    println!(
        "num containments found: {}",
        solve(parse_entry(&filecontents))
    );
    println!(
        "num overlaps found: {}",
        solve_v2(parse_entry(&filecontents))
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

fn solve_v2(assignment_pairs: Vec<AssignmentPair>) -> i32 {
    assignment_pairs
        .iter()
        .map(|&p| if overlap(p) { 1 } else { 0 })
        .sum::<i32>()
}

fn overlap((le, ri): AssignmentPair) -> bool {
    (ri.start <= le.start && le.start <= ri.end)
        || (ri.start <= le.end && le.end <= ri.end)
        || (le.start <= ri.start && ri.start <= le.end)
        || (le.start <= ri.end && ri.end <= le.end)
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

#[test]
pub fn test_solve_v2() {
    assert_eq!(
        4,
        solve_v2(parse_entry(
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

fn get_input() -> String {
    dotenv::dotenv().ok();
    let key = std::env::var("KEY").expect("Missing env var KEY");

    let bytes = std::fs::read("./input.txt.encrypted").unwrap();
    decrypt(key.as_bytes(), &bytes)
}

fn decrypt(key: &[u8], enc: &[u8]) -> String {
    String::from_utf8(
        enc.iter()
            .enumerate()
            .map(|(i, &b)| b.wrapping_sub(key[i % key.len()]))
            .collect(),
    )
    .unwrap()
}
