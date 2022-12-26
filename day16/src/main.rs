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
    // el_at: &'a str,
    time_left: usize,
    valves: HashMap<&'a str, Option<usize>>,
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
            total: 0,
        }
    }

    fn open_valve(&self, data: &Data<'a>) -> State<'a> {
        let mut valves = self.valves.clone();

        let pressure_to_be_released = data[self.at].0 * (self.time_left - 1);
        let total = self.total + pressure_to_be_released;

        valves.insert(self.at, Some(pressure_to_be_released));

        Self {
            at: self.at,
            time_left: self.time_left - 1,
            valves,
            total,
        }
    }

    fn goto(&self, dest: &'a str) -> State<'a> {
        Self {
            at: dest,
            time_left: self.time_left.saturating_sub(1),
            valves: self.valves.clone(),
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
            next.push(self.open_valve(data));
        }

        for &dest in &data[self.at].1 {
            next.push(self.goto(dest));
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

    assert_eq!(search(&data), 1651);
}
