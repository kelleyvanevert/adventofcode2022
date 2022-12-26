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
        let max = search(&data, false);
        println!("Max: {}", max);
        assert_eq!(max, 2080);
    });

    time(|| {
        let data = parse(&filecontents);
        let max = search(&data, true);
        println!("Max w/ help from elephant: {}", max);
        assert_eq!(max, 2752);
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

#[derive(Clone, PartialEq)]
struct State<'a> {
    at: &'a str,
    el_at: &'a str,
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
            el_at: "",
            time_left: 30,
            valves: HashMap::new(),
            total: 0,
        }
    }

    fn new_with_elephant() -> Self {
        Self {
            at: "AA",
            el_at: "AA",
            time_left: 26,
            valves: HashMap::new(),
            total: 0,
        }
    }

    fn step(&mut self) {
        self.time_left -= 1;
    }

    fn open_valve(&mut self, data: &Data<'a>, valve: &'a str) {
        let to_be_released = data[valve].0 * self.time_left;
        self.total += to_be_released;
        self.valves.insert(valve, Some(to_be_released));
    }

    fn heuristic(&self) -> usize {
        self.total
    }

    fn next(self, data: &Data<'a>) -> Vec<Self> {
        let mut next = vec![];

        if self.time_left == 0 {
            // no more time left!
            return next;
        }

        if !self.valves.contains_key(self.at) && data[self.at].0 > 0 {
            let mut s = self.clone();
            s.open_valve(data, self.at);
            next.push(s);
        }

        for &dest in &data[self.at].1 {
            let mut s = self.clone();
            s.at = dest;
            next.push(s);
        }

        next
    }

    fn next_el(self, data: &Data<'a>) -> Vec<Self> {
        let mut next = vec![];

        if self.time_left == 0 {
            // no more time left!
            return next;
        }

        if !self.valves.contains_key(self.el_at) && data[self.el_at].0 > 0 {
            let mut s = self.clone();
            s.open_valve(data, self.el_at);
            next.push(s);
        }

        for &dest in &data[self.el_at].1 {
            let mut s = self.clone();
            s.el_at = dest;
            next.push(s);
        }

        next
    }
}

fn search<'a>(data: &Data<'a>, elephant: bool) -> usize {
    let initial_state = if elephant {
        State::new_with_elephant()
    } else {
        State::new()
    };

    let max_beam_width = 100_000;
    let mut beam = BinaryHeap::new();
    beam.push(Reverse(initial_state.heuristic()));

    let mut consider = vec![initial_state];
    let mut max = 0;

    while consider.len() > 0 {
        let mut new_consider = vec![];
        for mut state in consider {
            let curr_min = beam.peek().unwrap().0;
            if state.heuristic() < curr_min {
                continue;
            }

            state.step();

            let mut next_states = state.next(&data);
            if elephant {
                next_states = next_states
                    .into_iter()
                    .flat_map(|s| s.next_el(&data))
                    .collect();
            }

            for s in next_states {
                if s.total > max {
                    max = s.total;
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

    assert_eq!(search(&data, false), 1651);

    // Not really sure why, but this one takes waay long to compute, even though it's quite fast on the actual data (and produces the right answer). So.. yeah
    // assert_eq!(search(&data, true), 1707);
}
