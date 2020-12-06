use std::fs::read_to_string;

pub fn run() {
    let text = read_to_string("input/day06.txt").unwrap();
    let full_mask = (1 << 26) - 1;
    let mut group_any_masks = vec![0];
    let mut group_all_masks = vec![full_mask];
    for mask in text.lines().map(|s| {
        s.chars().fold(0, |acc, c| {
            acc | match c {
                'a'..='z' => 1 << ((c as usize) - ('a' as usize)),
                _ => panic!("unknown symbol"),
            }
        })
    }) {
        if mask != 0 {
            *group_any_masks.last_mut().unwrap() |= mask;
            *group_all_masks.last_mut().unwrap() &= mask;
        } else {
            group_any_masks.push(0);
            group_all_masks.push(full_mask);
        }
    }
    println!(
        "day06: total any yes questions is {}",
        group_any_masks
            .iter()
            .fold(0, |acc, m: &usize| acc + m.count_ones())
    );
    println!(
        "day06: total all yes questions is {}",
        group_all_masks
            .iter()
            .fold(0, |acc, m: &usize| acc + m.count_ones())
    );
}
