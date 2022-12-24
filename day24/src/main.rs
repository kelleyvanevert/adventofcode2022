use std::{collections::HashSet, fs, time::Instant};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    time(|| {
        let steps = solve(&filecontents, false);
        println!("Steps: {}", steps);
        assert_eq!(steps, 290);
    });

    time(|| {
        let steps = solve(&filecontents, true);
        println!("Steps, but circling back for forgotten snacks: {}", steps);
        assert_eq!(steps, 842);
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

fn solve(s: &str, circle_back_for_forgotten_snacks: bool) -> usize {
    type Grid = Vec<Vec<char>>;
    type Pos = (usize, usize);

    let lines = s.lines().collect::<Vec<&str>>();
    let lines = &lines[1..lines.len() - 1];
    let height = lines.len();
    let width = lines[0].len() - 2;

    let grid: Grid = lines
        .iter()
        .map(|&line| {
            line[1..(width + 1)]
                .chars()
                .map(|c| vec![c; if c == '.' { 0 } else { 1 }])
        })
        .flatten()
        .collect();

    let wrap = |(x, y): Pos| (x % width, y % height);
    let index = |(x, y): Pos| y * width + x;
    let empty_at = |grid: &Grid, (x, y): Pos| grid[index((x, y))].is_empty();

    let blow = |grid: Grid| -> Grid {
        let mut next = vec![vec![]; width * height];

        for y in 0..height {
            for x in 0..width {
                for c in &grid[index((x, y))] {
                    match c {
                        '>' => next[index(wrap((x + 1, y)))].push('>'),
                        'v' => next[index(wrap((x, y + 1)))].push('v'),
                        '<' => next[index(wrap((x + width - 1, y)))].push('<'),
                        '^' => next[index(wrap((x, y + height - 1)))].push('^'),
                        _ => (),
                    }
                }
            }
        }

        next
    };

    let find_shortest = |mut grid: Grid, start: Pos, end: Pos| -> (Grid, usize) {
        let mut positions = HashSet::new();

        for minute in 1.. {
            grid = blow(grid);

            let mut next_positions = HashSet::new();
            if empty_at(&grid, start) {
                // start!
                next_positions.insert(start);
            }

            for &(x, y) in &positions {
                if (x, y) == end {
                    return (grid, minute);
                }

                // stay
                if empty_at(&grid, (x, y)) {
                    next_positions.insert((x, y));
                }
                // move right
                if x < width - 1 && empty_at(&grid, (x + 1, y)) {
                    next_positions.insert((x + 1, y));
                }
                // move down
                if y < height - 1 && empty_at(&grid, (x, y + 1)) {
                    next_positions.insert((x, y + 1));
                }
                // move left
                if x > 0 && empty_at(&grid, (x - 1, y)) {
                    next_positions.insert((x - 1, y));
                }
                // move up
                if y > 0 && empty_at(&grid, (x, y - 1)) {
                    next_positions.insert((x, y - 1));
                }
            }

            positions = next_positions;
        }

        unreachable!()
    };

    let (grid, mut steps) = find_shortest(grid, (0, 0), (width - 1, height - 1));

    if circle_back_for_forgotten_snacks {
        let (grid, steps_2) = find_shortest(grid, (width - 1, height - 1), (0, 0));
        let (_, steps_3) = find_shortest(grid, (0, 0), (width - 1, height - 1));

        steps += steps_2 + steps_3;
    }

    steps
}

#[test]
fn test_all() {
    let s = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    assert_eq!(solve(s, false), 18);
    assert_eq!(solve(s, true), 54);
}
