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

fn mask_valid_from_bits(bits: &[MaskBit]) -> u64 {
    bits.iter().fold(0, |acc, bit| {
        (acc << 1)
            | match bit {
                MaskBit::Ignored => 0,
                MaskBit::Set0 | MaskBit::Set1 => 1,
            }
    })
}

fn mask_set_from_bits(bits: &[MaskBit]) -> u64 {
    bits.iter().fold(0, |acc, bit| {
        (acc << 1)
            | match bit {
                MaskBit::Ignored | MaskBit::Set0 => 0,
                MaskBit::Set1 => 1,
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

    let mut mem = HashMap::new();
    let mut mask_valid = 0;
    let mut mask_set = 0;
    for cmd in commands.iter() {
        match cmd {
            Command::Mask { bits } => {
                mask_valid = mask_valid_from_bits(bits);
                mask_set = mask_set_from_bits(bits);
            }
            Command::Mem { address, value } => {
                let entry = mem.entry(address).or_insert(0);
                *entry = (value & !mask_valid) | mask_set;
            }
        }
    }
    println!(
        "day14: sum of memory values is {}",
        mem.values().sum::<u64>()
    );
}
