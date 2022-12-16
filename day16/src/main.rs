use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();
    let data = parse(&filecontents);
    println!("Hello, world!");
}

fn parse<'a>(s: &'a str) -> HashMap<&'a str, (usize, Vec<&'a str>)> {
    let re =
        Regex::new(r"^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)$")
            .unwrap();

    let mut data = HashMap::new();

    for line in s.lines() {
        let m = re.captures(line).unwrap();
        data.insert(
            m.get(1).unwrap().as_str(),
            (
                m[2].parse::<usize>().unwrap(),
                m.get(3).unwrap().as_str().split(", ").collect(),
            ),
        );
    }

    data
}

#[test]
fn test_parse() {
    let s = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    let data = parse(s);

    assert_eq!(
        data,
        HashMap::from([
            ("AA", (0, vec!["DD", "II", "BB"])),
            ("BB", (13, vec!["CC", "AA"])),
            ("CC", (2, vec!["DD", "BB"])),
            ("DD", (20, vec!["CC", "AA", "EE"])),
            ("EE", (3, vec!["FF", "DD"])),
            ("FF", (0, vec!["EE", "GG"])),
            ("GG", (0, vec!["FF", "HH"])),
            ("HH", (22, vec!["GG"])),
            ("II", (0, vec!["AA", "JJ"])),
            ("JJ", (21, vec!["II"])),
        ])
    );
}
