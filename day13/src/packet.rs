use std::iter::zip;

#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

fn decide(x: &Packet, y: &Packet) -> Option<bool> {
    match (x, y) {
        (Packet::Int(x), Packet::Int(y)) => {
            if x == y {
                None
            } else {
                Some(x < y)
            }
        }
        (Packet::Int(x), Packet::List(ys)) => decide_vec(&vec![Packet::Int(*x)], ys),
        (Packet::List(xs), Packet::Int(y)) => decide_vec(xs, &vec![Packet::Int(*y)]),
        (Packet::List(xs), Packet::List(ys)) => decide_vec(xs, ys),
    }
}

pub fn is_less(x: &Packet, y: &Packet) -> bool {
    Some(true) == decide(x, y)
}

fn decide_vec(xs: &Vec<Packet>, ys: &Vec<Packet>) -> Option<bool> {
    for (x, y) in zip(xs, ys) {
        match decide(x, y) {
            None => {} // continue
            otherwise => {
                return otherwise;
            }
        }
    }

    if xs.len() == ys.len() {
        None
    } else {
        Some(xs.len() < ys.len())
    }
}

#[test]
fn test_is_less() {
    use crate::parse::parse_packet;

    assert_eq!(decide(&Packet::Int(2), &Packet::Int(3)), Some(true));
    assert_eq!(decide(&Packet::Int(3), &Packet::Int(3)), None);
    assert_eq!(decide(&Packet::Int(4), &Packet::Int(3)), Some(false));

    assert_eq!(
        decide(&parse_packet("[1,1,3,1,1]"), &parse_packet("[1,1,5,1,1]")),
        Some(true)
    );

    assert_eq!(
        decide(&parse_packet("[[1],[2,3,4]]"), &parse_packet("[[1],4]")),
        Some(true)
    );

    assert_eq!(
        decide(&parse_packet("[9]"), &parse_packet("[[8,7,6]]")),
        Some(false)
    );

    assert_eq!(
        decide(&parse_packet("[[4,4],4,4]"), &parse_packet("[[4,4],4,4,4]")),
        Some(true)
    );

    assert_eq!(
        decide(&parse_packet("[7,7,7,7]"), &parse_packet("[7,7,7]")),
        Some(false)
    );

    assert_eq!(
        decide(&parse_packet("[]"), &parse_packet("[3]")),
        Some(true)
    );

    assert_eq!(
        decide(&parse_packet("[[[]]]"), &parse_packet("[[]]")),
        Some(false)
    );

    assert_eq!(
        decide(
            &parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            &parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]")
        ),
        Some(false)
    );
}
