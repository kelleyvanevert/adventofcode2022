use std::{collections::HashSet, fs};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    let num_visited = solve(&filecontents);
    println!("Num visited: {}", num_visited);
}

type Pos = (i32, i32);

fn solve(s: &str) -> usize {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut head: Pos = (0, 0);
    let mut tail: Pos = (0, 0);

    visited.insert(tail);

    for line in s.lines() {
        if line == "" {
            continue;
        }

        let (direction, num) = line.split_once(" ").unwrap();
        let num = num.parse::<usize>().unwrap();

        for _ in 0..num {
            // move head
            match direction {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                _ => unreachable!(),
            }

            // move tail
            tail = move_towards(head, tail);

            // remember visited
            visited.insert(tail);
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

    assert_eq!(13, solve(s));
}
