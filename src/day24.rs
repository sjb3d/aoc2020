use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::many1,
    IResult,
};
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn parse_direction(i: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("e"), |_| Direction::East),
        map(tag("se"), |_| Direction::SouthEast),
        map(tag("sw"), |_| Direction::SouthWest),
        map(tag("w"), |_| Direction::West),
        map(tag("nw"), |_| Direction::NorthWest),
        map(tag("ne"), |_| Direction::NorthEast),
    ))(i)
}

fn parse_directions(i: &str) -> Vec<Direction> {
    all_consuming(many1(parse_direction))(i).unwrap().1
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Coord(i32, i32);

impl Coord {
    fn move_step(&mut self, direction: Direction) {
        let odd_row = self.1 & 1;
        let step = match direction {
            Direction::East => (1, 0),
            Direction::SouthEast => (odd_row, 1),
            Direction::SouthWest => (odd_row - 1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (odd_row - 1, -1),
            Direction::NorthEast => (odd_row, -1),
        };
        self.0 += step.0;
        self.1 += step.1;
    }
}

pub fn run() {
    let text = std::fs::read_to_string("input/day24.txt").unwrap();

    let mut flipped = HashMap::new();
    for line in text.lines() {
        let mut coord = Coord(0, 0);
        for dir in parse_directions(&line).iter().cloned() {
            coord.move_step(dir);
        }
        let flip = flipped.entry(coord).or_insert(false);
        *flip = !*flip;
    }
    println!(
        "day24: black tile count is {}",
        flipped.values().cloned().filter(|&b| b).count()
    );
}
