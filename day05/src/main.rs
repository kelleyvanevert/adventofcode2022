mod parse;

use crate::parse::parse;
use std::fs;

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();
    let (mut stacks, instructions) = parse(&filecontents);

    crane(&mut stacks, instructions);
    println!("{}", top_crates(&stacks));
}

fn crane(stacks: &mut Vec<Vec<String>>, instructions: Vec<(usize, usize, usize)>) {
    for (amount, source, destination) in instructions {
        // println!("instr {}, {}, {}", amount, source, destination);
        for _ in 0..amount {
            let c = stacks[source - 1].pop().unwrap();
            // println!(" - {} {}", i, c);
            stacks[destination - 1].push(c);
            // println!("   - {:?}", stacks);
        }
    }
}

fn top_crates(stacks: &Vec<Vec<String>>) -> String {
    stacks
        .iter()
        .map(|stack| {
            let j = stack.last().unwrap();
            j.as_str()
        })
        .collect::<Vec<&str>>()
        .join("")
}

#[test]
fn test_crane() {
    let str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    let (mut stacks, instructions) = parse(str);
    crane(&mut stacks, instructions);

    assert_eq!(
        vec![
            vec!["C".to_owned()],
            vec!["M".to_owned()],
            vec![
                "P".to_owned(),
                "D".to_owned(),
                "N".to_owned(),
                "Z".to_owned()
            ]
        ],
        stacks,
    );

    assert_eq!("CMZ".to_owned(), top_crates(&stacks));
}
