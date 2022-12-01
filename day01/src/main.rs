use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut max = 0;
    let mut acc = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line == "" {
                    max = cmp::max(max, acc);
                    acc = 0;
                } else if let Ok(num) = line.parse::<i32>() {
                    acc += num;
                }
            }
        }
    }

    max = cmp::max(max, acc);

    println!("max: {}", max);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
