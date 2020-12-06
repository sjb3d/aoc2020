use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, char, line_ending, multispace0, multispace1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use std::fs::read_to_string;

#[derive(Default)]
struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn set(&mut self, k: &'a str, v: &'a str) {
        match k {
            "byr" => self.byr = Some(v),
            "iyr" => self.iyr = Some(v),
            "eyr" => self.eyr = Some(v),
            "hgt" => self.hgt = Some(v),
            "hcl" => self.hcl = Some(v),
            "ecl" => self.ecl = Some(v),
            "pid" => self.pid = Some(v),
            "cid" => self.cid = Some(v),
            _ => panic!("unknown key"),
        }
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_alt(&self) -> bool {
        self.is_valid()
            && self
                .byr
                .and_then(|s| s.parse::<usize>().ok())
                .map(|n| matches!(n, 1920..=2002))
                .unwrap_or(false)
            && self
                .iyr
                .and_then(|s| s.parse::<usize>().ok())
                .map(|n| matches!(n, 2010..=2020))
                .unwrap_or(false)
            && self
                .eyr
                .and_then(|s| s.parse::<usize>().ok())
                .map(|n| matches!(n, 2020..=2030))
                .unwrap_or(false)
            && self
                .hgt
                .map(|s| {
                    if let Some(h) = s.strip_suffix("cm") {
                        matches!(h.parse::<usize>().unwrap(), 150..=193)
                    } else if let Some(h) = s.strip_suffix("in") {
                        matches!(h.parse::<usize>().unwrap(), 59..=76)
                    } else {
                        false
                    }
                })
                .unwrap_or(false)
            && self
                .hcl
                .map(|s| {
                    s.len() == 7
                        && s.starts_with("#")
                        && s[1..].chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
                })
                .unwrap_or(false)
            && self
                .ecl
                .map(|s| matches!(s, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"))
                .unwrap_or(false)
            && self
                .pid
                .map(|s| s.len() == 9 && s.chars().all(|c| matches!(c, '0'..='9')))
                .unwrap_or(false)
    }
}

fn is_ident(c: char) -> bool {
    matches!(c, 'a'..='z' | '#' | '0'..='9')
}

fn key_value(i: &str) -> IResult<&str, (&str, &str)> {
    let (i, k) = alpha1(i)?;
    let (i, _) = char(':')(i)?;
    let (i, v) = take_while1(is_ident)(i)?;
    Ok((i, (k, v)))
}

fn passport(i: &str) -> IResult<&str, Passport> {
    let (i, m) = separated_list1(alt((tag(" "), line_ending)), key_value)(i)?;
    let mut p = Passport::default();
    for (k, v) in m {
        p.set(k, v);
    }
    Ok((i, p))
}

pub fn run() {
    let text = read_to_string("input/day04.txt").unwrap();
    let passports = all_consuming(terminated(
        separated_list1(multispace1, passport),
        multispace0,
    ))(&text)
    .unwrap()
    .1;
    let valid_count = passports.iter().filter(|p| p.is_valid()).count();
    println!("day04: valid count is {}", valid_count);
    let alt_valid_count = passports.iter().filter(|p| p.is_valid_alt()).count();
    println!("day04: alt valid count is {}", alt_valid_count);
}
