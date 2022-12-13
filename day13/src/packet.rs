use std::{cmp::Ordering, iter::zip};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

fn compare(x: &Packet, y: &Packet) -> Ordering {
    match (x, y) {
        (Packet::Int(x), Packet::Int(y)) => x.cmp(y),
        (Packet::Int(x), Packet::List(ys)) => compare_vec(&vec![Packet::Int(*x)], ys),
        (Packet::List(xs), Packet::Int(y)) => compare_vec(xs, &vec![Packet::Int(*y)]),
        (Packet::List(xs), Packet::List(ys)) => compare_vec(xs, ys),
    }
}

fn compare_vec(xs: &Vec<Packet>, ys: &Vec<Packet>) -> Ordering {
    for (x, y) in zip(xs, ys) {
        match compare(x, y) {
            Ordering::Equal => {} // continue
            otherwise => {
                return otherwise;
            }
        }
    }

    if xs.len() == ys.len() {
        Ordering::Equal
    } else {
        xs.len().cmp(&ys.len())
    }
}

#[test]
fn test_ordering() {
    use crate::parse::parse_packet;

    assert!(Packet::Int(2) < Packet::Int(3));
    assert!(Packet::Int(3) == Packet::Int(3));
    assert!(Packet::Int(4) > Packet::Int(3));

    assert!(parse_packet("[1,1,3,1,1]") < parse_packet("[1,1,5,1,1]"));
    assert!(parse_packet("[[1],[2,3,4]]") < parse_packet("[[1],4]"));
    assert!(parse_packet("[9]") > parse_packet("[[8,7,6]]"));
    assert!(parse_packet("[[4,4],4,4]") < parse_packet("[[4,4],4,4,4]"));
    assert!(parse_packet("[7,7,7,7]") > parse_packet("[7,7,7]"));
    assert!(parse_packet("[]") < parse_packet("[3]"));
    assert!(parse_packet("[[[]]]") > parse_packet("[[]]"));
    assert!(
        parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]") > parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]")
    );

    let mut packets = vec![
        parse_packet("[1,1,3,1,1]"),
        parse_packet("[1,1,5,1,1]"),
        parse_packet("[[1],[2,3,4]]"),
        parse_packet("[[1],4]"),
        parse_packet("[9]"),
        parse_packet("[[8,7,6]]"),
        parse_packet("[[4,4],4,4]"),
        parse_packet("[[4,4],4,4,4]"),
        parse_packet("[7,7,7,7]"),
        parse_packet("[7,7,7]"),
        parse_packet("[]"),
        parse_packet("[3]"),
        parse_packet("[[[]]]"),
        parse_packet("[[]]"),
        parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
        parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]"),
    ];

    packets.sort();

    assert_eq!(
        packets,
        vec![
            parse_packet("[]"),
            parse_packet("[[]]"),
            parse_packet("[[[]]]"),
            parse_packet("[1,1,3,1,1]"),
            parse_packet("[1,1,5,1,1]"),
            parse_packet("[[1],[2,3,4]]"),
            parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]"),
            parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            parse_packet("[[1],4]"),
            // parse_packet("[[2]]"),
            parse_packet("[3]"),
            parse_packet("[[4,4],4,4]"),
            parse_packet("[[4,4],4,4,4]"),
            // parse_packet("[[6]]"),
            parse_packet("[7,7,7]"),
            parse_packet("[7,7,7,7]"),
            parse_packet("[[8,7,6]]"),
            parse_packet("[9]"),
        ]
    );
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}
