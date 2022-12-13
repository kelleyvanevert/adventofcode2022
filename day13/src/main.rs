pub mod packet;
pub mod parse;

use std::fs;

use packet::Packet;
use parse::parse;

use crate::parse::parse_packet;

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    let pairs = parse(&filecontents);
    println!("Solution: {}", solve(&pairs));

    let mut all = pairs
        .iter()
        .flat_map(|pair| [&pair.0, &pair.1])
        .collect::<Vec<&Packet>>();

    let div1 = parse_packet("[[2]]");
    let div2 = parse_packet("[[6]]");

    all.push(&div1);
    all.push(&div2);

    all.sort();

    let i1 = all.iter().enumerate().find(|p| **p.1 == div1).unwrap().0;
    let i2 = all.iter().enumerate().find(|p| **p.1 == div2).unwrap().0;

    println!("Decoder key: {}", (i1 + 1) * (i2 + 1));
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
