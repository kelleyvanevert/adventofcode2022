use itertools::Itertools;
use std::fs;

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    let text = find_text(filecontents.trim());
    println!("{}", text);
    assert_eq!(text, "UNDER-THE-BLENDER".to_string());

    let num = find_num(filecontents.trim());
    println!("{}", num);
    assert_eq!(num, 14);
}

fn find_text(s: &str) -> String {
    s.chars()
        .tuple_windows()
        .filter_map(|(a, b)| {
            if a == b
                && (a == '-' || a.is_ascii_uppercase())
                && (b == '-' || b.is_ascii_uppercase())
            {
                Some(a)
            } else {
                None
            }
        })
        .collect()
}

fn find_num(s: &str) -> usize {
    s.chars()
        .tuple_windows()
        .filter_map(|(a, b)| {
            if a == b && a.is_ascii_digit() && b.is_ascii_digit() {
                Some(a)
            } else {
                None
            }
        })
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

#[test]
fn test() {
    let s = "7*(()HsdiKK%322sbu--1^*NsK##-DHHdMm&&--vhso55eeneskTT@#1k";

    assert_eq!(find_text(s), "K-H-T".to_string());
    assert_eq!(find_num(s), 25);
}
