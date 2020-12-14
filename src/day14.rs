use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{all_consuming, map, map_res},
    multi::many1,
    sequence::terminated,
    IResult,
};
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum MaskBit {
    Ignored,
    Set0,
    Set1,
}

#[derive(Clone)]
enum Command {
    Mask { bits: Vec<MaskBit> },
    Mem { address: u64, value: u64 },
}

fn ignored_mask_from_bits(bits: &[MaskBit]) -> u64 {
    bits.iter().fold(0, |acc, bit| {
        (acc << 1)
            | match bit {
                MaskBit::Ignored => 1,
                MaskBit::Set0 | MaskBit::Set1 => 0,
            }
    })
}

fn set1_mask_from_bits(bits: &[MaskBit]) -> u64 {
    bits.iter().fold(0, |acc, bit| {
        (acc << 1)
            | match bit {
                MaskBit::Set1 => 1,
                MaskBit::Ignored | MaskBit::Set0 => 0,
            }
    })
}

fn mask_bit(i: &str) -> IResult<&str, MaskBit> {
    alt((
        map(char('X'), |_| MaskBit::Ignored),
        map(char('0'), |_| MaskBit::Set0),
        map(char('1'), |_| MaskBit::Set1),
    ))(i)
}

fn mask(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("mask = ")(i)?;
    let (i, bits) = many1(mask_bit)(i)?;
    Ok((i, Command::Mask { bits }))
}

fn mem(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("mem[")(i)?;
    let (i, address) = map_res(digit1, |s: &str| s.parse::<u64>())(i)?;
    let (i, _) = tag("] = ")(i)?;
    let (i, value) = map_res(digit1, |s: &str| s.parse::<u64>())(i)?;
    Ok((i, Command::Mem { address, value }))
}

pub fn run() {
    let text = std::fs::read_to_string("input/day14.txt").unwrap();
    let commands: Vec<_> = all_consuming(many1(terminated(alt((mask, mem)), line_ending)))(&text)
        .unwrap()
        .1;

    let mut mem1 = HashMap::new();
    let mut mem2 = HashMap::new();
    let mut mask_ignored = 0;
    let mut mask_set1 = 0;
    for cmd in commands.iter() {
        match cmd {
            Command::Mask { bits } => {
                mask_ignored = ignored_mask_from_bits(bits);
                mask_set1 = set1_mask_from_bits(bits);
            }
            Command::Mem { address, value } => {
                mem1.insert(address, (value & mask_ignored) | mask_set1);

                let bit_count = mask_ignored.count_ones();
                for i in 0..(1 << bit_count) {
                    let mut mask_floating = 0;
                    let mut counter_bit_index = 0;
                    for mask_bit_index in 0..64 {
                        let mask_bit = 1u64 << mask_bit_index;
                        if (mask_ignored & mask_bit) != 0 {
                            if i & (1 << counter_bit_index) != 0 {
                                mask_floating |= mask_bit;
                            }
                            counter_bit_index += 1;
                        }
                    }
                    mem2.insert(
                        (address & !mask_ignored) | mask_floating | mask_set1,
                        *value,
                    );
                }
            }
        }
    }
    println!(
        "day14: sum of memory values is {}",
        mem1.values().sum::<u64>()
    );
    println!(
        "day14: alt sum of memory values is {}",
        mem2.values().sum::<u64>()
    );
}
