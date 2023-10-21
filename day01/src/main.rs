use std::{time::Instant, vec};

fn main() {
    let filecontents = get_input();

    time(|| {
        let parse_i32 = |s: &str| s.parse::<i32>().unwrap();

        let max_three = filecontents
            .split("\n\n")
            .map(|group| group.lines().map(parse_i32).sum())
            .fold(vec![0, 0, 0], keep_sorted_desc);

        println!("Max three: {:?}", max_three);
        println!("Their sum: {}", max_three.iter().sum::<i32>());
    });
}

fn time<F>(mut f: F)
where
    F: FnMut(),
{
    let t0 = Instant::now();
    f();
    println!("  took {:?}", t0.elapsed());
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
