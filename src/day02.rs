use nom::{
    character::complete::{alpha1, anychar, char, digit1, line_ending},
    combinator::{all_consuming, map_res},
    multi::many1,
    IResult,
};
use std::fs::read_to_string;

#[derive(Debug)]
struct Entry<'a> {
    min: usize,
    max: usize,
    c: char,
    pass: &'a str,
}

impl<'a> Entry<'a> {
    fn is_valid(&self) -> bool {
        let n = self.pass.chars().filter(|&c| c == self.c).count();
        self.min <= n && n <= self.max
    }
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let (i, min) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = char('-')(i)?;
    let (i, max) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, c) = anychar(i)?;
    let (i, _) = char(':')(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, pass) = alpha1(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Entry { min, max, c, pass }))
}

pub fn run() {
    let text = read_to_string("input/day02_input.txt").unwrap();
    let entries = all_consuming(many1(parse_entry))(&text).unwrap().1;
    let valid_count = entries.iter().filter(|e| e.is_valid()).count();
    println!("day02: valid count is {}", valid_count);
}
