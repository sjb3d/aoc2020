fn insert_index(index_by_val: &mut Vec<usize>, val: usize, index: usize) -> Option<usize> {
    if index_by_val.len() <= val {
        index_by_val.resize(val, 0);
        index_by_val.push(index + 1);
        None
    } else {
        let entry = unsafe { index_by_val.get_unchecked_mut(val) };
        let replaced_index = *entry;
        *entry = index + 1;
        if replaced_index != 0 {
            Some(replaced_index - 1)
        } else {
            None
        }
    }
}

pub fn run() {
    let mut seq = vec![1, 17, 0, 10, 18, 11, 6];
    let mut index_by_val = Vec::new();
    let mut replaced_index = None;
    for (index, val) in seq.iter().cloned().enumerate() {
        replaced_index = insert_index(&mut index_by_val, val, index);
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
        replaced_index = insert_index(&mut index_by_val, val, index);
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
        replaced_index = insert_index(&mut index_by_val, val, index);
    }
    println!("day15: 30000000th is {}", seq.last().unwrap());
}
