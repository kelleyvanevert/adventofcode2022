use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashSet, fs, ops::RangeInclusive, time::Instant};

fn main() {
    let filecontents = fs::read_to_string("./input.txt").unwrap();
    let grid = parse(&filecontents);

    let t0 = Instant::now();
    println!("Defo not: {}", grid.check_row_v1(2_000_000));
    println!(" - took {:?}", t0.elapsed());
    // 0ms

    let t0 = Instant::now();
    println!(
        "Found beacon: {:?}",
        grid.find_beacon(
            Span {
                start: 0,
                end: 4000000,
            },
            Span {
                start: 0,
                end: 4000000,
            },
        )
    );
    println!(" - took {:?}", t0.elapsed());
    // ±11s
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn manhattan(&self, other: Pos) -> i32 {
        (self.y - other.y).abs() + (self.x - other.x).abs()
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Diamond {
    // aka manhattan circle
    center: Pos,
    radius: i32,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Span {
    start: i32,
    end: i32,
}

impl Span {
    fn overlaps(&self, other: Span) -> bool {
        self.end >= other.start && self.start <= other.end
    }

    fn combine_with(&self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    fn contains(&self, value: i32) -> bool {
        self.start <= value && value <= self.end
    }

    fn contains_span(&self, other: Span) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

impl IntoIterator for Span {
    type Item = i32;
    type IntoIter = RangeInclusive<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Spans {
    // kept sorted
    spans: Vec<Span>,
}

impl Spans {
    fn new() -> Self {
        Self { spans: vec![] }
    }

    // bounds inclusive
    fn add(&self, mut add: Span) -> Spans {
        let mut spans = vec![];
        let mut i = 0;

        while self.spans.len() > i && add.end < self.spans[i].start {
            spans.push(self.spans[i]);
            i += 1;
        }

        while self.spans.len() > i && add.overlaps(self.spans[i]) {
            add = add.combine_with(self.spans[i]);
            i += 1;
        }
        spans.push(add);

        while self.spans.len() > i {
            spans.push(self.spans[i]);
            i += 1;
        }

        Spans { spans }
    }

    fn len(&self) -> usize {
        self.spans
            .iter()
            .map(|&span| (span.end - span.start + 1) as usize)
            .sum()
    }

    fn contains(&self, value: i32) -> bool {
        self.spans.iter().any(|span| span.contains(value))
    }

    fn contains_span(&self, other: Span) -> bool {
        self.spans.iter().any(|span| span.contains_span(other))
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    measurements: Vec<(Pos, Pos, i32)>,
    diamonds: Vec<Diamond>,
}

impl Grid {
    fn new() -> Self {
        Self {
            measurements: vec![],
            diamonds: vec![],
        }
    }

    fn measure(&mut self, sensor: Pos, beacon: Pos) {
        self.measurements
            .push((sensor, beacon, sensor.manhattan(beacon)));

        self.diamonds.push(Diamond {
            center: sensor,
            radius: sensor.manhattan(beacon),
        });
    }

    fn check_row_v1(&self, y: i32) -> usize {
        let mut no_beacons = Spans::new();

        for (sensor, _, dist) in &self.measurements {
            let dy = (y - sensor.y).abs();
            let xpad = dist - dy;
            if xpad >= 0 {
                no_beacons = no_beacons.add(Span {
                    start: sensor.x - xpad,
                    end: sensor.x + xpad,
                });
            }
        }

        let xs: HashSet<i32> = HashSet::from_iter(
            self.measurements
                .iter()
                .map(|p| p.1)
                .filter(|&beacon| beacon.y == y && no_beacons.contains(beacon.x))
                .map(|b| b.x),
        );

        no_beacons.len() - xs.len()
    }

    fn check_row_v2(&self, y: i32, span: Span) -> Option<Pos> {
        let mut no_beacons = Spans::new();

        for (sensor, _, dist) in &self.measurements {
            let dy = (y - sensor.y).abs();
            let xpad = dist - dy;
            if xpad >= 0 {
                no_beacons = no_beacons.add(Span {
                    start: sensor.x - xpad,
                    end: sensor.x + xpad,
                });
            }
        }

        if no_beacons.contains_span(span) {
            None
        } else {
            for x in span {
                if !no_beacons.contains(x) {
                    return Some(Pos::new(x, y));
                }
            }
            None
        }
    }

    fn find_beacon(&self, xspan: Span, yspan: Span) -> Option<(Pos, u64)> {
        yspan
            .into_iter()
            .into_par_iter()
            .find_map_any(|y| self.check_row_v2(y, xspan))
            .map(|beacon| (beacon, beacon.x as u64 * 4_000_000 + beacon.y as u64))
    }
}

fn parse(s: &str) -> Grid {
    let re = Regex::new(
        r"^Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
    )
    .unwrap();

    let mut grid = Grid::new();

    for line in s.lines() {
        let m = re.captures(line).unwrap();
        let sensor = Pos::new(m[1].parse::<i32>().unwrap(), m[2].parse::<i32>().unwrap());
        let beacon = Pos::new(m[3].parse::<i32>().unwrap(), m[4].parse::<i32>().unwrap());

        grid.measure(sensor, beacon);
    }

    grid
}

#[test]
fn test_all() {
    let s = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    let grid = parse(s);

    assert_eq!(grid.check_row_v1(10), 26);

    assert_eq!(
        grid.find_beacon(Span { start: 0, end: 20 }, Span { start: 0, end: 20 }),
        Some((Pos::new(14, 11), 56000011))
    );
}
