use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();

    let pixels = parse(&filecontents);
    println!("Pixels: {}", pixels.len());
    println!("Surface area: {:?}", surface_area(&pixels));

    {
        let mut filled_in_v1 = pixels.clone();
        fill_in_holes_v1(&mut filled_in_v1);
        println!("[Attempt 1] Pixels, filled in: {}", filled_in_v1.len());
        println!(
            "[Attempt 1] Exterior surface area: {:?}",
            surface_area(&filled_in_v1)
        );
        // 2668 -> too high
    }

    {
        let filled_in = fill_in_holes(&pixels);
        println!("");
        println!("[Attempt 2a] Pixels, filled in: {}", filled_in.len());
        println!(
            "[Attempt 2a] Exterior surface area: {:?}",
            surface_area(&filled_in)
        );
        // 2408 -> too low

        let nw = pixels
            .union(&filled_in)
            .map(|&p| p)
            .collect::<HashSet<Pixel>>();

        println!("");
        println!("[Attempt 2b] Pixels, filled in, unioned: {}", nw.len());
        println!(
            "[Attempt 2b] Exterior surface area: {:?}",
            surface_area(&nw)
        );
        // 2430 -> too low
    }

    println!("");
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

// fn exterior_surface_area(pixels: &Vec<Pixel>) -> usize {
//     let mut front_view = HashSet::new();
//     let mut top_view = HashSet::new();
//     let mut side_view = HashSet::new();

//     for &(x, y, z) in pixels {
//         front_view.insert((x, y)); // front
//         top_view.insert((x, z)); // top
//         side_view.insert((y, z)); // side
//     }

//     2 * (front_view.len() + side_view.len() + top_view.len())
// }

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

    let mut h: HashMap<Pixel, (i32, i32, i32)> = HashMap::new();

    for x in bounds.0 .0..=bounds.0 .1 {
        for y in bounds.1 .0..=bounds.1 .1 {
            for z in bounds.2 .0..=bounds.2 .1 {
                // x slices
                let minx = (bounds.0 .0..=bounds.0 .1)
                    .find(|&x| pixels.contains(&(x, y, z)))
                    .unwrap_or(bounds.0 .1 + 1);

                let maxx = (bounds.0 .0..=bounds.0 .1)
                    .rev()
                    .find(|&x| pixels.contains(&(x, y, z)))
                    .unwrap_or(bounds.0 .0 - 1);

                for x in minx..=maxx {
                    h.entry((x, y, z))
                        .and_modify(|f| f.0 += 1)
                        .or_insert((0, 0, 0));
                }

                // y slices
                let miny = (bounds.1 .0..=bounds.1 .1)
                    .find(|&z| pixels.contains(&(x, y, z)))
                    .unwrap_or(bounds.1 .1 + 1);

                let maxy = (bounds.1 .0..=bounds.1 .1)
                    .rev()
                    .find(|&y| pixels.contains(&(x, y, z)))
                    .unwrap_or(bounds.1 .0 - 1);

                for y in miny..=maxy {
                    h.entry((x, y, z))
                        .and_modify(|f| f.1 += 1)
                        .or_insert((0, 0, 0));
                }

                // z slices
                let minz = (bounds.2 .0..=bounds.2 .1)
                    .find(|&z| pixels.contains(&(x, y, z)))
                    .unwrap_or(bounds.2 .1 + 1);

                let maxz = (bounds.2 .0..=bounds.2 .1)
                    .rev()
                    .find(|&z| pixels.contains(&(x, y, z)))
                    .unwrap_or(bounds.2 .0 - 1);

                for z in minz..=maxz {
                    h.entry((x, y, z))
                        .and_modify(|f| f.2 += 1)
                        .or_insert((0, 0, 0));
                }
            }
        }
    }

    h.retain(|_, (cx, cy, cz)| *cx > 0 && *cy > 0 && *cz > 0);

    h.keys().map(|&p| p).collect::<HashSet<Pixel>>()
}

fn fill_in_holes_v1(pixels: &mut HashSet<Pixel>) {
    let bounds = compute_bounds(&pixels);

    let in_bounds = |(x, y, z): Pixel| {
        (bounds.0 .0 <= x && x <= bounds.0 .1)
            && (bounds.1 .0 <= y && y <= bounds.1 .1)
            && (bounds.2 .0 <= z && z <= bounds.2 .1)
    };

    let is_contained = |(x, y, z): Pixel| {
        let right = ((x + 1)..=bounds.0 .1).find(|&x| pixels.contains(&(x, y, z)));
        let above = ((y + 1)..=bounds.1 .1).find(|&y| pixels.contains(&(x, y, z)));
        let behind = ((z + 1)..=bounds.2 .1).find(|&z| pixels.contains(&(x, y, z)));
        let left = (bounds.0 .0..x).find(|&x| pixels.contains(&(x, y, z)));
        let below = (bounds.1 .0..y).find(|&y| pixels.contains(&(x, y, z)));
        let front = (bounds.2 .0..z).find(|&z| pixels.contains(&(x, y, z)));
        right.and_then(|right| {
            above.and_then(|above| {
                behind.and_then(|behind| {
                    left.and_then(|left| {
                        below.and_then(|below| {
                            front.and_then(|front| Some((right, above, behind, left, below, front)))
                        })
                    })
                })
            })
        })
    };

    let mut to_fill_in = HashSet::new();

    for p in pixels.iter().flat_map(|&p| neighbors(p)) {
        if !in_bounds(p) || to_fill_in.contains(&p) || pixels.contains(&p) {
            continue;
        }
        if let Some(arr) = is_contained(p) {
            // println!("Found contained! {:?} -- {:?}", p, arr);
            let mut todo = vec![p];
            while let Some(p) = todo.pop() {
                to_fill_in.insert(p);
                for n in neighbors(p) {
                    if !in_bounds(n)
                        || to_fill_in.contains(&n)
                        || pixels.contains(&n)
                        || todo.contains(&n)
                    {
                        continue;
                    } else {
                        todo.push(n);
                    }
                }
            }
        }
    }

    for p in to_fill_in {
        pixels.insert(p);
    }
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
