use std::fs;

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();
    println!(
        "starter marker found: {}",
        find_packet_marker(&filecontents)
    );
}

fn find_packet_marker(s: &str) -> usize {
    s.chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .find(|&(_, cs)| all_different(cs))
        .unwrap()
        .0
        + 4
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
fn test_find_packet_marker() {
    assert_eq!(find_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(find_packet_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(find_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(find_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}
