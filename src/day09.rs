use std::fs::read_to_string;

pub fn run() {
    let text = read_to_string("input/day09.txt").unwrap();
    let numbers: Vec<_> = text.lines().map(|s| s.parse::<usize>().unwrap()).collect();
    for window in numbers.windows(26) {
        let (&sum, prefix) = window.split_last().unwrap();
        let found = prefix
            .iter()
            .any(|a| prefix.iter().any(|b| a != b && a + b == sum));
        if !found {
            println!("day09: first number not found is {}", sum);
            for s in 0..numbers.len() {
                let mut total = 0;
                for t in s..numbers.len() {
                    total += numbers[t];
                    if total > sum {
                        break;
                    }
                    if s != t && total == sum {
                        let r = &numbers[s..(t + 1)];
                        let sum = r.iter().min().unwrap() + r.iter().max().unwrap();
                        println!("day09: range for endpoint sum is {}", sum);
                    }
                }
            }
            break;
        }
    }
}
