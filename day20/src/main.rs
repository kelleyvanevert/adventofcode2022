use std::{fs, time::Instant};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();
    let data = parse(&filecontents);

    time(|| {
        let sum = solve(data.clone());
        println!("Sum: {}", sum);
        assert_eq!(sum, 4151);
    });

    time(|| {
        let sum = solve_v2(data.clone());
        println!("Sum v2: {}", sum);
        assert_eq!(sum, 7848878698663);
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

fn modulo(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

fn parse(s: &str) -> Vec<(usize, i64)> {
    s.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .enumerate()
        .collect::<Vec<(usize, i64)>>()
}

fn mix(mut data: Vec<(usize, i64)>) -> Vec<(usize, i64)> {
    let n = data.len();

    for k in 0..n {
        let i = data.iter().position(|&(i, _)| i == k).unwrap();
        let x = data[i];

        data.remove(i);

        let j = modulo((i as i64) + x.1, (n as i64) - 1) as usize;
        data.insert(j, x);
    }

    data
}

fn solve(data: Vec<(usize, i64)>) -> i64 {
    let data = mix(data);
    let n = data.len();

    let i = data.iter().position(|&(_, n)| n == 0).unwrap();

    data[(i + 1000) % n].1 + data[(i + 2000) % n].1 + data[(i + 3000) % n].1
}

const DECRYPT: i64 = 811589153;

fn solve_v2(data: Vec<(usize, i64)>) -> i64 {
    let n = data.len();
    let mut data = data.into_iter().map(|p| (p.0, DECRYPT * p.1)).collect();

    for _ in 1..=10 {
        data = mix(data);
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

    assert_eq!(solve(parse(s)), 3);
    assert_eq!(solve_v2(parse(s)), 1623178306);
}
