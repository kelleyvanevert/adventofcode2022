use std::{collections::HashSet, fs};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    println!("Num visited: {}", solve(&filecontents, 2));
    println!("Num visited v2: {}", solve(&filecontents, 10));
}

type Pos = (i32, i32);

fn solve(s: &str, len: usize) -> usize {
    assert!(len >= 2);

    let mut visited: HashSet<Pos> = HashSet::new();
    let mut rope: Vec<Pos> = vec![(0, 0); len]; // head first

    visited.insert(*rope.last().unwrap());

    for line in s.lines() {
        if line == "" {
            continue;
        }

        let (direction, num) = line.split_once(" ").unwrap();
        let num = num.parse::<usize>().unwrap();

        for _ in 0..num {
            // move head
            match direction {
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                _ => unreachable!(),
            }

            // move tail
            for i in 0..(rope.len() - 1) {
                rope[i + 1] = move_towards(rope[i], rope[i + 1]);
            }

            // remember visited
            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn move_towards(head: Pos, tail: Pos) -> Pos {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        // no need to move
        return tail;
    }

    // a bit weirdly expressed, but I'm using the fact that (3/2) rounds to 1 to express the movement in a single equation
    return (
        tail.0 + (dx + dx.signum()) / 2,
        tail.1 + (dy + dy.signum()) / 2,
    );
}

#[test]
fn test_solve() {
    let s = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    assert_eq!(13, solve(s, 2));

    assert_eq!(1, solve(s, 10));
}
