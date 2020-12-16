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
    fn is_valid(&self, value: usize) -> bool {
        self.min <= value && value <= self.max
    }
}

struct Field {
    name: String,
    ranges: [Range; 2],
    index_mask: usize,
}

impl Field {
    fn is_valid(&self, value: usize) -> bool {
        self.ranges[0].is_valid(value) || self.ranges[1].is_valid(value)
    }
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
            name: name.to_owned(),
            ranges: [range0, range1],
            index_mask: (1 << 20) - 1,
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
    let my_ticket = parse_ticket(lines.next().unwrap());

    lines.next();
    if lines.next() != Some("nearby tickets:") {
        panic!("unexpected input");
    }
    let nearby_tickets: Vec<_> = lines.map(|s| parse_ticket(s)).collect();

    let mut valid_tickets = Vec::new();
    let mut invalid_field_sum = 0;
    for ticket in nearby_tickets.iter() {
        let mut ticket_is_valid = true;
        for value in ticket.iter().cloned() {
            if !fields.iter().any(|f| f.is_valid(value)) {
                invalid_field_sum += value;
                ticket_is_valid = false;
            }
        }
        if ticket_is_valid {
            valid_tickets.push(ticket.clone());
        }
    }
    println!("day16: invalid field sum is {}", invalid_field_sum);

    valid_tickets.push(my_ticket.clone());
    for ticket in valid_tickets.iter() {
        for (i, value) in ticket.iter().cloned().enumerate() {
            for field in fields.iter_mut() {
                if !field.is_valid(value) {
                    field.index_mask &= !(1 << i);
                }
            }
        }
    }
    let mut unknown_mask = (1 << 20) - 1;
    while unknown_mask != 0 {
        for field in fields.iter_mut() {
            if field.index_mask.count_ones() == 1 {
                unknown_mask &= !field.index_mask;
            } else {
                field.index_mask &= unknown_mask;
            }
        }
    }

    let mut departure_product = 1;
    for field in fields.iter() {
        if field.name.starts_with("departure") {
            let index = field.index_mask.trailing_zeros();
            departure_product *= my_ticket[index as usize];
        }
    }
    println!("day16: departure product is {}", departure_product);
}
