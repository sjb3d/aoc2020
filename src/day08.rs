use std::fs::read_to_string;

enum Instr {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instr {
    fn swap_nop_jmp(&mut self) -> bool {
        match self {
            Instr::Nop(val) => {
                *self = Instr::Jmp(*val);
                true
            }
            Instr::Acc(..) => false,
            Instr::Jmp(val) => {
                *self = Instr::Nop(*val);
                true
            }
        }
    }
}

fn halt_check(instr: &[Instr]) -> (bool, i32) {
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;
    let mut hits = vec![false; instr.len()];
    loop {
        if pc as usize == instr.len() {
            break (true, acc);
        }
        if hits[pc as usize] {
            break (false, acc);
        }
        hits[pc as usize] = true;
        pc += match instr[pc as usize] {
            Instr::Nop(..) => 1,
            Instr::Acc(val) => {
                acc += val;
                1
            }
            Instr::Jmp(val) => val,
        };
    }
}

pub fn run() {
    let text = read_to_string("input/day08.txt").unwrap();
    let mut instr: Vec<_> = text
        .lines()
        .map(|s| {
            let val = s[4..].parse::<i32>().unwrap();
            match &s[..3] {
                "nop" => Instr::Nop(val),
                "acc" => Instr::Acc(val),
                "jmp" => Instr::Jmp(val),
                _ => panic!("unknown instruction"),
            }
        })
        .collect();

    println!("day08: accumulator at loop is {}", halt_check(&instr).1);

    for i in 0..instr.len() {
        if instr[i].swap_nop_jmp() {
            let (ok, acc) = halt_check(&instr);
            if ok {
                println!("day08: terminating accumulator is {}", acc);
            }
            instr[i].swap_nop_jmp();
        }
    }
}
