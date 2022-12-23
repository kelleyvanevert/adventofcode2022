use std::io::{self, Write};
use std::{collections::HashSet, fs, iter::Cycle, time::Instant, vec::IntoIter};

fn main() {
    time(|| {
        println!();
        println!("First part");
        let mut rock_pattern = rocks().into_iter().cycle();
        let mut chamber = Chamber::new();

        simulate(
            &mut chamber,
            &mut get_jet_pattern(),
            &mut rock_pattern,
            2022,
        );

        let height = chamber.top() + 1;
        println!("height reached: {}", height);
        assert_eq!(height, 3127)
    });

    time(|| {
        println!();
        println!("Second part");
        let mut rock_pattern = rocks().into_iter().cycle();
        let mut chamber = Chamber::new();

        simulate(
            &mut chamber,
            &mut get_jet_pattern(),
            &mut rock_pattern,
            1_000_000_000_000,
        );

        let height = chamber.top() + 1;
        println!("height reached: {}", height);
    });

    println!();
}

fn get_jet_pattern() -> Cycle<IntoIter<i64>> {
    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '>' { 1 } else { -1 })
        .collect::<Vec<i64>>()
        .into_iter()
        .cycle()
}

fn time<F>(mut f: F)
where
    F: FnMut(),
{
    let t0 = Instant::now();
    f();
    println!("  took {:?}", t0.elapsed());
}

fn simulate<J, R>(
    chamber: &mut Chamber,
    jet_pattern: &mut J,
    rock_pattern: &mut R,
    num_rocks: usize,
) where
    J: Iterator<Item = i64>,
    R: Iterator<Item = Rock>,
{
    for i in 0..num_rocks {
        if i % 10_000 == 0 {
            print!("{} ", i);
            io::stdout().flush().unwrap();
            chamber.optimize();
        }

        let mut rock = rock_pattern.next().unwrap();

        rock.place_in_start_position(&chamber);
        // println!("New rock {} top {}", i, rock.top());
        // chamber.print(rock.top() as usize, &rock);

        loop {
            let dx = jet_pattern.next().unwrap();
            rock.try_move(&chamber, (dx, 0));
            // chamber.print(rock.top() as usize, &rock);

            let moved = rock.try_move(&chamber, (0, -1));
            // chamber.print(rock.top() as usize, &rock);

            if !moved {
                for &block in &rock.blocks {
                    chamber.occupied.insert(block);
                }
                break;
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Chamber {
    occupied: HashSet<Pos>,
}

impl Chamber {
    fn new() -> Self {
        Self {
            occupied: HashSet::new(),
        }
    }

    fn top(&self) -> i64 {
        if self.occupied.is_empty() {
            -1
        } else {
            self.occupied.iter().map(|p| p.1).max().unwrap()
        }
    }

    fn optimize(&mut self) {
        let top = self.top();
        self.occupied.retain(|&(_, y)| y >= top - 50);
    }

    // Has a bug that only surfaces with the actual data, but, too lazy to fix
    #[allow(unused)]
    fn print(&self, ymax: usize, rock: &Rock) {
        let mut data = vec![vec![".".to_string(); 9]; ymax + 3];

        data[0][0] = "+".to_string();
        data[0][8] = "+".to_string();
        for x in 1..=7 {
            data[0][x] = "-".to_string();
        }
        for y in 0..=ymax {
            data[y + 1][0] = "|".to_string();
            data[y + 1][8] = "|".to_string();
        }

        for &(x, y) in &self.occupied {
            data[y as usize + 1][x as usize + 1] = "X".to_string();
        }
        for &(x, y) in &rock.blocks {
            data[y as usize + 1][x as usize + 1] = "@".to_string();
        }

        println!(
            "{}",
            data.iter()
                .rev()
                .map(|line| line.join(""))
                .collect::<Vec<String>>()
                .join("\n")
        );
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
        self.blocks.iter().map(|p| p.1).max().unwrap()
    }

    fn place_in_start_position(&mut self, chamber: &Chamber) {
        let bottom = chamber.top() + 4;

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

fn rocks() -> Vec<Rock> {
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

    {
        let chamber = Chamber::new();
        let mut rock = rocks()[0].clone();
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
        let mut rock = rocks()[1].clone();
        rock.place_in_start_position(&chamber);
        assert_eq!(rock.blocks, vec![(3, 3), (2, 4), (3, 4), (4, 4), (3, 5)]);
    }

    {
        let mut jet_pattern = jet_pattern.clone().into_iter().cycle();
        let mut rock_pattern = rocks().into_iter().cycle();
        let mut chamber = Chamber::new();

        simulate(&mut chamber, &mut jet_pattern, &mut rock_pattern, 1);
        assert_eq!(chamber.top(), 0);

        simulate(&mut chamber, &mut jet_pattern, &mut rock_pattern, 1);
        assert_eq!(chamber.top(), 3);

        simulate(&mut chamber, &mut jet_pattern, &mut rock_pattern, 1);
        assert_eq!(chamber.top(), 5);

        simulate(&mut chamber, &mut jet_pattern, &mut rock_pattern, 1);
        assert_eq!(chamber.top(), 6);
    }

    {
        let mut jet_pattern = jet_pattern.clone().into_iter().cycle();
        let mut rock_pattern = rocks().into_iter().cycle();
        let mut chamber = Chamber::new();

        simulate(&mut chamber, &mut jet_pattern, &mut rock_pattern, 2022);
        assert_eq!(chamber.top() + 1, 3068);
    }

    {
        let mut jet_pattern = jet_pattern.into_iter().cycle();
        let mut rock_pattern = rocks().into_iter().cycle();
        let mut chamber = Chamber::new();

        simulate(
            &mut chamber,
            &mut jet_pattern,
            &mut rock_pattern,
            1000000000000,
        );
        assert_eq!(chamber.top() + 1, 1514285714288);
    }
}
