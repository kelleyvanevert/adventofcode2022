fn main() {
    let filecontents = get_input();
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
