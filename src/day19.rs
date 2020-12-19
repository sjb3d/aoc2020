use nom::{
    branch::alt,
    character::complete::{anychar, char, digit1, space0},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;

enum Rule {
    Atom(char),
    Tuples(Vec<Vec<u32>>),
}

fn parse_number(i: &str) -> IResult<&str, u32> {
    map(digit1, |s: &str| s.parse::<u32>().unwrap())(i)
}

fn parse_tuple(i: &str) -> IResult<&str, Vec<u32>> {
    many1(preceded(space0, parse_number))(i)
}

fn parse_tuples(i: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(preceded(space0, char('|')), parse_tuple)(i)
}

fn parse_atom(i: &str) -> IResult<&str, char> {
    preceded(space0, delimited(char('"'), anychar, char('"')))(i)
}

fn parse_rule(i: &str) -> IResult<&str, Rule> {
    alt((
        map(parse_atom, |a| Rule::Atom(a)),
        map(parse_tuples, |t| Rule::Tuples(t)),
    ))(i)
}

fn parse_rule_decl(i: &str) -> IResult<&str, (u32, Rule)> {
    separated_pair(parse_number, char(':'), parse_rule)(i)
}

struct State<'a> {
    rules: Vec<u32>,
    remain: &'a str,
}

fn is_match(rules: &HashMap<u32, Rule>, s: &str) -> bool {
    let mut stack = Vec::new();
    stack.push(State {
        rules: vec![0],
        remain: s,
    });

    while let Some(mut state) = stack.pop() {
        if let Some(n) = state.rules.pop() {
            if !state.remain.is_empty() {
                match rules.get(&n).unwrap() {
                    Rule::Atom(c) => {
                        if let Some(tail) = state.remain.strip_prefix(*c) {
                            state.remain = tail;
                            stack.push(state);
                        }
                    }
                    Rule::Tuples(v) => {
                        for t in v.iter() {
                            let mut new_rules = state.rules.clone();
                            new_rules.extend(t.iter().rev());
                            stack.push(State {
                                rules: new_rules,
                                remain: state.remain,
                            });
                        }
                    }
                }
            }
        } else {
            return state.remain.is_empty();
        }
    }
    false
}

pub fn run() {
    let text = std::fs::read_to_string("input/day19.txt").unwrap();

    let mut lines = text.lines();
    let mut rules = HashMap::new();
    loop {
        let s = lines.next().unwrap();
        if s.is_empty() {
            break;
        }
        let (n, rule) = all_consuming(parse_rule_decl)(s).unwrap().1;
        rules.insert(n, rule);
    }

    let messages: Vec<_> = lines.collect();
    println!(
        "day19: number of matches is {}",
        messages.iter().filter(|s| is_match(&rules, s)).count()
    );
}
