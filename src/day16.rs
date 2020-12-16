use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    IResult,
};

struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn is_valid(&self, n: usize) -> bool {
        self.min <= n && n <= self.max
    }
}

struct Field {
    _name: String,
    ranges: [Range; 2],
}

fn number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

fn range(i: &str) -> IResult<&str, Range> {
    let (i, min) = number(i)?;
    let (i, _) = char('-')(i)?;
    let (i, max) = number(i)?;
    Ok((i, Range { min, max }))
}

fn field(i: &str) -> IResult<&str, Field> {
    let (i, name) = take_until(":")(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, range0) = range(i)?;
    let (i, _) = tag(" or ")(i)?;
    let (i, range1) = range(i)?;
    Ok((
        i,
        Field {
            _name: name.to_owned(),
            ranges: [range0, range1],
        },
    ))
}

fn parse_ticket(i: &str) -> Vec<usize> {
    all_consuming(separated_list1(char(','), number))(i)
        .unwrap()
        .1
}

pub fn run() {
    let text = std::fs::read_to_string("input/day16.txt").unwrap();
    let mut lines = text.lines();

    let mut fields = Vec::new();
    for _ in 0..20 {
        let s = lines.next().unwrap();
        fields.push(all_consuming(field)(s).unwrap().1);
    }

    lines.next();
    if lines.next() != Some("your ticket:") {
        panic!("unexpected input");
    }
    let _my_ticket = parse_ticket(lines.next().unwrap());

    lines.next();
    if lines.next() != Some("nearby tickets:") {
        panic!("unexpected input");
    }
    let nearby_tickets: Vec<_> = lines.map(|s| parse_ticket(s)).collect();

    let mut invalid_field_sum = 0;
    for ticket in nearby_tickets.iter() {
        for n in ticket.iter().cloned() {
            if !fields
                .iter()
                .any(|f| f.ranges[0].is_valid(n) || f.ranges[1].is_valid(n))
            {
                invalid_field_sum += n;
            }
        }
    }
    println!("day16: invalid field sum is {}", invalid_field_sum);
}
