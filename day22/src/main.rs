use regex::Regex;
use std::{collections::HashMap, fs, time::Instant};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    time(|| {
        let pass = solve(&filecontents, false, false);
        println!("\nPassword: {}", pass);
        assert_eq!(pass, 149138);
    });

    time(|| {
        let pass_3d = solve(&filecontents, true, false);
        println!("\nPassword (3D mode): {}", pass_3d);
        assert_eq!(pass_3d, 153203);
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
type Dir = usize;

const DELTAS: [Pos; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn opposite(dir: Dir) -> Dir {
    (dir + 2) % 4
}

fn solve(s: &str, cube: bool, is_test_case: bool) -> i32 {
    let (map, path) = s.split_once("\n\n").unwrap();
    let mut path = path.trim();

    let width = map.lines().map(|line| line.len()).max().unwrap();

    let grid = map
        .lines()
        .map(|s| {
            let mut line = s.chars().collect::<Vec<char>>();
            line.resize(width, ' ');
            line
        })
        .collect::<Vec<Vec<char>>>();

    let height = grid.len();

    // inclusive
    let hbounds = (0..height)
        .map(|y| {
            let xmin = (0..width).find(|&x| grid[y][x] != ' ').unwrap_or(0);
            let xmax = (0..width).rev().find(|&x| grid[y][x] != ' ').unwrap_or(0);
            (xmin as i32, xmax as i32)
        })
        .collect::<Vec<(i32, i32)>>();

    // inclusive
    let vbounds = (0..width)
        .map(|x| {
            let ymin = (0..height).find(|&y| grid[y][x] != ' ').unwrap_or(0);
            let ymax = (0..height).rev().find(|&y| grid[y][x] != ' ').unwrap_or(0);
            (ymin as i32, ymax as i32)
        })
        .collect::<Vec<(i32, i32)>>();

    let mut teleport = HashMap::new();
    if is_test_case {
        for i in 0..4 {
            teleport.insert(((12 + i, 8), 3), ((11, 7 - i), 2)); // A
            teleport.insert(((11, i), 0), ((15, 11 - i), 2)); // B
            teleport.insert(((0, 4 + i), 2), ((15 - i, 11), 3)); // C
            teleport.insert(((4 + i, 4), 3), ((8, i), 0)); // D
            teleport.insert(((8 + i, 0), 3), ((3 - i, 4), 1)); // E
            teleport.insert(((4 + i, 7), 1), ((8, 11 - i), 0)); // F
            teleport.insert(((i, 7), 1), ((11 - i, 11), 3)); // G
        }
    } else {
        for i in 0..50 {
            teleport.insert(((50 + i, 0), 3), ((0, 150 + i), 0)); // A
            teleport.insert(((100 + i, 0), 3), ((i, 199), 3)); // B
            teleport.insert(((50, i), 2), ((0, 149 - i), 0)); // C
            teleport.insert(((50, 50 + i), 2), ((i, 100), 1)); // D
            teleport.insert(((100 + i, 49), 1), ((99, 50 + i), 2)); // E
            teleport.insert(((149, i), 0), ((99, 149 - i), 2)); // F
            teleport.insert(((50 + i, 149), 1), ((49, 150 + i), 2)); // G
        }
    }
    for ((source_pos, source_dir), (target_pos, target_dir)) in teleport.clone() {
        teleport.insert(
            (target_pos, opposite(target_dir)),
            (source_pos, opposite(source_dir)),
        );
    }

    let mut pos = (
        (0..width).position(|x| grid[1][x] == '.').unwrap() as i32,
        0,
    );
    let mut dir: usize = 0;

    let re_token = Regex::new(r"^(?:[0-9]+|R|L)").unwrap();
    while let Some(m) = re_token.captures(path) {
        path = &path[m[0].len()..];

        match &m[0] {
            "R" => {
                dir = (dir + 1) % 4;
            }
            "L" => {
                dir = (dir + 3) % 4;
            }
            steps => {
                let nsteps = steps.parse::<usize>().unwrap();

                for _ in 0..nsteps {
                    let d = DELTAS[dir];
                    let mut new_pos = (pos.0 + d.0, pos.1 + d.1);
                    let mut new_dir = dir;

                    if cube {
                        if let Some((teleport_pos, teleport_dir)) = teleport.get(&(pos, dir)) {
                            new_pos = *teleport_pos;
                            new_dir = *teleport_dir;
                        }
                    } else {
                        if dir == 0 && new_pos.0 > hbounds[new_pos.1 as usize].1 {
                            new_pos = (hbounds[new_pos.1 as usize].0, new_pos.1);
                        } else if dir == 1 && new_pos.1 > vbounds[new_pos.0 as usize].1 {
                            new_pos = (new_pos.0, vbounds[new_pos.0 as usize].0);
                        } else if dir == 2 && new_pos.0 < hbounds[new_pos.1 as usize].0 {
                            new_pos = (hbounds[new_pos.1 as usize].1, new_pos.1);
                        } else if dir == 3 && new_pos.1 < vbounds[new_pos.0 as usize].0 {
                            new_pos = (new_pos.0, vbounds[new_pos.0 as usize].1);
                        }
                    }

                    if grid[new_pos.1 as usize][new_pos.0 as usize] == '#' {
                        break;
                    } else if grid[new_pos.1 as usize][new_pos.0 as usize] == ' ' {
                        panic!("Ended up in invalid position {:?}", new_pos);
                    }

                    pos = new_pos;
                    dir = new_dir;
                }
            }
        }
    }

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + (dir as i32)
}

#[test]
fn test_all() {
    let s = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    assert_eq!(solve(s, false, true), 6032);

    assert_eq!(solve(s, true, true), 5031);
}
