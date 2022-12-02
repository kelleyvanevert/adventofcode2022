use std::{
    fs::File,
    io::{self, BufRead},
};

// rock = 1, paper = 2, scissors = 3

fn to_shape(s: &str) -> i32 {
    match s {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => panic!(),
    }
}

fn main() {
    let lines = read_file_to_lines("./input.txt");

    println!("total score: {}", score_total(lines));
}

fn read_file_to_lines(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    lines.into_iter().map(|line| line.unwrap()).collect()
}

fn outcome(a: i32, b: i32) -> i32 {
    match (a, b) {
        (1, 3) => 6,
        (3, 2) => 6,
        (2, 1) => 6,
        (a, b) if a == b => 3,
        (_, _) => 0,
    }
}

fn round_score(line: &str) -> i32 {
    let pieces: Vec<&str> = line.split(" ").collect();
    let opponent = to_shape(pieces[0]);
    let me = to_shape(pieces[1]);

    me + outcome(me, opponent)
}

fn score_total(lines: Vec<String>) -> i32 {
    let total = lines.iter().map(|line| round_score(line)).sum::<i32>();

    total
}

#[test]
fn test_outcome() {
    assert_eq!(6, outcome(1, 3));
    assert_eq!(0, outcome(1, 2));
    assert_eq!(3, outcome(1, 2));
}

#[test]
fn test_round_score() {
    assert_eq!(8, round_score("A Y"));
    assert_eq!(1, round_score("B X"));
    assert_eq!(6, round_score("C Z"));
}

#[test]
fn test_score() {
    assert_eq!(
        15,
        score_total(vec!["A Y".into(), "B X".into(), "C Z".into()])
    );
}
