use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let file = File::open("input/day03.txt").unwrap();
    let buf = BufReader::new(file);

    let mut pos = 0;
    let mut tree_count = 0;
    for line in buf.lines() {
        let s = line.unwrap();
        let c = s.chars().cycle().nth(pos).unwrap();
        if c == '#' {
            tree_count += 1;
        }
        pos += 3;
    }
    println!("day03: tree count is {}", tree_count);
}
