use std::{collections::HashSet, fs};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    println!("Steps: {}", solve(&filecontents));
}

fn solve(s: &str) -> usize {
    type Grid = Vec<Vec<char>>;
    type Pos = (usize, usize);

    let start: Grid = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '.' { vec![] } else { vec![c] })
        })
        .flatten()
        .collect();

    let height = s.lines().count();
    let width = start.len() / height;

    let is_border = |(x, y): Pos| x == 0 || x == width - 1 || y == 0 || y == height - 1;
    let index = |mut x, mut y| {
        if y == 0 {
            y = height - 2;
        }
        if y == height - 1 {
            y = 1;
        }
        if x == 0 {
            x = width - 2;
        }
        if x == width - 1 {
            x = 1;
        }
        y * width + x
    };

    let blow = |grid: &Grid| -> Grid {
        let mut next = vec![vec![]; width * height];

        for y in 0..height {
            for x in 0..width {
                if is_border((x, y)) {
                    next[y * width + x] = vec!['#'];
                } else {
                    for c in &grid[y * width + x] {
                        match c {
                            '>' => next[index(x + 1, y)].push('>'),
                            'v' => next[index(x, y + 1)].push('v'),
                            '<' => next[index(x - 1, y)].push('<'),
                            '^' => next[index(x, y - 1)].push('^'),
                            _ => (),
                        }
                    }
                }
            }
        }

        next
    };

    #[allow(unused)]
    let print = |grid: &Grid| -> String {
        (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let cs = &grid[y * width + x];
                        if cs.len() == 0 {
                            ".".to_string()
                        } else if *cs == vec!['#'] {
                            "#".to_string()
                        } else if cs.len() == 1 {
                            format!("{}", cs[0])
                        } else {
                            format!("{}", cs.len())
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    };

    let mut curr = start;
    let mut positions = HashSet::from([(1, 0)]);

    for minute in 0.. {
        curr = blow(&curr);
        let mut next_positions = HashSet::from([(1, 0)]);

        for &(x, y) in &positions {
            if (x, y) == (width - 2, height - 2) {
                return minute + 1;
            }

            // stay
            if curr[y * width + x].len() == 0 {
                next_positions.insert((x, y));
            }
            // move right
            if curr[y * width + x + 1].len() == 0 {
                next_positions.insert((x + 1, y));
            }
            // move down
            if curr[(y + 1) * width + x].len() == 0 {
                next_positions.insert((x, y + 1));
            }
            // move left
            if curr[y * width + x - 1].len() == 0 {
                next_positions.insert((x - 1, y));
            }
            // move up
            if y > 0 && curr[(y - 1) * width + x].len() == 0 {
                next_positions.insert((x, y - 1));
            }
        }

        positions = next_positions;
    }

    unreachable!()
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

    assert_eq!(solve(s), 18);
}
