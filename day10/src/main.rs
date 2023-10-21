fn main() {
    let filecontents = get_input();

    let (accum, image) = solve(&filecontents);
    println!("Solution: {}", accum);
    println!("{}", image);
}

fn solve(s: &str) -> (i32, String) {
    let mut x = 1;
    let mut i = 1;
    let mut accum = 0;
    let mut image: Vec<Vec<&str>> = vec![vec!["."; 40]; 6];

    let mut increase_cycle = |x: i32| {
        if (i + 20) % 40 == 0 {
            let signal_strength = i * x;
            accum += signal_strength;
        }

        let xy = ((i - 1) % 40, ((i - 1) / 40) % 6);
        if x - 1 <= xy.0 && xy.0 <= x + 1 {
            image[xy.1 as usize][xy.0 as usize] = "#";
        }

        i += 1;
    };

    for line in s.lines() {
        match &line[0..4] {
            "addx" => {
                increase_cycle(x);
                increase_cycle(x);
                let num = line[5..].parse::<i32>().unwrap();
                x += num;
            }
            "noop" => {
                increase_cycle(x);
            }
            _ => unreachable!(),
        }
    }

    (
        accum,
        image
            .iter()
            .map(|line| line.join(""))
            .collect::<Vec<String>>()
            .join("\n"),
    )
}

#[test]
fn test_solve() {
    let s = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    let image = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        .to_string();

    assert_eq!((13140, image), solve(s));
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
