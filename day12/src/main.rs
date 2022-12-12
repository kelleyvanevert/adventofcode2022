use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
struct State {
    pos: (usize, usize),
    dest: (usize, usize),
    map: Vec<Vec<usize>>,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pos = (0, 0);
        let mut dest = (0, 0);
        let map = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            pos = (x, y);
                            0
                        }
                        'E' => {
                            dest = (x, y);
                            25
                        }
                        _ => c as usize - 'a' as usize,
                    })
                    .collect()
            })
            .collect();

        Ok(Self { pos, dest, map })
    }
}

#[test]
fn test_parse() {
    let s = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    let state = State {
        pos: (0, 0),
        dest: (5, 2),
        map: vec![
            vec![0, 0, 1, 16, 15, 14, 13, 12],
            vec![0, 1, 2, 17, 24, 23, 23, 11],
            vec![0, 2, 2, 18, 25, 25, 23, 10],
            vec![0, 2, 2, 19, 20, 21, 22, 9],
            vec![0, 1, 3, 4, 5, 6, 7, 8],
        ],
    };

    assert_eq!(Ok(state), s.parse());
}
