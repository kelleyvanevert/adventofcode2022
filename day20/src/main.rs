use std::fs;

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    let sum = solve(&filecontents);
    println!("Sum: {}", sum);
}

fn modulo(mut a: i32, m: i32) -> i32 {
    while a < 0 {
        a += m;
    }
    a % m
}

fn solve(s: &str) -> i32 {
    let mut data = s
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .enumerate()
        .collect::<Vec<(usize, i32)>>();

    let n = data.len();

    for k in 0..n {
        let i = data.iter().position(|&(i, _)| i == k).unwrap();
        let x = data[i];

        data.remove(i);

        let j = modulo((i as i32) + x.1, (n as i32) - 1) as usize;
        data.insert(j, x);
    }

    let i = data.iter().position(|&(_, n)| n == 0).unwrap();

    data[(i + 1000) % n].1 + data[(i + 2000) % n].1 + data[(i + 3000) % n].1
}

#[test]
fn test_all() {
    let s = "1
2
-3
3
-2
0
4
";

    assert_eq!(solve(s), 3);
}
