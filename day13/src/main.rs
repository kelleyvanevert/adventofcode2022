pub mod packet;
pub mod parse;

use std::fs;

use packet::Packet;
use parse::parse;

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    let pairs = parse(&filecontents);
    println!("Solution: {}", solve(&pairs));
}

fn solve(pairs: &Vec<(Packet, Packet)>) -> usize {
    pairs
        .iter()
        .enumerate()
        .map(|(i, pair)| if pair.0 < pair.1 { i + 1 } else { 0 })
        .sum()
}

#[test]
fn test_solve() {
    let s = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    let pairs = parse(s);
    assert_eq!(solve(&pairs), 13);
}
