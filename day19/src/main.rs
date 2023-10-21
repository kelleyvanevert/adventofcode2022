use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use std::{cmp::Reverse, collections::BinaryHeap, fmt::Debug, time::Instant};

const DEBUG: bool = false;

fn main() {
    let filecontents = get_input();
    let blueprints = parse(&filecontents);

    time(|| {
        let total_quality = blueprints
            .par_iter()
            .map(|(id, blueprint)| {
                let num_geodes = find_max(blueprint, 24);
                let quality = id * num_geodes;
                println!(
                    "Blueprint {} can make max. {} geodes, quality: {}",
                    id, num_geodes, quality
                );
                quality
            })
            .sum::<usize>();

        println!("Total quality (checksum): {}", total_quality);
        assert_eq!(total_quality, 988);
    });

    time(|| {
        let geodes_multiplied = blueprints[0..3]
            .par_iter()
            .map(|(id, blueprint)| {
                let num_geodes = find_max(blueprint, 32);
                println!("Blueprint {} can make max. {} geodes", id, num_geodes);
                num_geodes
            })
            .reduce(|| 1, |a, b| a * b);

        println!("Total geodes, multiplied: {}", geodes_multiplied);
        assert_eq!(geodes_multiplied, 8580);
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

#[derive(Debug, Clone, PartialEq)]
struct Blueprint {
    ore_for_ore_robot: usize,
    ore_for_clay_robot: usize,
    ore_for_obs_robot: usize,
    clay_for_obs_robot: usize,
    ore_for_geode_robot: usize,
    obs_for_geode_robot: usize,
}

impl From<Vec<usize>> for Blueprint {
    fn from(p: Vec<usize>) -> Self {
        Self {
            ore_for_ore_robot: p[0],
            ore_for_clay_robot: p[1],
            ore_for_obs_robot: p[2],
            clay_for_obs_robot: p[3],
            ore_for_geode_robot: p[4],
            obs_for_geode_robot: p[5],
        }
    }
}

#[derive(Clone)]
struct State {
    time_left: usize,

    ore_robots: usize,
    clay_robots: usize,
    obs_robots: usize,
    geode_robots: usize,

    ore: usize,
    clay: usize,
    obs: usize,
    geode: usize,

    // for debugging purposes
    trace: Vec<[usize; 9]>,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "State({}) [{},{},{},{}] collected [{},{},{},{}]",
            self.time_left,
            self.ore_robots,
            self.clay_robots,
            self.obs_robots,
            self.geode_robots,
            self.ore,
            self.clay,
            self.obs,
            self.geode,
        ))
    }
}

impl State {
    fn new(time_available: usize) -> Self {
        let mut me = Self {
            time_left: time_available,

            ore_robots: 1,
            clay_robots: 0,
            obs_robots: 0,
            geode_robots: 0,

            ore: 0,
            clay: 0,
            obs: 0,
            geode: 0,

            trace: vec![],
        };

        me.log();

        me
    }

    fn log(&mut self) {
        if DEBUG {
            self.trace.push([
                self.time_left,
                self.ore_robots,
                self.clay_robots,
                self.obs_robots,
                self.geode_robots,
                self.ore,
                self.clay,
                self.obs,
                self.geode,
            ]);
        }
    }

    fn step(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obs += self.obs_robots;
        self.geode += self.geode_robots;
        self.time_left = self.time_left.saturating_sub(1);
    }

