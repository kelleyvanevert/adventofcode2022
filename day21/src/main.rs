use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();
    println!("Num: {}", solve(&filecontents));
}

#[derive(Debug, PartialEq)]
enum Expr<'a> {
    Num(u64),
    Op(char, &'a str, &'a str),
}

impl<'a> Expr<'a> {
    fn eval(&self, nodes: &HashMap<&str, Expr>) -> u64 {
        match &self {
            Expr::Num(n) => *n,
            Expr::Op('+', a, b) => nodes[a].eval(&nodes) + nodes[b].eval(&nodes),
            Expr::Op('-', a, b) => nodes[a].eval(&nodes) - nodes[b].eval(&nodes),
            Expr::Op('*', a, b) => nodes[a].eval(&nodes) * nodes[b].eval(&nodes),
            Expr::Op('/', a, b) => nodes[a].eval(&nodes) / nodes[b].eval(&nodes),
            Expr::Op(..) => unreachable!(),
        }
    }
}

fn solve(s: &str) -> u64 {
    let expr_re = Regex::new(r"^([a-z]+): (?:([0-9]+)|([a-z]+) (.) ([a-z]+))$").unwrap();

    let mut nodes = HashMap::new();

    for line in s.lines() {
        let m = expr_re.captures(line).unwrap();
        let name = m.get(1).unwrap().as_str();

        if let Some(m) = m.get(2) {
            let num = m.as_str().parse::<u64>().unwrap();
            nodes.insert(name, Expr::Num(num));
        } else {
            let op = m.get(4).unwrap().as_str().chars().next().unwrap();
            let le = m.get(3).unwrap().as_str();
            let ri = m.get(5).unwrap().as_str();
            nodes.insert(name, Expr::Op(op, le, ri));
        }
    }

    nodes["root"].eval(&nodes)
}

#[test]
fn test_all() {
    let s = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    assert_eq!(solve(s), 152);
}
