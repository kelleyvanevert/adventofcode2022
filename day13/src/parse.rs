use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

fn parse_int(s: &str) -> IResult<&str, Packet> {
    map(digit1, |s: &str| Packet::Int(s.parse::<usize>().unwrap()))(s)
}

fn parse_packet(s: &str) -> IResult<&str, Packet> {
    delimited(
        tag("["),
        map(
            separated_list0(tag(","), alt((parse_int, parse_packet))),
            |list| Packet::List(list),
        ),
        tag("]"),
    )(s)
}

fn parse_packet_pair(s: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(parse_packet, newline, parse_packet)(s)
}

fn parse_all(s: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list0(tag("\n\n"), parse_packet_pair)(s)
}

#[test]
fn test_parse() {
    assert_eq!(parse_int("6a"), Ok(("a", Packet::Int(6))));
    assert_eq!(parse_packet("[]"), Ok(("", Packet::List(vec![]))));
    assert_eq!(
        parse_packet("[5]-"),
        Ok(("-", Packet::List(vec![Packet::Int(5)])))
    );
    assert_eq!(
        parse_packet("[5,6]--"),
        Ok(("--", Packet::List(vec![Packet::Int(5), Packet::Int(6)])))
    );
    assert_eq!(
        parse_packet("[[1],[2,[3],4]]--"),
        Ok((
            "--",
            Packet::List(vec![
                Packet::List(vec![Packet::Int(1)]),
                Packet::List(vec![
                    Packet::Int(2),
                    Packet::List(vec![Packet::Int(3)]),
                    Packet::Int(4)
                ]),
            ])
        ))
    );

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

    let r = parse_all(s);
    assert!(r.is_ok());
    let (_, pairs) = r.unwrap();
    assert_eq!(pairs.len(), 8);
}

pub fn parse(s: &str) -> Vec<(Packet, Packet)> {
    parse_all(s).unwrap().1
}
