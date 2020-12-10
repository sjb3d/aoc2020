pub fn run() {
    let text = std::fs::read_to_string("input/day10.txt").unwrap();
    let numbers: Vec<_> = text.lines().map(|s| s.parse::<usize>().unwrap()).collect();

    let mut joltages = Vec::new();
    joltages.push(0);
    joltages.extend_from_slice(&numbers);
    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);

    let mut diff1_count = 0;
    let mut diff3_count = 0;
    for s in joltages.windows(2) {
        match s[1] - s[0] {
            0 | 2 => {}
            1 => diff1_count += 1,
            3 => diff3_count += 1,
            _ => panic!("joltage chain broken"),
        }
    }

    println!("day10: product of counts is {}", diff1_count * diff3_count);
}
