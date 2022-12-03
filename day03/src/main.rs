use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_file_to_lines("./input.txt");
    println!("total: {}", solve(lines));
}

fn read_file_to_lines(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    lines.into_iter().map(|line| line.unwrap()).collect()
}

fn solve(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .map(|line| to_priority(find_common(line)))
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

fn find_common(s: &str) -> char {
    let mid = s.len() / 2;
    let le = &s[0..mid].chars().collect::<Vec<char>>();
    let ri = &s[mid..].chars().collect::<Vec<char>>();
    for c in le {
        if ri.contains(c) {
            return *c;
        }
    }

    panic!();
}

#[test]
fn test_find_common() {
    assert_eq!('p', find_common("vJrwpWtwJgWrhcsFMMfFFhFp"));
    assert_eq!('L', find_common("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
    assert_eq!('P', find_common("PmmdzqPrVvPwwTWBwg"));
    assert_eq!('v', find_common("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
    assert_eq!('t', find_common("ttgJtRGJQctTZtZT"));
    assert_eq!('s', find_common("CrZsJsPPZsGzwwsLwLmpwMDw"));
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
