use std::fs::File;
use std::io::{BufRead, BufReader};

fn tree_count(slope_x: usize, slope_y: usize) -> usize {
    let file = File::open("input/day03.txt").unwrap();
    let buf = BufReader::new(file);

    let mut pos_x = 0;
    let mut step_y = 0;
    let mut count = 0;
    for line in buf.lines() {
        let s = line.unwrap();
        if step_y == 0 {
            let c = s.chars().cycle().nth(pos_x).unwrap();
            if c == '#' {
                count += 1;
            }
        }
        step_y += 1;
        if step_y == slope_y {
            pos_x += slope_x;
            step_y = 0;
        }
    }
    count
}

pub fn run() {
    println!("day03: tree count is {}", tree_count(3, 1));

    let count_product = tree_count(1, 1)
        * tree_count(3, 1)
        * tree_count(5, 1)
        * tree_count(7, 1)
        * tree_count(1, 2);
    println!("day03: product of slopes is {}", count_product);
}
