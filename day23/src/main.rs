use std::io::{self, Write};
use std::{collections::HashMap, fs, time::Instant};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    time(|| {
        println!();
        println!("Empty space after 10 steps:");
        println!("{}", solve(&filecontents));
    });

    time(|| {
        println!();
        println!("Number of steps until completion:");
        println!("{}", solve_v2(&filecontents));
    });

    println!();
}

fn time<F>(mut f: F)
where
    F: FnMut(),
{
    let t0 = Instant::now();
    f();
    println!("  took {:?}", t0.elapsed());
}

type Pos = (i32, i32);

#[derive(Debug, PartialEq, Clone)]
struct Grid {
    elves: Vec<Pos>,
}

impl Grid {
    fn new(s: &str) -> Self {
        let mut elves = vec![];

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    elves.push((x as i32, y as i32));
                }
            }
        }

        Self { elves }
    }

    fn empty_at(&self, x: i32, y: i32) -> bool {
        !self.elves.contains(&(x, y))
    }

    fn propose_elf_move(&self, (x, y): Pos, step_no: usize) -> Option<Pos> {
        if [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .into_iter()
        .all(|(x, y)| self.empty_at(x, y))
        {
            return None;
        }

        let mut proposals = vec![];

        proposals.push(
            if self.empty_at(x, y - 1) && self.empty_at(x - 1, y - 1) && self.empty_at(x + 1, y - 1)
            {
                Some((x, y - 1)) // north
            } else {
                None
            },
        );
        proposals.push(
            if self.empty_at(x, y + 1) && self.empty_at(x - 1, y + 1) && self.empty_at(x + 1, y + 1)
            {
                Some((x, y + 1)) // south
            } else {
                None
            },
        );
        proposals.push(
            if self.empty_at(x - 1, y) && self.empty_at(x - 1, y + 1) && self.empty_at(x - 1, y - 1)
            {
                Some((x - 1, y)) // west
            } else {
                None
            },
        );
        proposals.push(
            if self.empty_at(x + 1, y) && self.empty_at(x + 1, y + 1) && self.empty_at(x + 1, y - 1)
            {
                Some((x + 1, y)) // east
            } else {
                None
            },
        );

        (0..4).find_map(|i| proposals[(step_no + i) % 4])
    }

    fn step(&mut self, step_no: usize) -> bool {
        print!(".");
        io::stdout().flush().unwrap();

        let proposed = self
            .elves
            .iter()
            .map(|&elf| self.propose_elf_move(elf, step_no))
            .collect::<Vec<Option<Pos>>>();

        if proposed.iter().all(|p| p.is_none()) {
            return true;
        }

        let mut counts = HashMap::new();
        for pos in proposed.iter() {
            if let Some(pos) = pos {
                counts.entry(pos).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        for (i, &pos) in proposed.iter().enumerate() {
            if let Some(pos) = pos {
                if *counts.get(&pos).unwrap_or(&1) < 2 {
                    self.elves[i] = pos;
                }
            }
        }

        return false;
    }

    fn answer(&self) -> usize {
        let xmin = self.elves.iter().map(|p| p.0).min().unwrap();
        let xmax = self.elves.iter().map(|p| p.0).max().unwrap();
        let ymin = self.elves.iter().map(|p| p.1).min().unwrap();
        let ymax = self.elves.iter().map(|p| p.1).max().unwrap();
        let surface = (ymax - ymin + 1) * (xmax - xmin + 1);
        (surface as usize) - self.elves.len()
    }

    #[allow(unused)]
    fn print(&self) -> String {
        let xmin = self.elves.iter().map(|p| p.0).min().unwrap();
        let xmax = self.elves.iter().map(|p| p.0).max().unwrap();
        let ymin = self.elves.iter().map(|p| p.1).min().unwrap();
        let ymax = self.elves.iter().map(|p| p.1).max().unwrap();
        (ymin.min(0)..=ymax)
            .map(|y| {
                (xmin.min(0)..=xmax)
                    .map(|x| {
                        if self.elves.contains(&(x, y)) {
                            "#".to_string()
                        } else {
                            ".".to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn solve(s: &str) -> usize {
    let mut grid = Grid::new(s);

    for step_no in 0..10 {
        grid.step(step_no);
    }

    grid.answer()
}

fn solve_v2(s: &str) -> usize {
    let mut grid = Grid::new(s);

    (0..).position(|step_no| grid.step(step_no)).unwrap() + 1
}

#[test]
fn test_all() {
    let s = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    assert_eq!(solve(s), 110);

    assert_eq!(solve_v2(s), 20);
}
