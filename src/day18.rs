use nom::{
    branch::alt,
    character::complete::{char, digit1, space0},
    combinator::{all_consuming, map},
    multi::fold_many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Clone)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone)]
enum Expr {
    Constant(usize),
    Op(Box<Expr>, Op, Box<Expr>),
}

impl Expr {
    fn evaluate(&self) -> usize {
        match self {
            Expr::Constant(n) => *n,
            Expr::Op(a, op, b) => {
                let a = a.evaluate();
                let b = b.evaluate();
                match op {
                    Op::Add => a + b,
                    Op::Mul => a * b,
                }
            }
        }
    }
}

fn parse_constant(i: &str) -> IResult<&str, Expr> {
    map(digit1, |s: &str| {
        Expr::Constant(s.parse::<usize>().unwrap())
    })(i)
}

fn parse_term(i: &str) -> IResult<&str, Expr> {
    preceded(
        space0,
        alt((parse_constant, delimited(char('('), parse_expr, char(')')))),
    )(i)
}

fn parse_op(i: &str) -> IResult<&str, Op> {
    preceded(
        space0,
        alt((map(char('+'), |_| Op::Add), map(char('*'), |_| Op::Mul))),
    )(i)
}

fn parse_infix(i: &str) -> IResult<&str, (Op, Expr)> {
    tuple((parse_op, parse_term))(i)
}

fn parse_expr(i: &str) -> IResult<&str, Expr> {
    let (i, e) = parse_term(i)?;
    fold_many0(parse_infix, e, |acc, (op, e)| {
        Expr::Op(Box::new(acc), op, Box::new(e))
    })(i)
}

pub fn run() {
    let text = std::fs::read_to_string("input/day18.txt").unwrap();
    let expr: Vec<_> = text
        .lines()
        .map(|s| all_consuming(parse_expr)(s).unwrap().1)
        .collect();

    println!(
        "day18: sum of values is {}",
        expr.iter().map(|e| e.evaluate()).sum::<usize>()
    )
}
