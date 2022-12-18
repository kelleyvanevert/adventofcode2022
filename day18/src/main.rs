use std::{collections::HashMap, fs};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();
    println!("Surface area: {}", surface_area(&parse(&filecontents)));
}

type Pixel = (i32, i32, i32);

fn parse(s: &str) -> Vec<Pixel> {
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

fn surface_area(pixels: &Vec<Pixel>) -> usize {
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

    assert_eq!(surface_area(&parse(s)), 64);
}
