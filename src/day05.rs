use std::fs::read_to_string;

pub fn run() {
    let text = read_to_string("input/day05.txt").unwrap();
    let ids: Vec<_> = text
        .lines()
        .map(|s| {
            let mut id = 0;
            for c in s.chars() {
                id = (id << 1)
                    | match c {
                        'B' | 'R' => 1,
                        'F' | 'L' => 0,
                        _ => panic!("unknown symbol"),
                    };
            }
            id
        })
        .collect();
    println!("day05: highest id is {}", ids.iter().max().unwrap());
}
