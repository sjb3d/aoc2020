use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(entries: &[usize]) {
    let mut matches = HashSet::new();
    for n in entries.iter() {
        matches.insert(n);
        let m = 2020 - n;
        if matches.contains(&m) {
            println!("day01: product is {} from {} and {}", n * m, n, m);
        }
    }
}

fn part2(entries: &[usize]) {
    for x0 in entries.iter() {
        for x1 in entries.iter().filter(|&x1| x1 > x0) {
            for x2 in entries.iter().filter(|&x2| x2 > x1) {
                if x0 + x1 + x2 == 2020 {
                    println!(
                        "day01: product is {} from {}, {} and {}",
                        x0 * x1 * x2,
                        x0,
                        x1,
                        x2
                    );
                }
            }
        }
    }
}

pub fn run() {
    let file = File::open("input/day01.txt").unwrap();
    let buf = BufReader::new(file);
    let entries: Vec<_> = buf
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();

    part1(&entries);
    part2(&entries);
}
