use std::fs;

fn main() {
    let s = fs::read_to_string("./input.txt").unwrap();
    println!("Number of trees visible: {}", solve(&s));
}

fn solve(s: &str) -> usize {
    let tree_heights: Vec<Vec<usize>> = s
        .lines()
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| (c as usize) - ('0' as usize))
                .collect()
        })
        .collect();

    let w = tree_heights[0].len();
    let h = tree_heights.len();

    let mut num_visible = 0;

    for y in 0..h {
        for x in 0..w {
            let height = tree_heights[y][x];

            // could be optimized later by changing the order of the checks depending on the position

            let visible = (0..y).all(|y0| tree_heights[y0][x] < height)
                || ((y + 1)..h).all(|y0| tree_heights[y0][x] < height)
                || (0..x).all(|x0| tree_heights[y][x0] < height)
                || ((x + 1)..w).all(|x0| tree_heights[y][x0] < height);

            if visible {
                num_visible += 1;
            }
        }
    }

    num_visible
}

#[test]
fn test_solve() {
    let s = "30373
25512
65332
33549
35390";

    assert_eq!(solve(s), 21);
}
