use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, recognize},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    Finish, IResult,
};

#[derive(Clone, Copy, Hash, PartialEq, Debug, PartialOrd, Eq, Ord)]
pub struct Pos(i32, i32);

fn decimal(input: &str) -> IResult<&str, i32> {
    map(recognize(many1(digit1)), |s: &str| {
        s.parse::<i32>().unwrap()
    })(input)
}

fn p_pos(s: &str) -> IResult<&str, Pos> {
    map(separated_pair(decimal, tag(","), decimal), |p| {
        Pos(p.0, p.1)
    })(s)
}

fn p_rock_structure(s: &str) -> IResult<&str, Vec<Pos>> {
    separated_list1(tag(" -> "), p_pos)(s)
}

pub fn parse_rock_structure(s: &str) -> Vec<Pos> {
    p_rock_structure(s).finish().unwrap().1
}

#[test]
fn test_parse_rock_structure() {
    assert_eq!(
        parse_rock_structure("498,4 -> 498,6 -> 496,6"),
        vec![Pos(498, 4), Pos(498, 6), Pos(496, 6),]
    );
    assert_eq!(
        parse_rock_structure("503,4 -> 502,4 -> 502,9 -> 494,9"),
        vec![Pos(503, 4), Pos(502, 4), Pos(502, 9), Pos(494, 9),]
    );
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaveBlock {
    data: HashMap<Pos, char>,
}

impl CaveBlock {
    fn drop_grain_of_sand(&mut self) -> Option<Pos> {
        let mut at = Pos(500, 0);

        if self.data.get(&at).is_some() {
            // blocked
            return None;
        }

        let ymax = self.data.keys().map(|p| p.1).max().unwrap();

        'fall: loop {
            if at.1 >= ymax {
                // flowing out
                return None;
            }

            for p in [
                Pos(at.0, at.1 + 1),
                Pos(at.0 - 1, at.1 + 1),
                Pos(at.0 + 1, at.1 + 1),
            ] {
                if self.data.get(&p) == None {
                    at = p;
                    continue 'fall;
                }
            }

            self.data.insert(at, 'o');
            return Some(at);
        }
    }

    pub fn drop_grains_until_done(&mut self) -> usize {
        let mut num = 0;

        while let Some(_) = self.drop_grain_of_sand() {
            num += 1;
        }

        num
    }

    pub fn drop_grains_until_hole_blocked(&mut self) -> usize {
        let floor = self.data.keys().map(|p| p.1).max().unwrap() + 2;

        println!("floor is at {}", floor);
        for x in (500 - floor - 10)..(500 + floor + 10) {
            self.data.insert(Pos(x, floor), '_');
        }

        self.drop_grains_until_done()
    }
}

pub fn parse(s: &str) -> CaveBlock {
    let structures: Vec<Vec<Pos>> = s.lines().map(|line| parse_rock_structure(line)).collect();

    let mut data = HashMap::new();

    for structure in structures {
        let mut a = structure[0];

        for &b in &structure[1..] {
            data.insert(a, '#');

            while a != b {
                let sx = (b.0 - a.0).signum();
                let sy = (b.1 - a.1).signum();
                a = Pos(a.0 + sx, a.1 + sy);

                data.insert(a, '#');
            }
        }
    }

    CaveBlock { data }
}

#[test]
fn test_parse() {
    let cave_block = parse(
        "498,4 -> 498,6 -> 496,6,
503,4 -> 502,4 -> 502,9 -> 494,9
",
    );

    assert_eq!(cave_block.data.len(), 20);
}

#[test]
fn test_dropping_grains_of_sand() {
    let cave_block = parse(
        "498,4 -> 498,6 -> 496,6,
503,4 -> 502,4 -> 502,9 -> 494,9
",
    );

    let mut c1 = cave_block.clone();

    assert_eq!(c1.drop_grain_of_sand(), Some(Pos(500, 8)));
    assert_eq!(c1.drop_grain_of_sand(), Some(Pos(499, 8)));
    assert_eq!(c1.drop_grain_of_sand(), Some(Pos(501, 8)));
    assert_eq!(c1.drop_grain_of_sand(), Some(Pos(500, 7)));
    assert_eq!(c1.drop_grain_of_sand(), Some(Pos(498, 8)));

    let mut c2 = cave_block.clone();

    assert_eq!(c2.drop_grains_until_done(), 24);
}

#[test]
fn test_dropping_grains_of_sand_v2() {
    let mut cave_block = parse(
        "498,4 -> 498,6 -> 496,6,
503,4 -> 502,4 -> 502,9 -> 494,9
",
    );

    assert_eq!(cave_block.drop_grains_until_hole_blocked(), 93);
}
