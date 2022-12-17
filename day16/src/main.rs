use regex::Regex;
use std::{cmp, collections::HashMap, fmt::Debug, fs, time::Instant};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    let data = parse(&filecontents);

    let t0 = Instant::now();
    let max = State::new().find_best(&data, "".to_string());
    println!("Max: {}", max);
    println!(" took: {:?}", t0.elapsed());
}

type Data<'a> = HashMap<&'a str, (usize, Vec<&'a str>)>;

fn parse<'a>(s: &'a str) -> Data<'a> {
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

struct State<'a> {
    at: &'a str,
    time_left: usize,
    valves: HashMap<&'a str, Option<usize>>,
    visited: HashMap<&'a str, usize>,
    total: usize,
}

impl<'a> Debug for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "State({} @ {} ({}) total {})",
            self.at,
            self.time_left,
            self.valves
                .iter()
                .filter_map(|(&name, &open)| { open.map(|n| format!("{}: {}", name, n)) })
                .collect::<Vec<String>>()
                .join(","),
            self.total
        ))
    }
}

impl<'a> State<'a> {
    fn new() -> Self {
        Self {
            at: "AA",
            time_left: 30,
            valves: HashMap::new(),
            visited: HashMap::from([("AA", 0)]),
            total: 0,
        }
    }

    fn open_valve(&self, data: &Data<'a>) -> State<'a> {
        let mut visited = self.visited.clone();
        let mut valves = self.valves.clone();

        let pressure_to_be_released = data[self.at].0 * (self.time_left - 1);
        let total = self.total + pressure_to_be_released;

        valves.insert(self.at, Some(pressure_to_be_released));
        visited.insert(self.at, total);

        Self {
            at: self.at,
            time_left: self.time_left - 1,
            valves,
            visited,
            total,
        }
    }

    fn goto(&self, dest: &'a str) -> State<'a> {
        let mut visited = self.visited.clone();
        visited.insert(dest, self.total);

        Self {
            at: dest,
            time_left: self.time_left - 1,
            valves: self.valves.clone(),
            visited,
            total: self.total,
        }
    }

    fn find_best(&self, data: &Data<'a>, depth: String) -> usize {
        let subdepth = format!("  {}", depth);
        let mut max = self.total;

        // println!(
        //     "{}At {} time {} total {}",
        //     depth, self.at, self.time_left, self.total
        // );

        if self.time_left > 0 {
            if !self.valves.contains_key(self.at) && data[self.at].0 > 0 {
                // [x] only if total increases
                // println!("{} -> open", depth);
                max = cmp::max(max, self.open_valve(data).find_best(data, subdepth.clone()));
            }
            for &dest in &data[self.at].1 {
                // [ ] prevent unnecessary move back

                //  AA 0 @ 30  -> DD 0 @ 29
                // (DD 0 @ 29) -> AA 0 @ 28 <- prevent

                match self.visited.get(dest) {
                    None => {
                        // not visited before -> defo try
                        max = cmp::max(max, self.goto(dest).find_best(data, subdepth.clone()));
                    }
                    Some(&total) => {
                        // only try if
                        if total < self.total {
                            max = cmp::max(max, self.goto(dest).find_best(data, subdepth.clone()));
                        }
                    }
                }
            }
        }

        max
    }
}

#[test]
fn test_all() {
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

    assert_eq!(State::new().find_best(&data, "".to_string()), 1651);
}
