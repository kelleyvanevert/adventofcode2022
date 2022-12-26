use regex::Regex;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    fs,
    time::Instant,
};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    time(|| {
        let data = parse(&filecontents);
        let max = search(&data);
        println!("Max: {}", max);
    });
}

fn time<F>(mut f: F)
where
    F: FnMut(),
{
    let t0 = Instant::now();
    f();
    println!("  took {:?}", t0.elapsed());
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

    fn heuristic(&self) -> usize {
        self.total_to_be_released()
    }

    fn total_to_be_released(&self) -> usize {
        self.valves.iter().map(|v| v.1.unwrap_or(0)).sum()
    }

    fn next(&self, data: &Data<'a>) -> Vec<Self> {
        let mut next = vec![];

        if self.time_left == 0 {
            // no more time left!
            return next;
        }

        if !self.valves.contains_key(self.at) && data[self.at].0 > 0 {
            // [x] only if total increases
            next.push(self.open_valve(data));
        }

        for (&dest, &dist) in &data[self.at].1 {
            // [x] prevent unnecessary move back

            match self.visited.get(dest) {
                None => {
                    // not visited before -> defo try
                    next.push(self.goto(dist, dest));
                }
                Some(&total) => {
                    // only try if
                    if total < self.total {
                        next.push(self.goto(dist, dest));
                    }
                }
            }
        }

        next
    }
}

fn search<'a>(data: &Data<'a>) -> usize {
    let max_beam_width = 1000;
    let mut beam = BinaryHeap::new();
    let initial = State::new();
    beam.push(Reverse(initial.heuristic()));

    let mut consider = vec![initial];
    let mut max = 0;

    while consider.len() > 0 {
        let mut new_consider = vec![];
        for state in consider {
            let curr_min = beam.peek().unwrap().0;
            if state.heuristic() < curr_min {
                continue;
            }

            for s in state.next(&data) {
                if s.total_to_be_released() > max {
                    max = s.total_to_be_released();
                }

                let my_min = s.heuristic();
                if beam.len() >= max_beam_width {
                    if my_min < curr_min {
                        // all of the 1000 best are at least `min`, and I'm no better
                        // -> discard this branch
                    } else {
                        beam.pop();
                        beam.push(Reverse(my_min));
                        new_consider.push(s);
                    }
                } else {
                    // less than 1000 in beam, so add regardless of score
                    beam.push(Reverse(my_min));
                    new_consider.push(s);
                }
            }
        }

        consider = new_consider;
    }

    max
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

    assert_eq!(search(&data), 1651);
}
