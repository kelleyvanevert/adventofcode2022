fn main() {
    dotenv::dotenv().ok();
    let key = std::env::var("KEY").expect("Missing env var KEY");
    println!("Encrypting input files with key: {}", key);
    let key = key.as_bytes();

    let days = [
        "day01", "day02", "day03", "day04", "day05", "day06", "day07", "day08", "day09", "day10",
        "day11", "day12", "day13", "day14", "day15", "day16", "day17", "day18", "day19", "day20",
        "day21", "day22", "day23", "day24", "day25",
    ];

    for day in days {
        if std::path::Path::new(&format!("../{day}/input.txt")).exists() {
            let input = std::fs::read_to_string(format!("../{day}/input.txt")).unwrap();
            let encrypted = encrypt(key, &input);
            assert_eq!(decrypt(key, &encrypted), input);
            std::fs::write(format!("../{day}/input.txt.encrypted"), encrypted)
                .expect("could not write encrypted file");

            println!("{day} - DONE");
        } else {
            println!("{day} (skipped)");
        }
    }
}

fn encrypt(key: &[u8], original: &str) -> Vec<u8> {
    original
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, &b)| b.wrapping_add(key[i % key.len()]))
        .collect()
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
