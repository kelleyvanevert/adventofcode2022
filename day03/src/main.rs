use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_file_to_lines("./input.txt");
    println!("total: {}", solve(lines.clone()));
    println!("total v2: {}", solve_v2(lines));
}

fn read_file_to_lines(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    lines.into_iter().map(|line| line.unwrap()).collect()
}

fn solve(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .map(|line| to_priority(find_common(split(line))))
        .sum::<i32>()
}

fn solve_v2(lines: Vec<String>) -> i32 {
    lines
        .chunks(3)
        .into_iter()
        .map(|lines| {
            let lines: Vec<&str> = lines.into_iter().map(|line| line.as_str()).collect();
            let c = find_common(lines);
            to_priority(c)
        })
        .sum::<i32>()
}

fn to_priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        (c as i32) + 1 - ('a' as i32)
    } else if c.is_ascii_uppercase() {
        (c as i32) + 27 - ('A' as i32)
    } else {
        panic!()
    }
}

fn split(s: &str) -> Vec<&str> {
    let mid = s.len() / 2;
    vec![&s[0..mid], &s[mid..]]
}

fn find_common(s: Vec<&str>) -> char {
    for c in 'a'..'{' {
        if s.iter().all(|&s| s.contains(c)) {
            return c;
        }
    }
    for c in 'A'..'[' {
        if s.iter().all(|&s| s.contains(c)) {
            return c;
        }
    }

    panic!("Could not find common char for {:?}", s);
}

#[test]
fn test_find_common() {
    assert_eq!('p', find_common(split("vJrwpWtwJgWrhcsFMMfFFhFp")));
    assert_eq!('L', find_common(split("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")));
    assert_eq!('P', find_common(split("PmmdzqPrVvPwwTWBwg")));
    assert_eq!('v', find_common(split("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")));
    assert_eq!('t', find_common(split("ttgJtRGJQctTZtZT")));
    assert_eq!('s', find_common(split("CrZsJsPPZsGzwwsLwLmpwMDw")));
}

#[test]
fn test_to_priority() {
    assert_eq!(1, to_priority('a'));
    assert_eq!(2, to_priority('b'));
    assert_eq!(26, to_priority('z'));
    assert_eq!(27, to_priority('A'));
    assert_eq!(28, to_priority('B'));
    assert_eq!(52, to_priority('Z'));
}

#[test]
fn test_solve() {
    assert_eq!(
        157,
        solve(vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".into(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".into(),
            "PmmdzqPrVvPwwTWBwg".into(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".into(),
            "ttgJtRGJQctTZtZT".into(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".into(),
        ])
    );
}

#[test]
fn test_solve_v2() {
    assert_eq!(
        70,
        solve_v2(vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".into(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".into(),
            "PmmdzqPrVvPwwTWBwg".into(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".into(),
            "ttgJtRGJQctTZtZT".into(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".into(),
        ])
    );
}
