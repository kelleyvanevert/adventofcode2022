use std::{fmt::Debug, fs};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    let sum = solve(&filecontents);
    println!("Sum: {}", sum);
    // Gives -9999, which apparently is wrong. Hmm :|
}

fn modulo(mut a: i32, m: i32) -> i32 {
    while a < 0 {
        a += m;
    }
    a % m
}

/// This is very tricky business...
fn move_index_by(index: i32, num: i32, n: i32) -> i32 {
    if num == 0 {
        return index;
    }

    if num < 0 {
        modulo(index + num - 1, n - 1) + 1
    } else {
        modulo(index + num, n - 1)
    }
}

#[test]
fn test_move_index_by() {
    assert_eq!(move_index_by(0, 1, 7), 1);
    assert_eq!(move_index_by(0, 2, 7), 2);
    assert_eq!(move_index_by(1, -3, 7), 4);
    assert_eq!(move_index_by(2, 3, 7), 5);
    assert_eq!(move_index_by(2, -2, 7), 6);
    assert_eq!(move_index_by(3, 0, 7), 3);
    assert_eq!(move_index_by(5, 4, 7), 3);
}

fn move_range<T: Debug>(v: &mut Vec<T>, at: usize, num: usize, move_by: i32) {
    assert!(num <= v.len());

    if move_by > 0 {
        assert!(at + num + (move_by as usize) <= v.len());
    } else {
        assert!((at as i32) + move_by >= 0);
    }

    let removed = v.splice(at..(at + num), []).collect::<Vec<T>>();

    let i = (at as i32 + move_by) as usize;

    v.splice(i..i, removed);
}

#[test]
fn test_move_range() {
    let mut v = vec!["A", "B", "C", "D", "E"];

    move_range(&mut v, 0, 5, 0);
    assert_eq!(v, vec!["A", "B", "C", "D", "E"]);

    move_range(&mut v, 1, 2, 2);
    assert_eq!(v, vec!["A", "D", "E", "B", "C"]);

    move_range(&mut v, 1, 1, 2);
    assert_eq!(v, vec!["A", "E", "B", "D", "C"]);

    move_range(&mut v, 3, 2, -2);
    assert_eq!(v, vec!["A", "D", "C", "E", "B"]);

    move_range(&mut v, 4, 1, -4);
    assert_eq!(v, vec!["B", "A", "D", "C", "E"]);
}

fn solve(s: &str) -> i32 {
    let mut nums = s
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let n = nums.len();

    let mut indices = (0..n).collect::<Vec<usize>>();

    for i in 0..n {
        let current_index = indices.iter().position(|&index| i == index).unwrap();
        // let num = nums[current_index];
        let num = nums.remove(current_index);
        let new_index = move_index_by(current_index as i32, num, n as i32) as usize;
        nums.insert(new_index, num);

        if new_index == current_index {
            // noop
        } else if new_index > current_index {
            move_range(
                &mut indices,
                current_index + 1,
                new_index - current_index,
                -1,
            );
        } else {
            move_range(&mut indices, new_index + 1, current_index - new_index, 1);
        }

        // println!("{:?}", nums);
    }

    let index_of_zero = nums.iter().position(|&n| n == 0).unwrap();

    // println!("index of zero: {}", index_of_zero);
    // println!("found: {}", nums[(index_of_zero + 1000) % n]);
    // println!("found: {}", nums[(index_of_zero + 2000) % n]);
    // println!("found: {}", nums[(index_of_zero + 3000) % n]);

    nums[(index_of_zero + 1000) % n]
        + nums[(index_of_zero + 2000) % n]
        + nums[(index_of_zero + 3000) % n]
}

#[test]
fn test_all() {
    let s = "1
2
-3
3
-2
0
4
";

    assert_eq!(solve(s), 3);
}
