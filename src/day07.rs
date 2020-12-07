use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0, multispace1},
    combinator::{all_consuming, map, map_res, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct BagName<'a>(&'a str, &'a str);

struct Bag<'a> {
    name: BagName<'a>,
    contents: Vec<(usize, BagName<'a>)>,
}

fn bag_name(i: &str) -> IResult<&str, BagName> {
    let (i, (a, b)) = separated_pair(alpha1, multispace1, alpha1)(i)?;
    Ok((i, BagName(a, b)))
}

fn contents(i: &str) -> IResult<&str, (usize, BagName)> {
    let (i, count) = preceded(multispace0, map_res(digit1, |s: &str| s.parse::<usize>()))(i)?;
    let (i, name) = preceded(multispace1, bag_name)(i)?;
    let (i, _) = preceded(multispace1, tag("bag").and(opt(char('s'))))(i)?;
    Ok((i, (count, name)))
}

fn bag(i: &str) -> IResult<&str, Bag> {
    let (i, name) = preceded(multispace0, bag_name)(i)?;
    let (i, _) = delimited(
        multispace1,
        separated_pair(tag("bags"), multispace1, tag("contain")),
        multispace1,
    )(i)?;
    let (i, contents) = terminated(
        alt((
            map(tag("no other bags"), |_| Vec::new()),
            separated_list1(char(','), contents),
        )),
        char('.'),
    )(i)?;
    Ok((i, Bag { name, contents }))
}

fn count_bags(name: BagName, bags: &[Bag]) -> usize {
    let bag = bags.iter().find(|b| b.name == name).unwrap();
    1 + bag
        .contents
        .iter()
        .map(|c| c.0 * count_bags(c.1, bags))
        .sum::<usize>()
}

pub fn run() {
    let text = read_to_string("input/day07.txt").unwrap();
    let bags = all_consuming(terminated(separated_list1(multispace0, bag), multispace0))(&text)
        .unwrap()
        .1;

    let root_name = BagName("shiny", "gold");

    let mut active = HashSet::new();
    active.insert(root_name);
    loop {
        let mut changed = false;
        for bag in bags.iter() {
            if bag.contents.iter().any(|c| active.contains(&c.1)) {
                if active.insert(bag.name) {
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    println!("day07: shiny gold containers {}", active.len() - 1);

    println!(
        "day07: contained bag count is {}",
        count_bags(root_name, &bags) - 1
    );
}
