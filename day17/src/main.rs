#![feature(is_some_and)]

use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

fn main() {
    let jet_pattern = get_jet_pattern();
    let rock_pattern = get_rock_pattern();

    time(|| {
        println!();
        println!("First part");
        let mut chamber = Chamber::new();

        chamber.simulate(2022, &jet_pattern, &rock_pattern);

        let height = chamber.top();
        println!("height reached: {}", height);
        assert_eq!(height, 3127)
    });

    time(|| {
        println!();
        println!("Second part");
        let mut chamber = Chamber::new();

        chamber.simulate(1_000_000_000_000, &jet_pattern, &rock_pattern);

        let height = chamber.top();
        println!("height reached: {}", height);
        assert_eq!(height, 1542941176480);
    });

    println!();
}

fn get_jet_pattern() -> Vec<i64> {
    fs::read_to_string("./input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|c| if c == '>' { 1 } else { -1 })
        .collect::<Vec<i64>>()
}

fn time<F>(mut f: F)
where
    F: FnMut(),
{
    let t0 = Instant::now();
    f();
    println!("  took {:?}", t0.elapsed());
}

type HashKey = (Vec<Option<i64>>, usize, usize);

#[derive(Debug, Clone, PartialEq)]
struct StepInfo {
    rocks_settled: usize,
    top: i64,
}

#[derive(Clone, Debug, PartialEq)]
struct Chamber {
    occupied: HashSet<Pos>,
    tops: Vec<Option<i64>>,
    rocks_settled: usize,
    j: usize,
    r: usize,

    seen_before: HashMap<HashKey, StepInfo>,
}

impl Chamber {
    fn new() -> Self {
        Self {
            occupied: HashSet::new(),
            tops: vec![None; 7],
            rocks_settled: 0,
            j: 0,
            r: 0,

            seen_before: HashMap::new(),
        }
    }

    fn top(&self) -> i64 {
        self.tops
            .iter()
            .map(|&o| o.map_or(0, |y| y + 1))
            .max()
            .unwrap()
    }

    fn basic_bitch_hash(&self) -> HashKey {
        let top = self.top();

        let relative_tops = self
            .tops
            .iter()
            .map(|&o| o.map(|t| t - top))
            .collect::<Vec<Option<i64>>>();

        (relative_tops, self.j, self.r)
    }

    fn simulate(&mut self, mut num_rocks: usize, jet_pattern: &Vec<i64>, rock_pattern: &Vec<Rock>) {
        while num_rocks > 0 {
            let mut rock = rock_pattern[self.r].clone();
            self.r = (self.r + 1) % rock_pattern.len();

            rock.place_in_start_position(&self);

            loop {
                let dx = jet_pattern[self.j];
                self.j = (self.j + 1) % jet_pattern.len();
                rock.try_move(&self, (dx, 0));

                let moved = rock.try_move(&self, (0, -1));

                if !moved {
                    for &(x, y) in &rock.blocks {
                        self.occupied.insert((x, y));
                        self.tops[x as usize] = Some(self.tops[x as usize].map_or(y, |t| t.max(y)));
                    }
                    break;
                }
            }

            self.rocks_settled += 1;
            num_rocks -= 1;

            // ===
            // Optimization time!
            // Let's see if we've seen this top pattern before.
            // If so, instead of simulating, we can just jump ahead a whole bunch of steps!
            // ===
            let key = self.basic_bitch_hash();

            if let Some(info) = self.seen_before.get(&key) {
                let height_diff = self.top() - info.top;
                let added_rocks = self.rocks_settled - info.rocks_settled;

                let n = num_rocks / added_rocks;

                if n > 0 {
                    self.rocks_settled += added_rocks * n;

                    for o in self.tops.iter_mut() {
                        *o.as_mut().unwrap() += height_diff * (n as i64);
                    }

                    self.occupied = HashSet::from_iter(
                        self.tops
                            .iter()
                            .map(|o| o.unwrap())
                            .enumerate()
                            .map(|(x, y)| (x as i64, y)),
                    );

                    num_rocks -= added_rocks * n;
                }
            }

            self.seen_before.insert(
                key,
                StepInfo {
                    rocks_settled: self.rocks_settled,
                    top: self.top(),
                },
            );
        }
    }

    #[allow(unused)]
    fn viz(&self, rock: Option<&Rock>) -> String {
        let ymax = self.top().max(rock.map(|r| r.top()).unwrap_or(0));
        let grid = ((ymax - 12)..ymax)
            .rev()
            .map(|y| {
                (0..8)
                    .map(|x| {
                        if rock.is_some_and(|r| r.blocks.contains(&(x, y))) {
                            "o".to_string()
                        } else if x == 7 {
                            "|".to_string()
                        } else if self.occupied.contains(&(x, y)) {
                            "#".to_string()
                        } else {
                            " ".to_string()
                        }
                    })
                    .chain(std::iter::once(format!(" {}", y)))
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "rocks={}  j={}  r={}  tops={:?}\n{}",
            self.rocks_settled, self.j, self.r, self.tops, grid,
        )
    }
}

type Pos = (i64, i64);

#[derive(Clone, Debug, PartialEq)]
struct Rock {
    blocks: Vec<Pos>,
}

impl Rock {
    fn new(blocks: Vec<Pos>) -> Self {
        Self { blocks }
    }

    #[allow(unused)]
    fn top(&self) -> i64 {
        self.blocks.iter().map(|p| p.1).max().unwrap() + 1
    }

    fn place_in_start_position(&mut self, chamber: &Chamber) {
        let bottom = chamber.top() + 3;

        for p in &mut self.blocks {
            p.1 += bottom;
        }
    }

    fn try_move(&mut self, chamber: &Chamber, (dx, dy): (i64, i64)) -> bool {
        let mut updated = vec![];

        for &p in &self.blocks {
            let p = (p.0 + dx, p.1 + dy);
            if p.0 < 0 {
                return false;
            }
            if p.0 > 6 {
                return false;
            }
            if p.1 < 0 {
                return false;
            }
            if chamber.occupied.contains(&p) {
                return false;
            }

            updated.push(p);
        }

        self.blocks = updated;
        true
    }
}

fn get_rock_pattern() -> Vec<Rock> {
    vec![
        Rock::new(vec![(2, 0), (3, 0), (4, 0), (5, 0)]),
        Rock::new(vec![(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)]),
        Rock::new(vec![(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)]),
        Rock::new(vec![(2, 0), (2, 1), (2, 2), (2, 3)]),
        Rock::new(vec![(2, 0), (3, 0), (2, 1), (3, 1)]),
    ]
}

#[test]
fn test() {
    let jet_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
        .chars()
        .map(|c| if c == '>' { 1 } else { -1 })
        .collect::<Vec<i64>>();

    let rock_pattern = get_rock_pattern();

    {
        let chamber = Chamber::new();
        let mut rock = get_rock_pattern()[0].clone();
        rock.place_in_start_position(&chamber);
        assert_eq!(rock.blocks, vec![(2, 3), (3, 3), (4, 3), (5, 3)]);

        rock.try_move(&chamber, (-1, 0));
        assert_eq!(rock.blocks, vec![(1, 3), (2, 3), (3, 3), (4, 3)]);

        rock.try_move(&chamber, (-1, 0));
        rock.try_move(&chamber, (-1, 0));
        assert_eq!(rock.blocks, vec![(0, 3), (1, 3), (2, 3), (3, 3)]);
    }

    {
        let chamber = Chamber::new();
        let mut rock = get_rock_pattern()[1].clone();
        rock.place_in_start_position(&chamber);
        assert_eq!(rock.blocks, vec![(3, 3), (2, 4), (3, 4), (4, 4), (3, 5)]);
    }

    {
        let mut chamber = Chamber::new();

        chamber.simulate(1, &jet_pattern, &rock_pattern);
        assert_eq!(chamber.top(), 1);

        chamber.simulate(1, &jet_pattern, &rock_pattern);
        assert_eq!(chamber.top(), 4);

        chamber.simulate(1, &jet_pattern, &rock_pattern);
        assert_eq!(chamber.top(), 6);

        chamber.simulate(1, &jet_pattern, &rock_pattern);
        assert_eq!(chamber.top(), 7);
    }

    {
        let mut chamber = Chamber::new();

        chamber.simulate(2022, &jet_pattern, &rock_pattern);
        assert_eq!(chamber.top(), 3068);
    }

    {
        let mut chamber = Chamber::new();

        chamber.simulate(1000000000000, &jet_pattern, &rock_pattern);
        assert_eq!(chamber.top(), 1514285714288);
    }
}
