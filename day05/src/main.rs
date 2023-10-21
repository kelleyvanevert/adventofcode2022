mod parse;

use crate::parse::parse;

fn main() {
    let filecontents = get_input();
    let (stacks, instructions) = parse(&filecontents);

    let mut stacks_a = stacks.clone();
    crane(&mut stacks_a, instructions.clone(), false);
    println!("first result: {}", top_crates(&stacks_a));

    let mut stacks_b = stacks.clone();
    crane(&mut stacks_b, instructions, true);
    println!("second result: {}", top_crates(&stacks_b));
}

fn crane(
    stacks: &mut Vec<Vec<String>>,
    instructions: Vec<(usize, usize, usize)>,
    bonus_rules: bool,
) {
    for (amount, source, destination) in instructions {
        if bonus_rules {
            let len = stacks[source - 1].len();

            let mut cs = stacks[source - 1]
                .splice((len - amount).., [])
                .collect::<Vec<String>>();

            stacks[destination - 1].append(&mut cs);
        } else {
            for _ in 0..amount {
                let c = stacks[source - 1].pop().unwrap();
                stacks[destination - 1].push(c);
            }
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

    let (stacks, instructions) = parse(str);

    let mut stacks_a = stacks.clone();
    crane(&mut stacks_a, instructions.clone(), false);
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
        stacks_a,
    );
    assert_eq!("CMZ".to_owned(), top_crates(&stacks_a));

    let mut stacks_b = stacks.clone();
    crane(&mut stacks_b, instructions, true);
    assert_eq!(
        vec![
            vec!["M".to_owned()],
            vec!["C".to_owned()],
            vec![
                "P".to_owned(),
                "Z".to_owned(),
                "N".to_owned(),
                "D".to_owned()
            ]
        ],
        stacks_b,
    );
    assert_eq!("MCD".to_owned(), top_crates(&stacks_b));
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
