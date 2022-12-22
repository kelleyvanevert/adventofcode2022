#![feature(drain_filter)]
#![feature(iter_collect_into)]

use itertools::Itertools;
use quaternion::{axis_angle, rotate_vector};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
    fs,
    time::Instant,
};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    time(|| {
        let pass = solve(&filecontents, None, false);
        println!("\nPassword: {}", pass);
        assert_eq!(pass, 149138);
    });

    time(|| {
        let pass_3d = solve(&filecontents, Some(50), false);
        println!("\nPassword (cube): {}", pass_3d);
        assert_eq!(pass_3d, 153203);
    });

    println!();
}

fn time<F>(mut f: F)
where
    F: FnMut(),
{
    let t0 = Instant::now();
    f();
    println!("  took {:?}", t0.elapsed());
}

type Pos = (i32, i32);
type Dir = usize;

type Pos3D = (i32, i32, i32);

// (original, rotated)
type Face = ([Pos3D; 4], [Pos3D; 4]);

fn add(a: Pos3D, b: Pos3D) -> Pos3D {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn sub(a: Pos3D, b: Pos3D) -> Pos3D {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn delta_step(b: Pos3D, a: Pos3D) -> Pos3D {
    (
        (b.0 - a.0).signum(),
        (b.1 - a.1).signum(),
        (b.2 - a.2).signum(),
    )
}

const DELTAS: [Pos; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn opposite(dir: Dir) -> Dir {
    (dir + 2) % 4
}

fn solve(s: &str, cube_side_length: Option<usize>, is_test_case: bool) -> i32 {
    let (map, path) = s.split_once("\n\n").unwrap();
    let mut path = path.trim();

    let width = map.lines().map(|line| line.len()).max().unwrap();

    let grid = map
        .lines()
        .map(|s| {
            let mut line = s.chars().collect::<Vec<char>>();
            line.resize(width, ' ');
            line
        })
        .collect::<Vec<Vec<char>>>();

    let height = grid.len();

    // inclusive
    let hbounds = (0..height)
        .map(|y| {
            let xmin = (0..width).find(|&x| grid[y][x] != ' ').unwrap_or(0);
            let xmax = (0..width).rev().find(|&x| grid[y][x] != ' ').unwrap_or(0);
            (xmin as i32, xmax as i32)
        })
        .collect::<Vec<(i32, i32)>>();

    // inclusive
    let vbounds = (0..width)
        .map(|x| {
            let ymin = (0..height).find(|&y| grid[y][x] != ' ').unwrap_or(0);
            let ymax = (0..height).rev().find(|&y| grid[y][x] != ' ').unwrap_or(0);
            (ymin as i32, ymax as i32)
        })
        .collect::<Vec<(i32, i32)>>();

    let teleports = if let Some(side_length) = cube_side_length {
        let cube = find_cube_folding({
            let mut faces = vec![];

            for x in (0..(width as i32)).step_by(side_length) {
                for y in (0..(height as i32)).step_by(side_length) {
                    if grid[y as usize][x as usize] != ' ' {
                        let face = [
                            (x, y, 0),
                            (x + side_length as i32, y, 0),
                            (x + side_length as i32, y + side_length as i32, 0),
                            (x, y + side_length as i32, 0),
                        ];
                        // println!("Face: {:?}", face);
                        faces.push((face, face));
                    }
                }
            }

            faces
        });

        Some(compute_teleportation(&cube))
    } else {
        None
    };

    // let mut teleport = HashMap::new();
    // if is_test_case {
    //     for i in 0..4 {
    //         teleport.insert(((12 + i, 8), 3), ((11, 7 - i), 2)); // A
    //         teleport.insert(((11, i), 0), ((15, 11 - i), 2)); // B
    //         teleport.insert(((0, 4 + i), 2), ((15 - i, 11), 3)); // C
    //         teleport.insert(((4 + i, 4), 3), ((8, i), 0)); // D
    //         teleport.insert(((8 + i, 0), 3), ((3 - i, 4), 1)); // E
    //         teleport.insert(((4 + i, 7), 1), ((8, 11 - i), 0)); // F
    //         teleport.insert(((i, 7), 1), ((11 - i, 11), 3)); // G
    //     }
    // } else {
    //     for i in 0..50 {
    //         teleport.insert(((50 + i, 0), 3), ((0, 150 + i), 0)); // A
    //         teleport.insert(((100 + i, 0), 3), ((i, 199), 3)); // B
    //         teleport.insert(((50, i), 2), ((0, 149 - i), 0)); // C
    //         teleport.insert(((50, 50 + i), 2), ((i, 100), 1)); // D
    //         teleport.insert(((100 + i, 49), 1), ((99, 50 + i), 2)); // E
    //         teleport.insert(((149, i), 0), ((99, 149 - i), 2)); // F
    //         teleport.insert(((50 + i, 149), 1), ((49, 150 + i), 2)); // G
    //     }
    // }
    // for ((source_pos, source_dir), (target_pos, target_dir)) in teleport.clone() {
    //     teleport.insert(
    //         (target_pos, opposite(target_dir)),
    //         (source_pos, opposite(source_dir)),
    //     );
    // }

    let mut pos = (
        (0..width).position(|x| grid[1][x] == '.').unwrap() as i32,
        0,
    );
    let mut dir: usize = 0;

    let re_token = Regex::new(r"^(?:[0-9]+|R|L)").unwrap();
    while let Some(m) = re_token.captures(path) {
        path = &path[m[0].len()..];

        match &m[0] {
            "R" => {
                dir = (dir + 1) % 4;
            }
            "L" => {
                dir = (dir + 3) % 4;
            }
            steps => {
                let nsteps = steps.parse::<usize>().unwrap();

                for _ in 0..nsteps {
                    let d = DELTAS[dir];
                    let mut new_pos = (pos.0 + d.0, pos.1 + d.1);
                    let mut new_dir = dir;

                    if let Some(teleports) = &teleports {
                        if grid[new_pos.1 as usize][new_pos.0 as usize] == ' ' {
                            if let Some(target) = teleports.get(&new_pos) {
                                println!(
                                    "Teleport! from {:?} going {}, to {:?}",
                                    new_pos, dir, target
                                );
                            }
                        }
                        //
                        //
                        // if let Some((teleport_pos, teleport_dir)) = teleport.get(&(pos, dir)) {
                        //     new_pos = *teleport_pos;
                        //     new_dir = *teleport_dir;
                        // }
                    } else {
                        if dir == 0 && new_pos.0 > hbounds[new_pos.1 as usize].1 {
                            new_pos = (hbounds[new_pos.1 as usize].0, new_pos.1);
                        } else if dir == 1 && new_pos.1 > vbounds[new_pos.0 as usize].1 {
                            new_pos = (new_pos.0, vbounds[new_pos.0 as usize].0);
                        } else if dir == 2 && new_pos.0 < hbounds[new_pos.1 as usize].0 {
                            new_pos = (hbounds[new_pos.1 as usize].1, new_pos.1);
                        } else if dir == 3 && new_pos.1 < vbounds[new_pos.0 as usize].0 {
                            new_pos = (new_pos.0, vbounds[new_pos.0 as usize].1);
                        }
                    }

                    if grid[new_pos.1 as usize][new_pos.0 as usize] == '#' {
                        break;
                    } else if grid[new_pos.1 as usize][new_pos.0 as usize] == ' ' {
                        panic!("Ended up in invalid position {:?}", new_pos);
                    }

                    pos = new_pos;
                    dir = new_dir;
                }
            }
        }
    }

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + (dir as i32)
}

#[test]
fn test_quaternion_rotations() {
    assert_eq!(rotate_point((1, 0, 0), (1, 0, 0), true), (1, 0, 0));
    assert_eq!(rotate_point((0, 1, 0), (1, 0, 0), true), (0, 0, 1));
    assert_eq!(rotate_point((1, 1, 0), (1, 0, 0), true), (1, 0, 1));
}

fn rotate_point(p: Pos3D, axis: Pos3D, left: bool) -> Pos3D {
    let q = axis_angle(
        [axis.0 as f32, axis.1 as f32, axis.2 as f32],
        if left { PI / 2. } else { -PI / 2. },
    );
    let res = rotate_vector(q, [p.0 as f32, p.1 as f32, p.2 as f32]);
    (
        res[0].round() as i32,
        res[1].round() as i32,
        res[2].round() as i32,
    )
}

fn rotate_point_around(p: Pos3D, (zero, axis): (Pos3D, Pos3D), left: bool) -> Pos3D {
    add(rotate_point(sub(p, zero), axis, left), zero)
}

fn num_points(v: &Vec<Face>) -> usize {
    v.iter()
        .map(|f| f.1)
        .flatten()
        .collect::<HashSet<Pos3D>>()
        .len()
}

fn adjacent(
    (a_orig, a): Face,
    (b_orig, b): Face,
) -> Option<((Pos3D, Pos3D), Pos3D, (Pos3D, Pos3D), Pos3D, Pos3D)> {
    let edge = HashSet::from(a)
        .intersection(&HashSet::from(b))
        .map(|&p| p)
        .collect::<Vec<Pos3D>>();

    if edge.len() == 2 {
        let start = edge[0];
        let end = edge[1];
        let a_start_i = a.iter().position(|&p| p == start).unwrap();
        let a_end_i = a.iter().position(|&p| p == end).unwrap();
        let b_start_i = b.iter().position(|&p| p == start).unwrap();
        let b_end_i = b.iter().position(|&p| p == end).unwrap();

        return Some((
            (a_orig[a_start_i], b_orig[b_start_i]),
            start,
            (a_orig[a_end_i], b_orig[b_end_i]),
            end,
            delta_step(end, start),
        ));
    }

    None
}

fn same_plane((_, a): Face, (_, b): Face) -> bool {
    let xs = HashSet::from([
        a[0].0, a[1].0, a[2].0, a[3].0, b[0].0, b[1].0, b[2].0, b[3].0,
    ]);
    let ys = HashSet::from([
        a[0].1, a[1].1, a[2].1, a[3].1, b[0].1, b[1].1, b[2].1, b[3].1,
    ]);
    let zs = HashSet::from([
        a[0].2, a[1].2, a[2].2, a[3].2, b[0].2, b[1].2, b[2].2, b[3].2,
    ]);

    xs.len() == 1 || ys.len() == 1 || zs.len() == 1
}

fn find_cube_folding(faces: Vec<Face>) -> Vec<Face> {
    let mut try_foldings = vec![faces.clone()];

    while let Some(faces) = try_foldings.pop() {
        let foldable = faces
            .iter()
            .tuple_combinations()
            .filter_map(|(&a, &b)| {
                if let Some((_, _, _, edge_zero, edge_dir)) = adjacent(a, b) {
                    if !same_plane(a, b) {
                        return None;
                    }

                    return Some((a, b, (edge_zero, edge_dir)));
                }

                None
            })
            .collect::<Vec<(Face, Face, (Pos3D, Pos3D))>>();

        if foldable.len() == 0 {
            if num_points(&faces) == 8 {
                return faces;
            }
        }

        for (a, b, edge) in foldable {
            let mut faces = faces.clone();

            let mut le = HashSet::new();
            let mut todo = vec![a];
            while let Some(next) = todo.pop() {
                le.insert(next);
                faces
                    .drain_filter(|f| adjacent(*f, next).is_some() && *f != b)
                    .collect_into(&mut todo);
            }

            let mut ri = HashSet::new();
            let mut todo = vec![b];
            while let Some(next) = todo.pop() {
                ri.insert(next);
                faces
                    .drain_filter(|f| adjacent(*f, next).is_some() && *f != a)
                    .collect_into(&mut todo);
            }

            // rotate one side the one way
            let ri_clockwise = ri
                .clone()
                .into_iter()
                .map(|(orig, f)| (orig, f.map(|p| rotate_point_around(p, edge, true))))
                .collect::<HashSet<Face>>();

            let mut faces_one_way: Vec<Face> = vec![];
            le.iter().collect_into(&mut faces_one_way);
            ri_clockwise.iter().collect_into(&mut faces_one_way);

            try_foldings.push(faces_one_way);

            // rotate one side the other way
            let ri_counter_clockwise = ri
                .into_iter()
                .map(|(orig, f)| (orig, f.map(|p| rotate_point_around(p, edge, false))))
                .collect::<HashSet<Face>>();

            let mut faces_other_way: Vec<Face> = vec![];
            le.iter().collect_into(&mut faces_other_way);
            ri_counter_clockwise
                .iter()
                .collect_into(&mut faces_other_way);

            try_foldings.push(faces_other_way);
        }
    }

    unreachable!();
}

fn compute_teleportation(faces: &Vec<Face>) -> HashMap<Pos, Pos> {
    let mut teleports = HashMap::new();

    for ((a_orig_start, b_orig_start), _, (a_orig_end, b_orig_end), _, _) in faces
        .iter()
        .tuple_combinations()
        .filter_map(|(&a, &b)| adjacent(a, b))
    {
        // println!(
        //     "Map {:?} --- {:?} TO {:?} --- {:?}",
        //     a_orig_start, a_orig_end, b_orig_start, b_orig_end
        // );

        for (&a, b) in steps(a_orig_start, a_orig_end)
            .iter()
            .zip(steps(b_orig_start, b_orig_end))
        {
            // println!("  teleport {:?} to {:?}", a, b);
            teleports.insert((a.0, a.1), (b.0, b.1));
            teleports.insert((b.0, b.1), (a.0, a.1));
        }
    }

    teleports
}

fn steps(a: Pos3D, b: Pos3D) -> Vec<Pos3D> {
    let d = delta_step(b, a);

    let mut steps = vec![];

    let mut curr = a;
    while curr != b {
        steps.push(curr);
        curr = add(curr, d);
    }

    steps
}

// // inclusive on the low side, exclusive on the high side
// // using the fact that only one axis will actually step
// fn steps(mut a: Pos3D, mut b: Pos3D, side_length: i32) -> Vec<Pos3D> {
//     let d = delta_step(b, a);
//     assert!(!(d.0 != 0 && d.1 != 0 && d.2 != 0));

//     let going_up = d.0 > 0 || d.1 > 0 || d.2 > 0;

//     if d.0 == 0 && a.0 % side_length == 0 {
//         a.0 -= 1;
//         b.0 -= 1;
//     }
//     if d.1 == 0 && a.1 % side_length == 0 {
//         a.1 -= 1;
//         b.1 -= 1;
//     }
//     if d.2 == 0 && a.2 % side_length == 0 {
//         a.2 -= 1;
//         b.2 -= 1;
//     }

//     let mut steps = vec![];
//     if going_up {
//         // inclusive start
//         steps.push(a);
//     }

//     let mut curr = a;
//     loop {
//         curr = add(curr, d);

//         if curr == b {
//             if !going_up {
//                 // inclusive end
//                 steps.push(curr);
//             }
//             break;
//         } else {
//             steps.push(curr);
//         }
//     }

//     steps
// }

// #[test]
// fn test_steps() {
//     assert_eq!(
//         steps((0, 4, 0), (4, 4, 0), 4),
//         vec![(0, 3, 0), (1, 3, 0), (2, 3, 0), (3, 3, 0)]
//     );
//     // assert_eq!(
//     //     steps((12, 4, 0), (8, 4, 0), 4),
//     //     vec![(11, 4, 0), (10, 4, 0), (9, 4, 0), (8, 4, 0)]
//     // );
// }

#[test]
fn test_all() {
    let s = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    assert_eq!(solve(s, None, true), 6032);

    assert_eq!(solve(s, Some(4), true), 5031);

    // assert!(false);
}
