use std::collections::HashMap;

pub fn run() {
    let mut seq = vec![1, 17, 0, 10, 18, 11, 6];
    let mut index_by_val = HashMap::new();
    let mut replaced_index = None;
    for (index, val) in seq.iter().cloned().enumerate() {
        replaced_index = index_by_val.insert(val, index);
    }

    while seq.len() < 2020 {
        let val = if let Some(first_index) = replaced_index {
            let second_index = seq.len() - 1;
            second_index - first_index
        } else {
            0
        };
        let index = seq.len();
        seq.push(val);
        replaced_index = index_by_val.insert(val, index);
    }
    println!("day15: 2020th is {}", seq.last().unwrap());

    while seq.len() < 30000000 {
        let val = if let Some(first_index) = replaced_index {
            let second_index = seq.len() - 1;
            second_index - first_index
        } else {
            0
        };
        let index = seq.len();
        seq.push(val);
        replaced_index = index_by_val.insert(val, index);
    }
    println!("day15: 30000000th is {}", seq.last().unwrap());
}
