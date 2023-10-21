use std::{
    fs::File,
    io::{self, BufRead},
};

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const WIN: i32 = 6;
const LOSE: i32 = 0;
const DRAW: i32 = 3;

fn to_shape(s: &str) -> i32 {
    match s {
        "A" | "X" => ROCK,
        "B" | "Y" => PAPER,
        "C" | "Z" => SCISSORS,
        _ => panic!(),
    }
}

fn to_desired_outcome(s: &str) -> i32 {
    match s {
        "X" => LOSE,
        "Y" => DRAW,
        "Z" => WIN,
        _ => panic!(),
    }
}

fn main() {
    let lines = get_input()
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    println!("total score: {}", score_total(lines.clone(), round_score));
    println!(
        "total score (new rules): {}",
        score_total(lines.clone(), round_score_new_rules)
    );
}

fn outcome(me: i32, opponent: i32) -> i32 {
    match (me, opponent) {
        (ROCK, SCISSORS) => WIN,
        (SCISSORS, PAPER) => WIN,
        (PAPER, ROCK) => WIN,
        (a, b) if a == b => DRAW,
        (_, _) => LOSE,
    }
}

fn choose_play(opponent: i32, desired_outcome: i32) -> i32 {
    match desired_outcome {
        DRAW => opponent,
        WIN => match opponent {
            ROCK => PAPER,
            PAPER => SCISSORS,
            SCISSORS => ROCK,
            _ => panic!(),
        },
        LOSE => match opponent {
            ROCK => SCISSORS,
            PAPER => ROCK,
            SCISSORS => PAPER,
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn round_score(line: &str) -> i32 {
    let pieces: Vec<&str> = line.split(" ").collect();
    let opponent = to_shape(pieces[0]);
    let me = to_shape(pieces[1]);

    me + outcome(me, opponent)
}

fn round_score_new_rules(line: &str) -> i32 {
    let pieces: Vec<&str> = line.split(" ").collect();
    let opponent = to_shape(pieces[0]);
    let outcome = to_desired_outcome(pieces[1]);
    let me = choose_play(opponent, outcome);

    me + outcome
}

fn score_total(lines: Vec<String>, f: fn(line: &str) -> i32) -> i32 {
    let total = lines.iter().map(|line| f(line)).sum::<i32>();

    total
}

#[test]
fn test_outcome() {
    assert_eq!(WIN, outcome(ROCK, SCISSORS));
    assert_eq!(LOSE, outcome(ROCK, PAPER));
    assert_eq!(DRAW, outcome(ROCK, ROCK));
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
        score_total(vec!["A Y".into(), "B X".into(), "C Z".into()], round_score)
    );
    assert_eq!(
        12,
        score_total(
            vec!["A Y".into(), "B X".into(), "C Z".into()],
            round_score_new_rules
        )
    );
}

fn get_input() -> String {
    dotenv::dotenv().ok();
    let key = std::env::var("KEY").expect("Missing env var KEY");

    let bytes = std::fs::read("./input.txt.encrypted").unwrap();
    decrypt(key.as_bytes(), &bytes)
}

fn decrypt(key: &[u8], enc: &[u8]) -> String {
    String::from_utf8(
        enc.iter()
            .enumerate()
            .map(|(i, &b)| b.wrapping_sub(key[i % key.len()]))
            .collect(),
    )
    .unwrap()
}