    fn next(mut self, blueprint: &Blueprint) -> Vec<Self> {
        let mut next = vec![];

        if self.time_left == 0 {
            // no more time left!
            return next;
        }

        // build new geode robot?
        if self.ore >= blueprint.ore_for_geode_robot && self.obs >= blueprint.obs_for_geode_robot {
            let mut s = self.clone();
            s.step();
            s.geode_robots += 1;
            s.ore = s.ore.saturating_sub(blueprint.ore_for_geode_robot);
            s.obs = s.obs.saturating_sub(blueprint.obs_for_geode_robot);
            s.log();
            next.push(s);
        }

        // build new obs robot?
        if self.ore >= blueprint.ore_for_obs_robot && self.clay >= blueprint.clay_for_obs_robot {
            let mut s = self.clone();
            s.step();
            s.obs_robots += 1;
            s.ore = s.ore.saturating_sub(blueprint.ore_for_obs_robot);
            s.clay = s.clay.saturating_sub(blueprint.clay_for_obs_robot);
            s.log();
            next.push(s);
        }

        // build new clay robot?
        if self.ore >= blueprint.ore_for_clay_robot {
            let mut s = self.clone();
            s.step();
            s.clay_robots += 1;
            s.ore = s.ore.saturating_sub(blueprint.ore_for_clay_robot);
            s.log();
            next.push(s);
        }

        // build new ore robot?
        if self.ore >= blueprint.ore_for_ore_robot {
            let mut s = self.clone();
            s.step();
            s.ore_robots += 1;
            s.ore = s.ore.saturating_sub(blueprint.ore_for_ore_robot);
            s.log();
            next.push(s);
        }

        self.step();
        self.log();
        next.push(self);

        next
    }

    /// A heuristic for how good this branch is, in terms of how many geodes it'll be able to produce, or something.. I tried more complex (read: more reasonable) things, but this simplified heuristic apparently works best? :P Oh well, that's the idea of a heuristic, I guess.. A carefully fine-tuned but kinda weird thing.
    fn heuristic(&self) -> usize {
        let clay_final = self.clay + self.clay_robots * self.time_left;
        let obs_final = self.obs + self.obs_robots * self.time_left;
        let geode_final = self.geode + self.geode_robots * self.time_left;

        geode_final * 1000 + obs_final * 10 + clay_final
    }
}

fn parse(s: &str) -> Vec<(usize, Blueprint)> {
    let r_num = Regex::new(r"[0-9]+").unwrap();

    s.lines()
        .enumerate()
        .map(|(i, line)| {
            let id = i + 1;
            let blueprint: Blueprint = r_num
                .find_iter(line.split_once(": ").unwrap().1)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .into();

            (id, blueprint)
        })
        .collect::<Vec<(usize, Blueprint)>>()
}

/// I'm going for the BFS + beam search solution that someone suggested on Reddit.
fn find_max(blueprint: &Blueprint, time_available: usize) -> usize {
    let max_beam_width = 1000;
    let mut beam = BinaryHeap::new();
    let init = State::new(time_available);
    beam.push(Reverse(init.heuristic()));

    let mut consider = vec![init];
    let mut max = 0;
    let mut max_trace = vec![];

    let mut i = 0;
    while consider.len() > 0 {
        i += 1;
        if DEBUG {
            println!(
                "[{}] Considering {} (curr max {}, beam min {}, beam width {})",
                i,
                consider.len(),
                max,
                beam.peek().unwrap().0,
                beam.len()
            );
        }

        let mut new_consider = vec![];
        for state in consider {
            let curr_min = beam.peek().unwrap().0;
            if state.heuristic() < curr_min {
                continue;
            }

            for s in state.next(&blueprint) {
                if s.geode > max {
                    max = s.geode;
                    max_trace = s.trace.clone();
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

    if DEBUG {
        println!(
            "MAX: {} for decisions:\n{}",
            max,
            max_trace
                .iter()
                .map(|p| { format!("{:?}", p) })
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    max
}

#[test]
fn test_all() {
    let s = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    let blueprints = parse(s);

    assert_eq!(blueprints.len(), 2);
    assert_eq!(
        blueprints[0].1,
        Blueprint {
            ore_for_ore_robot: 4,
            ore_for_clay_robot: 2,
            ore_for_obs_robot: 3,
            clay_for_obs_robot: 14,
            ore_for_geode_robot: 2,
            obs_for_geode_robot: 7,
        }
    );

    assert_eq!(find_max(&blueprints[0].1, 24), 9);
    assert_eq!(find_max(&blueprints[1].1, 24), 12);
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
