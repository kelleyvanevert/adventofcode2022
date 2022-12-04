use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct AdventParser;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Assignment {
    pub start: i32,
    pub end: i32,
}

impl Assignment {
    #[allow(unused)]
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}

pub type AssignmentPair = (Assignment, Assignment);

fn parse_into_assignment(p: Pair<Rule>) -> Assignment {
    let mut p = p.into_inner();

    Assignment {
        start: p.next().unwrap().as_str().parse::<i32>().unwrap(),
        end: p.next().unwrap().as_str().parse::<i32>().unwrap(),
    }
}

pub fn parse_entry(s: &str) -> Vec<AssignmentPair> {
    let entry = AdventParser::parse(Rule::entry, s).unwrap().next().unwrap();

    let mut assignment_pairs: Vec<AssignmentPair> = vec![];

    for p in entry.into_inner() {
        match p.as_rule() {
            Rule::assignment_pair => {
                let mut p = p.into_inner();
                assignment_pairs.push((
                    parse_into_assignment(p.next().unwrap()),
                    parse_into_assignment(p.next().unwrap()),
                ));
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    assignment_pairs
}

#[test]
fn test_parse_entry() {
    assert_eq!(
        vec![
            (Assignment::new(2, 4), Assignment::new(6, 8)),
            (Assignment::new(2, 3), Assignment::new(4, 5)),
            (Assignment::new(5, 7), Assignment::new(7, 9)),
            (Assignment::new(2, 8), Assignment::new(3, 7)),
            (Assignment::new(6, 6), Assignment::new(4, 6)),
            (Assignment::new(2, 6), Assignment::new(4, 8))
        ],
        parse_entry(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
        )
    );
}
