use crate::sand::parse;

mod sand;

fn main() {
    let filecontents = get_input();

    let cave_block = parse(&filecontents);

    println!(
        "Number of grains of sand: {}",
        cave_block.clone().drop_grains_until_done()
    );
    println!(
        "Until blocked: {}",
        cave_block.clone().drop_grains_until_hole_blocked()
    );
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
