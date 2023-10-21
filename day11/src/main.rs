use std::str::FromStr;

use derive_builder::Builder;
use regex::Regex;

fn main() {
    let filecontents = get_input();
    println!("Monkey business: {}", solve(&parse(&filecontents), false));
    println!("Monkey business v2: {}", solve(&parse(&filecontents), true));
}

#[derive(Debug, Clone, PartialEq)]
enum MiniNode {
    Num(usize),
    Old,
}

impl MiniNode {
    fn eval(&self, old: usize) -> usize {
        match self {
            Self::Old => old,
            Self::Num(num) => *num,
        }
    }
}

impl FromStr for MiniNode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(MiniNode::Old),
            s => Ok(MiniNode::Num(s.parse::<usize>().unwrap())),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MiniExpr {
    op: char,
    lhs: MiniNode,
    rhs: MiniNode,
}

impl MiniExpr {
    fn eval(&self, old: usize) -> usize {
        let le = self.lhs.eval(old);
        let ri = self.rhs.eval(old);
        match self.op {
            '*' => le * ri,
            '+' => le + ri,
            _ => unreachable!(),
        }
    }
}

impl FromStr for MiniExpr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_op_expr = Regex::new(r"^(old|[0-9]+) ([+*]) (old|[0-9]+)$").unwrap();
        let m = re_op_expr.captures(s).expect("Operation syntax error");

        Ok(MiniExpr {
            op: m.get(2).unwrap().as_str().chars().next().unwrap(),
            lhs: m.get(1).unwrap().as_str().parse().unwrap(),
            rhs: m.get(3).unwrap().as_str().parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Builder)]
struct Monkey {
    items: Vec<usize>,
    op_expr: MiniExpr,
    div_check: usize,
    dest_if_true: usize,
    dest_if_false: usize,
}

fn parse(s: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut building = false;
    let mut builder = MonkeyBuilder::default();

    for line in s.lines() {
        if line.starts_with("Monkey ") {
            builder = MonkeyBuilder::default();
            building = true;
        } else if line.starts_with("  Starting items: ") {
            builder.items(
                line[18..]
                    .split(", ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect(),
            );
        } else if line.starts_with("  Operation: new = ") {
            builder.op_expr(line[19..].parse().unwrap());
        } else if line.starts_with("  Test: divisible by ") {
            builder.div_check(line[21..].parse::<usize>().unwrap());
        } else if line.starts_with("    If true: throw to monkey ") {
            builder.dest_if_true(line[29..].parse::<usize>().unwrap());
        } else if line.starts_with("    If false: throw to monkey ") {
            builder.dest_if_false(line[30..].parse::<usize>().unwrap());
        } else if line == "" && building {
            monkeys.push(builder.build().expect("Monkey building not complete"));
            building = false;
        } else {
            unreachable!("Unknown line syntax");
        }
    }

    if building {
        monkeys.push(builder.build().expect("Monkey building not complete"));
    }

    monkeys
}

fn solve(monkeys: &Vec<Monkey>, new_rules: bool) -> usize {
    let mut monkeys = monkeys.clone();
    let mut activity = vec![0usize; monkeys.len()];

    let m = monkeys.iter().fold(1, |a, m| a * m.div_check);

    let num_rounds = if new_rules { 10_000 } else { 20 };

    for _round in 0..num_rounds {
        for i in 0..monkeys.len() {
            for old in monkeys[i].items.clone() {
                let mut new = monkeys[i].op_expr.eval(old);
                if new_rules {
                    new = new % m;
                } else {
                    new = new / 3;
                }
                let dest = if new % monkeys[i].div_check == 0 {
                    monkeys[i].dest_if_true
                } else {
                    monkeys[i].dest_if_false
                };
                monkeys[dest].items.push(new);
            }

            activity[i] += monkeys[i].items.len();
            monkeys[i].items = vec![];
        }
    }

    activity.sort();
    activity.reverse();

    activity[0] * activity[1]
}

#[test]
fn test_solve() {
    let s = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    let monkeys = parse(s);

    assert_eq!(
        monkeys,
        vec![
            Monkey {
                items: vec![79, 98],
                op_expr: MiniExpr {
                    op: '*',
                    lhs: MiniNode::Old,
                    rhs: MiniNode::Num(19)
                },
                div_check: 23,
                dest_if_true: 2,
                dest_if_false: 3,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                op_expr: MiniExpr {
                    op: '+',
                    lhs: MiniNode::Old,
                    rhs: MiniNode::Num(6)
                },
                div_check: 19,
                dest_if_true: 2,
                dest_if_false: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                op_expr: MiniExpr {
                    op: '*',
                    lhs: MiniNode::Old,
                    rhs: MiniNode::Old
                },
                div_check: 13,
                dest_if_true: 1,
                dest_if_false: 3,
            },
            Monkey {
                items: vec![74],
                op_expr: MiniExpr {
                    op: '+',
                    lhs: MiniNode::Old,
                    rhs: MiniNode::Num(3)
                },
                div_check: 17,
                dest_if_true: 0,
                dest_if_false: 1,
            }
        ]
    );

    assert_eq!(10605, solve(&monkeys, false));
    assert_eq!(2713310158, solve(&monkeys, true));
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
