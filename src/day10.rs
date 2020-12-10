fn count_reps(n: usize) -> usize {
    match n {
        0 => panic!("zero not expected"),
        1 => 1, // 1
        2 => 2, // 1,1 | 2
        3 => 4, // 1,1,1 | 2,1 | 1,2 | 3
        _ => count_reps(n - 1) + count_reps(n - 2) + count_reps(n - 3),
    }
}

fn count_permutations(s: &[usize]) -> usize {
    assert!(s.iter().all(|&d| d == 1));
    count_reps(s.len())
}

pub fn run() {
    let text = std::fs::read_to_string("input/day10.txt").unwrap();
    let numbers: Vec<_> = text.lines().map(|s| s.parse::<usize>().unwrap()).collect();

    let mut joltages = Vec::new();
    joltages.push(0);
    joltages.extend_from_slice(&numbers);
    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);

    let diffs: Vec<_> = joltages.windows(2).map(|s| s[1] - s[0]).collect();
    let mut diff1_count = 0;
    let mut diff3_count = 0;
    for d in diffs.iter() {
        match d {
            0 | 2 => {}
            1 => diff1_count += 1,
            3 => diff3_count += 1,
            _ => panic!("joltage chain broken"),
        }
    }

    println!("day10: product of counts is {}", diff1_count * diff3_count);

    let perm_count = diffs
        .split(|&d| d == 3)
        .filter(|s| !s.is_empty())
        .map(|s| count_permutations(s))
        .product::<usize>();
    println!("day10: permutation count is {}", perm_count);
}
