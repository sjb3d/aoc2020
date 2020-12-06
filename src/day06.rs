use std::fs::read_to_string;

pub fn run() {
    let text = read_to_string("input/day06.txt").unwrap();
    let mut group_masks = vec![0];
    for mask in text.lines().map(|s| {
        s.chars().fold(0, |acc, c| {
            acc | match c {
                'a'..='z' => 1 << ((c as usize) - ('a' as usize)),
                _ => panic!("unknown symbol"),
            }
        })
    }) {
        if mask != 0 {
            *group_masks.last_mut().unwrap() |= mask;
        } else {
            group_masks.push(0)
        }
    }
    println!(
        "day06: total yes questions is {}",
        group_masks
            .iter()
            .fold(0, |acc, m: &usize| acc + m.count_ones())
    );
}
