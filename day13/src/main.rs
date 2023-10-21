pub mod packet;
pub mod parse;

use crate::parse::parse_packet;
use packet::Packet;
use parse::parse;

fn main() {
    let filecontents = get_input();

    let pairs = parse(&filecontents);
    println!("Solution: {}", solve(&pairs));
    println!("Decoder key: {}", solve_v2(&pairs));
}

fn solve(pairs: &Vec<Packet>) -> usize {
    pairs
        .chunks(2)
        .enumerate()
        .map(|(i, pair)| if pair[0] < pair[1] { i + 1 } else { 0 })
        .sum()
}

fn solve_v2(pairs: &Vec<Packet>) -> usize {
    let mut all = pairs.iter().collect::<Vec<&Packet>>();

    let div1 = parse_packet("[[2]]");
    let div2 = parse_packet("[[6]]");

    all.push(&div1);
    all.push(&div2);

    all.sort();

    let i1 = all.iter().position(|p| **p == div1).unwrap();
    let i2 = all.iter().position(|p| **p == div2).unwrap();

    (i1 + 1) * (i2 + 1)
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
    assert_eq!(solve_v2(&pairs), 140);
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
