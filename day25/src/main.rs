use memoize::memoize;
use std::{fs, time::Instant};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    time(|| {
        println!(
            "Total in SNAFU: {}",
            into_snafu(filecontents.lines().map(from_snafu).sum::<i64>())
        );
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

fn digit_from_snafu(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("not a SNAFU digit"),
    }
}

fn digit_into_snafu(d: i64) -> char {
    match d {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!("not a SNAFU-able digit"),
    }
}

#[memoize]
fn fem_pow(i: usize) -> i64 {
    (5 as i64).pow(i as u32)
}

fn from_snafu(s: &str) -> i64 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| fem_pow(i) * digit_from_snafu(c))
        .sum()
}

fn into_snafu(mut n: i64) -> String {
    let mut i = (1_usize..)
        .position(|i| {
            let b = fem_pow(i) / 2;
            (-b <= n) && (n <= b)
        })
        .unwrap();

    let mut chars = vec![];

    while i > 0 {
        let p = fem_pow(i);
        let b = p / 2;
        if (-b <= n) && (n <= b) {
            chars.push('0');
        } else {
            let diff = n - b * n.signum();
            let d = diff.signum() + (diff - diff.signum()) / p;
            chars.push(digit_into_snafu(d));
            n = n - d * p;
        }

        i -= 1;
    }

    chars.push(digit_into_snafu(n));

    chars.into_iter().collect()
}

#[test]
fn test() {
    let s = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    assert_eq!(s.lines().map(from_snafu).sum::<i64>(), 4890);

    assert_eq!(
        s.lines()
            .map(from_snafu)
            .map(into_snafu)
            .collect::<Vec<String>>(),
        s.lines().map(|s| s.to_string()).collect::<Vec<String>>()
    );

    assert_eq!(
        into_snafu(s.lines().map(from_snafu).sum::<i64>()),
        "2=-1=0".to_string()
    );
}
