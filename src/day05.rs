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

    let mut bits = vec![false; 1024];
    for id in ids.iter() {
        bits[*id] = true;
    }
    let bits = bits;
    for id in 8..1016 {
        if !bits[id] && bits[id - 1] && bits[id + 1] {
            println!("day05: seat id is {}", id);
        }
    }
}
