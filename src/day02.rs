use nom::{
    character::complete::{alpha1, anychar, char, digit1, line_ending},
    combinator::{all_consuming, map_res},
    multi::many1,
    IResult,
};
use std::fs::read_to_string;

#[derive(Debug)]
struct Entry<'a> {
    a: usize,
    b: usize,
    c: char,
    pass: &'a str,
}

impl<'a> Entry<'a> {
    fn is_valid(&self) -> bool {
        let n = self.pass.chars().filter(|&c| c == self.c).count();
        self.a <= n && n <= self.b
    }

    fn is_valid_alt(&self) -> bool {
        let c0 = self.pass.chars().nth(self.a - 1).unwrap();
        let c1 = self.pass.chars().nth(self.b - 1).unwrap();
        (self.c == c0 || self.c == c1) && c0 != c1
    }
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let (i, a) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = char('-')(i)?;
    let (i, b) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, c) = anychar(i)?;
    let (i, _) = char(':')(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, pass) = alpha1(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Entry { a, b, c, pass }))
}

pub fn run() {
    let text = read_to_string("input/day02_input.txt").unwrap();
    let entries = all_consuming(many1(parse_entry))(&text).unwrap().1;
    let valid_count = entries.iter().filter(|e| e.is_valid()).count();
    let valid_alt_count = entries.iter().filter(|e| e.is_valid_alt()).count();
    println!("day02: valid count is {}", valid_count);
    println!("day02: alternate valid count is {}", valid_alt_count);
}
