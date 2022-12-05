use regex::Regex;

pub type Stack = Vec<String>;
pub type Instruction = (usize, usize, usize);

pub fn parse(s: &str) -> (Vec<Stack>, Vec<Instruction>) {
    let re_stack_layer = Regex::new(r"(?:\[([A-Z])\]|   )(?: |$)").unwrap();
    let re_inst = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();

    let mut stacks: Vec<Stack> = vec![];
    let mut instructions: Vec<Instruction> = vec![];

    for line in s.lines() {
        if line == "" {
            // divider
        } else if line.starts_with("move") {
            let m = re_inst.captures(line).unwrap();
            instructions.push((
                m[1].parse::<usize>().unwrap(),
                m[2].parse::<usize>().unwrap(),
                m[3].parse::<usize>().unwrap(),
            ));
        } else {
            for (i, m) in re_stack_layer.captures_iter(line).enumerate() {
                if stacks.len() <= i {
                    stacks.push(vec![]);
                }
                if let Some(m2) = m.get(1) {
                    stacks[i].push(m2.as_str().into());
                }
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    // instructions.reverse();

    (stacks, instructions)
}

#[test]
fn test_parse() {
    let str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    assert_eq!(
        (
            vec![
                vec!["Z".to_owned(), "N".to_owned()],
                vec!["M".to_owned(), "C".to_owned(), "D".to_owned()],
                vec!["P".to_owned()]
            ],
            vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2),]
        ),
        parse(str)
    );
}
