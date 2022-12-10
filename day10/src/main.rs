use std::fs;

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    println!("Solution: {}", solve(&filecontents));
}

fn solve(s: &str) -> i32 {
    let mut x = 1;
    let mut i = 1;
    let mut accum = 0;

    let mut increase_cycle = |x: i32| {
        if (i + 20) % 40 == 0 {
            let signal_strength = i * x;
            accum += signal_strength;
        }

        i += 1;
    };

    for line in s.lines() {
        match &line[0..4] {
            "addx" => {
                increase_cycle(x);
                increase_cycle(x);
                let num = line[5..].parse::<i32>().unwrap();
                x += num;
            }
            "noop" => {
                increase_cycle(x);
            }
            _ => unreachable!(),
        }
    }

    // increase_cycle(x);

    accum
}

#[test]
fn test_solve() {
    let s = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    assert_eq!(13140, solve(s));
}
