use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{cmp, vec};

fn main() {
    let mut max = vec![0, 0, 0];
    let mut acc = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line == "" {
                    max = keep_sorted_desc(max, acc);
                    acc = 0;
                } else if let Ok(num) = line.parse::<i32>() {
                    acc += num;
                }
            }
        }
    }

    max = keep_sorted_desc(max, acc);

    println!("max: {:?}, total: {}", max, max.iter().sum::<i32>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn keep_sorted_desc(mut max: Vec<i32>, num: i32) -> Vec<i32> {
    for i in 0..max.len() {
        if num > max[i] {
            max.insert(i, num);
            max.pop();
            return max;
        }
    }

    max
}

#[test]
fn test_keep_sorted_desc() {
    assert_eq!(keep_sorted_desc(vec![8, 4, 1], 5), vec![8, 5, 4]);
    assert_eq!(keep_sorted_desc(vec![8, 4, 1], 4), vec![8, 4, 4]);
    assert_eq!(keep_sorted_desc(vec![8, 4, 1], 0), vec![8, 4, 1]);
    assert_eq!(keep_sorted_desc(vec![8, 4, 1], 10), vec![10, 8, 4]);
}
