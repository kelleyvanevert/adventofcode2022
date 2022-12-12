use std::{fs, str::FromStr};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();
    let state = filecontents.parse().unwrap();
    println!("Min number of steps: {}", solve(&state));
}

type Pos = (usize, usize);

#[derive(PartialEq, Debug)]
struct State {
    pos: Pos,
    dest: Pos,
    height: usize,
    width: usize,
    map: Vec<Vec<usize>>,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pos = (0, 0);
        let mut dest = (0, 0);
        let map: Vec<Vec<usize>> = s
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

        Ok(Self {
            pos,
            dest,
            height: map.len(),
            width: map[0].len(),
            map,
        })
    }
}

impl State {
    fn height(&self, pos: Pos) -> usize {
        self.map[pos.1][pos.0]
    }

    fn neighbors(&self, pos: Pos) -> Vec<Pos> {
        vec![
            (pos.0 as i32 - 1, pos.1 as i32),
            (pos.0 as i32 + 1, pos.1 as i32),
            (pos.0 as i32, pos.1 as i32 - 1),
            (pos.0 as i32, pos.1 as i32 + 1),
        ]
        .into_iter()
        .filter(|&(x, y)| {
            x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
        })
        .map(|p| (p.0 as usize, p.1 as usize))
        .collect()
    }
}

fn solve(s: &State) -> usize {
    let mut reachability: Vec<Vec<Option<usize>>> = s
        .map
        .iter()
        .map(|line| line.iter().map(|_| None).collect())
        .collect();

    reachability[s.pos.1][s.pos.0] = Some(0);

    let mut todo = vec![s.pos];

    while todo.len() > 0 {
        let p = todo.pop().unwrap();
        let s1 = reachability[p.1][p.0].unwrap();
        for n in s.neighbors(p) {
            if s.height(n) <= s.height(p) + 1 {
                match reachability[n.1][n.0] {
                    None => {
                        reachability[n.1][n.0] = Some(s1 + 1);
                        todo.push(n);
                    }
                    Some(s0) => {
                        if s1 + 1 < s0 {
                            reachability[n.1][n.0] = Some(s1 + 1);
                            todo.push(n);
                        }
                    }
                }
            }
        }
    }

    reachability[s.dest.1][s.dest.0].unwrap()
}

#[test]
fn test_all() {
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
        width: 8,
        height: 5,
    };

    assert_eq!(state, s.parse().unwrap());
    assert_eq!(31, solve(&state));
}
