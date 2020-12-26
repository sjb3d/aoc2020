const P: usize = 20201227;

fn iterate(value: usize, subject: usize) -> usize {
    (value * subject) % P
}

fn find_loop_size(target_value: usize) -> usize {
    let mut counter = 0;
    let mut value = 1;
    while value != target_value {
        value = iterate(value, 7);
        counter += 1;
    }
    counter
}

fn transform(subject: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = iterate(value, subject);
    }
    value
}

pub fn run() {
    let input1 = 10604480;
    let input2 = 4126658;

    let loop_size1 = find_loop_size(input1);
    println!("day25: encryption key is {}", transform(input2, loop_size1));
}
