use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
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

fn outcome(a: Shape, b: Shape) -> i32 {
    match (a, b) {
        (Shape::Rock, Shape::Scissors) => 6,
        (Shape::Scissors, Shape::Paper) => 6,
        (Shape::Paper, Shape::Rock) => 6,
        (a, b) if a == b => 3,
        (_, _) => 0,
    }
}

fn round_score(line: &str) -> i32 {
    let pieces: Vec<&str> = line.split(" ").collect();
    let opponent = pieces[0].parse::<Shape>().unwrap();
    let me = pieces[1].parse::<Shape>().unwrap();

    me.score() + outcome(me, opponent)
}

fn score_total(lines: Vec<String>) -> i32 {
    let total = lines.iter().map(|line| round_score(line)).sum::<i32>();

    total
}

#[test]
fn test_outcome() {
    assert_eq!(6, outcome(Shape::Rock, Shape::Scissors));
    assert_eq!(0, outcome(Shape::Rock, Shape::Paper));
    assert_eq!(3, outcome(Shape::Rock, Shape::Rock));
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
