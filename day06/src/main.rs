use std::fs;

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();
    println!(
        "first packet marker found: {}",
        find_marker(&filecontents, 4)
    );
    println!(
        "first message marker found: {}",
        find_marker(&filecontents, 14)
    );
}

fn find_marker(s: &str, len: usize) -> usize {
    s.chars()
        .collect::<Vec<char>>()
        .windows(len)
        .enumerate()
        .find(|&(_, cs)| all_different(cs))
        .unwrap()
        .0
        + len
}

fn all_different(cs: &[char]) -> bool {
    for i in 0..cs.len() {
        for j in (i + 1)..cs.len() {
            if cs[i] == cs[j] {
                return false;
            }
        }
    }

    true
}

#[test]
fn test_find_marker() {
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);

    assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
}
