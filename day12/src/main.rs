use std::{collections::HashSet, fs};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();
    let (start, end, map) = parse(&filecontents);
    println!(
        "Min number of steps: {}",
        solve(HashSet::from([start]), end, &map)
    );
    println!(
        "Best route: {}",
        solve(find_starting_positions(&map), end, &map)
    );
}

type Pos = (usize, usize);

#[derive(PartialEq, Clone, Debug)]
struct HeightMap {
    height: usize,
    width: usize,
    map: Vec<Vec<usize>>,
}

impl HeightMap {
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

fn parse(s: &str) -> (Pos, Pos, HeightMap) {
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

    (
        pos,
        dest,
        HeightMap {
            height: map.len(),
            width: map[0].len(),
            map,
        },
    )
}

fn solve(start: HashSet<Pos>, end: Pos, map: &HeightMap) -> usize {
    let mut reachability: Vec<Vec<Option<usize>>> = vec![vec![None; map.width]; map.height];

    for p in &start {
        reachability[p.1][p.0] = Some(0);
    }

    let mut todo = start.into_iter().collect::<Vec<Pos>>();

    while todo.len() > 0 {
        let p = todo.pop().unwrap();
        let s1 = reachability[p.1][p.0].unwrap();
        for n in map.neighbors(p) {
            if map.height(n) <= map.height(p) + 1 {
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

    reachability[end.1][end.0].unwrap()
}

fn find_starting_positions(map: &HeightMap) -> HashSet<Pos> {
    let mut ps = HashSet::new();

    for y in 0..map.height {
        for x in 0..map.width {
            if map.height((x, y)) == 0 {
                ps.insert((x, y));
            }
        }
    }

    ps
}

#[test]
fn test_all() {
    let s = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    let map = HeightMap {
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

    assert_eq!(((0, 0), (5, 2), map.clone()), parse(s));
    assert_eq!(31, solve(HashSet::from([(0, 0)]), (5, 2), &map));
    assert_eq!(29, solve(find_starting_positions(&map), (5, 2), &map));
}
