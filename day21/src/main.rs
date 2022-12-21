use either::Either::{self, Left, Right};
use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();
    let nodes = parse(&filecontents);

    let root = build(&nodes, "root", false);
    println!("Num: {}", root.eval());

    let humn = build_humn_expr(&nodes);
    println!("You should yell: {}", humn.eval());
}

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Humn,
    Num(u64),
    Op(char, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> u64 {
        match &self {
            Self::Num(n) => *n,
            Self::Op('+', a, b) => a.eval() + b.eval(),
            Self::Op('-', a, b) => a.eval() - b.eval(),
            Self::Op('*', a, b) => a.eval() * b.eval(),
            Self::Op('/', a, b) => a.eval() / b.eval(),
            _ => unreachable!(),
        }
    }

    fn contains_humn(&self) -> bool {
        match &self {
            Self::Humn => true,
            Self::Num(_) => false,
            Self::Op(_, a, b) => a.contains_humn() || b.contains_humn(),
        }
    }

    /// Recursively peel layers off of self, while building the other side of the equation, then return the other side
    fn simplify(self, other: Expr) -> Expr {
        match self {
            Self::Humn => other,
            Self::Op('/', a, b) if a.contains_humn() => {
                return a.simplify(Expr::Op('*', Box::new(other), b));
            }
            Self::Op('/', a, b) => {
                return b.simplify(Expr::Op('/', a, Box::new(other)));
            }
            Self::Op('*', a, b) if a.contains_humn() => {
                return a.simplify(Expr::Op('/', Box::new(other), b));
            }
            Self::Op('*', a, b) => {
                return b.simplify(Expr::Op('/', Box::new(other), a));
            }
            Self::Op('+', a, b) if a.contains_humn() => {
                return a.simplify(Expr::Op('-', Box::new(other), b));
            }
            Self::Op('+', a, b) => {
                return b.simplify(Expr::Op('-', Box::new(other), a));
            }
            Self::Op('-', a, b) if a.contains_humn() => {
                return a.simplify(Expr::Op('+', Box::new(other), b));
            }
            Self::Op('-', a, b) => {
                return b.simplify(Expr::Op('-', a, Box::new(other)));
            }
            _ => unreachable!(),
        }
    }
}

type ParsedNodes<'a> = HashMap<&'a str, Either<u64, (char, &'a str, &'a str)>>;

fn parse<'a>(s: &'a str) -> ParsedNodes<'a> {
    let expr_re = Regex::new(r"^([a-z]+): (?:([0-9]+)|([a-z]+) (.) ([a-z]+))$").unwrap();

    let mut nodes = HashMap::new();

    for line in s.lines() {
        let m = expr_re.captures(line).unwrap();
        let name = m.get(1).unwrap().as_str();

        if let Some(m) = m.get(2) {
            let num = m.as_str().parse::<u64>().unwrap();
            nodes.insert(name, Left(num));
        } else {
            let op = m.get(4).unwrap().as_str().chars().next().unwrap();
            let le = m.get(3).unwrap().as_str();
            let ri = m.get(5).unwrap().as_str();
            nodes.insert(name, Right((op, le, ri)));
        }
    }

    nodes
}

fn build<'a>(nodes: &ParsedNodes<'a>, name: &'a str, recognize_humn: bool) -> Expr {
    if recognize_humn && name == "humn" {
        return Expr::Humn;
    }

    match nodes[name] {
        Left(n) => Expr::Num(n),
        Right((op, le, ri)) => Expr::Op(
            op,
            Box::new(build(&nodes, le, recognize_humn)),
            Box::new(build(&nodes, ri, recognize_humn)),
        ),
    }
}

fn build_humn_expr<'a>(nodes: &ParsedNodes<'a>) -> Expr {
    let Right((_, a, b)) = nodes["root"] else {
        unreachable!()
    };

    let mut a = build(nodes, a, true);
    let mut b = build(nodes, b, true);

    if !a.contains_humn() {
        (b, a) = (a, b);
    }

    a.simplify(b)
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

    let nodes = parse(&s);

    let root = build(&nodes, "root", false);
    assert_eq!(root.eval(), 152);

    let humn = build_humn_expr(&nodes);
    assert_eq!(humn.eval(), 301);
}
