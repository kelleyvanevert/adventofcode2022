use regex::Regex;
use std::{cmp, collections::HashMap, fmt::Debug, fs, time::Instant};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    let mut data = parse(&filecontents);

    println!("Before simplification: {:?}", data);

    simplify(&mut data);

    println!("After simplification: {:?}", data);

    let t0 = Instant::now();
    let max = State::new().find_best(&data, "".to_string());
    println!("Max: {}", max);
    println!(" took: {:?}", t0.elapsed());
}

type Data<'a> = HashMap<&'a str, (usize, HashMap<&'a str, usize>)>;

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
                m.get(3)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|dest| (dest, 1))
                    .collect(),
            ),
        );
    }

    data
}

fn simplify<'a>(data: &mut Data<'a>) {
    while let Some((&a, _)) = data
        .iter()
        .find(|&(&a, (flow_rate, _))| *flow_rate == 0 && a != "AA")
    {
        let (_, destinations) = data.remove(a).unwrap();
        // println!("Found node to remove: {} -- {:?}", a, destinations);

        for (&b, &dist_to_b) in &destinations {
            if !data.contains_key(b) {
                panic!("Weird: data does not contain {}", b);
            }
            let (_, tunnels) = data.get_mut(b).unwrap();
            // println!("Rewrite {} from {:?}", b, tunnels);

            tunnels.retain(|&name, _| name != a);

            for (&c, &dist_to_c) in &destinations {
                if b != c {
                    tunnels.insert(c, dist_to_b + dist_to_c);
                }
                // updated_dests.insert((d + dist, dest));
            }

            // println!("  to {:?}", tunnels);
        }
    }
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

    fn goto(&self, dist: usize, dest: &'a str) -> State<'a> {
        let mut visited = self.visited.clone();
        visited.insert(dest, self.total);

        Self {
            at: dest,
            time_left: self.time_left.saturating_sub(dist),
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
                max = cmp::max(max, self.open_valve(data).find_best(data, subdepth.clone()));
            }

            for (&dest, &dist) in &data[self.at].1 {
                // [x] prevent unnecessary move back

                match self.visited.get(dest) {
                    None => {
                        // not visited before -> defo try
                        max =
                            cmp::max(max, self.goto(dist, dest).find_best(data, subdepth.clone()));
                    }
                    Some(&total) => {
                        // only try if
                        if total < self.total {
                            max = cmp::max(
                                max,
                                self.goto(dist, dest).find_best(data, subdepth.clone()),
                            );
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

    let mut data = parse(s);

    assert_eq!(
        data,
        HashMap::from([
            ("AA", (0, HashMap::from([("DD", 1,), ("II", 1), ("BB", 1)]))),
            ("BB", (13, HashMap::from([("CC", 1,), ("AA", 1)]))),
            ("CC", (2, HashMap::from([("DD", 1,), ("BB", 1)]))),
            (
                "DD",
                (20, HashMap::from([("CC", 1,), ("AA", 1), ("EE", 1)]))
            ),
            ("EE", (3, HashMap::from([("FF", 1,), ("DD", 1)]))),
            ("FF", (0, HashMap::from([("EE", 1,), ("GG", 1)]))),
            ("GG", (0, HashMap::from([("FF", 1,), ("HH", 1)]))),
            ("HH", (22, HashMap::from([("GG", 1,)]))),
            ("II", (0, HashMap::from([("AA", 1,), ("JJ", 1)]))),
            ("JJ", (21, HashMap::from([("II", 1,)]))),
        ])
    );

    simplify(&mut data);

    assert_eq!(
        data,
        HashMap::from([
            ("AA", (0, HashMap::from([("DD", 1), ("JJ", 2), ("BB", 1)]))),
            ("BB", (13, HashMap::from([("CC", 1), ("AA", 1)]))),
            ("CC", (2, HashMap::from([("DD", 1), ("BB", 1)]))),
            ("DD", (20, HashMap::from([("CC", 1), ("AA", 1), ("EE", 1)]))),
            ("EE", (3, HashMap::from([("HH", 3), ("DD", 1)]))),
            ("HH", (22, HashMap::from([("EE", 3)]))),
            ("JJ", (21, HashMap::from([("AA", 2)]))),
        ])
    );

    assert_eq!(State::new().find_best(&data, "".to_string()), 1651);
}
