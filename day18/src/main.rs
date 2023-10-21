use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn main() {
    let filecontents = get_input();

    let t0 = Instant::now();
    let pixels = parse(&filecontents);
    println!("Pixels: {}", pixels.len());
    println!("Surface area: {:?}", surface_area(&pixels));
    println!("  took {:?}", t0.elapsed());

    let t0 = Instant::now();
    let filled_in = fill_in_holes(&pixels);
    println!("Pixels, filled in: {}", filled_in.len());
    println!("Exterior surface area: {:?}", surface_area(&filled_in));
    println!("  took {:?}", t0.elapsed());
}

type Pixel = (i32, i32, i32);

fn parse(s: &str) -> HashSet<Pixel> {
    s.lines()
        .map(|line| {
            let pts = line
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let x = pts[0];
            let y = pts[1];
            let z = pts[2];

            (x, y, z)
        })
        .collect()
}

fn surface_area(pixels: &HashSet<Pixel>) -> usize {
    let mut xs = HashMap::new();
    let mut ys = HashMap::new();
    let mut zs = HashMap::new();

    for &(x, y, z) in pixels {
        xs.entry((x, y, z)).and_modify(|c| *c += 1).or_insert(1);
        xs.entry((x + 1, y, z)).and_modify(|c| *c += 1).or_insert(1);

        ys.entry((x, y, z)).and_modify(|c| *c += 1).or_insert(1);
        ys.entry((x, y + 1, z)).and_modify(|c| *c += 1).or_insert(1);

        zs.entry((x, y, z)).and_modify(|c| *c += 1).or_insert(1);
        zs.entry((x, y, z + 1)).and_modify(|c| *c += 1).or_insert(1);
    }

    xs.values().filter(|&&c| c == 1).count()
        + ys.values().filter(|&&c| c == 1).count()
        + zs.values().filter(|&&c| c == 1).count()
}

fn neighbors((x, y, z): Pixel) -> Vec<Pixel> {
    vec![
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn fill_in_holes(pixels: &HashSet<Pixel>) -> HashSet<Pixel> {
    let bounds = compute_bounds(&pixels);

    let in_bounds = |(x, y, z): Pixel| {
        (bounds.0 .0 - 1 <= x && x <= bounds.0 .1 + 1)
            && (bounds.1 .0 - 1 <= y && y <= bounds.1 .1 + 1)
            && (bounds.2 .0 - 1 <= z && z <= bounds.2 .1 + 1)
    };

    let mut space = HashSet::new();

    let start = (bounds.0 .0 - 1, bounds.1 .0 - 1, bounds.2 .0 - 1);
    let mut todo = vec![start];

    while let Some(curr) = todo.pop() {
        space.insert(curr);
        for n in neighbors(curr) {
            if in_bounds(n) && !space.contains(&n) && !todo.contains(&n) && !pixels.contains(&n) {
                todo.push(n);
            }
        }
    }

    let mut nw = HashSet::new();
    for x in bounds.0 .0..=bounds.0 .1 {
        for y in bounds.1 .0..=bounds.1 .1 {
            for z in bounds.2 .0..=bounds.2 .1 {
                let p = (x, y, z);
                if !space.contains(&p) {
                    nw.insert(p);
                }
            }
        }
    }

    nw
}

type Bounds = ((i32, i32), (i32, i32), (i32, i32));

fn compute_bounds(pixels: &HashSet<Pixel>) -> Bounds {
    pixels
        .iter()
        .map(|&p| p)
        .fold(None, |bounds, pa| {
            Some(
                bounds.map_or(((pa.0, pa.0), (pa.1, pa.1), (pa.2, pa.2)), |b: Bounds| {
                    (
                        (b.0 .0.min(pa.0), b.0 .1.max(pa.0)),
                        (b.1 .0.min(pa.1), b.1 .1.max(pa.1)),
                        (b.2 .0.min(pa.2), b.2 .1.max(pa.2)),
                    )
                }),
            )
        })
        .unwrap()
}

#[test]
fn test_parse() {
    let s = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    let pixels = parse(s);
    assert_eq!(surface_area(&pixels), 64);

    let filled_in = fill_in_holes(&pixels);
    assert_eq!(surface_area(&filled_in), 58);
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
