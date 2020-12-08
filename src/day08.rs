use std::fs::read_to_string;

enum Instr {
    Nop,
    Acc(i32),
    Jmp(i32),
}

pub fn run() {
    let text = read_to_string("input/day08.txt").unwrap();
    let instr: Vec<_> = text
        .lines()
        .map(|s| {
            let val = s[4..].parse::<i32>().unwrap();
            match &s[..3] {
                "nop" => Instr::Nop,
                "acc" => Instr::Acc(val),
                "jmp" => Instr::Jmp(val),
                _ => panic!("unknown instruction"),
            }
        })
        .collect();
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;
    let mut hits = vec![false; instr.len()];
    loop {
        if hits[pc as usize] {
            println!("day08: accumulator at loop is {}", acc);
            break;
        }
        hits[pc as usize] = true;
        pc += match instr[pc as usize] {
            Instr::Nop => 1,
            Instr::Acc(val) => {
                acc += val;
                1
            }
            Instr::Jmp(val) => val,
        };
    }
}
