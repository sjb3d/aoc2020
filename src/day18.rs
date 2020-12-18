use nom::{
    branch::alt,
    character::complete::{char, digit1, space0},
    combinator::{all_consuming, map},
    multi::{fold_many0, fold_many_m_n},
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

fn parse_op(i: &str) -> IResult<&str, Op> {
    preceded(
        space0,
        alt((map(char('+'), |_| Op::Add), map(char('*'), |_| Op::Mul))),
    )(i)
}

fn parse_term1(i: &str) -> IResult<&str, Expr> {
    preceded(
        space0,
        alt((parse_constant, delimited(char('('), parse_expr1, char(')')))),
    )(i)
}

fn parse_infix1(i: &str) -> IResult<&str, (Op, Expr)> {
    tuple((parse_op, parse_term1))(i)
}

fn parse_expr1(i: &str) -> IResult<&str, Expr> {
    let (i, e) = parse_term1(i)?;
    fold_many0(parse_infix1, e, |acc, (op, e)| {
        Expr::Op(Box::new(acc), op, Box::new(e))
    })(i)
}

fn parse_term2(i: &str) -> IResult<&str, Expr> {
    preceded(
        space0,
        alt((parse_constant, delimited(char('('), parse_expr2, char(')')))),
    )(i)
}

fn parse_infix2(i: &str) -> IResult<&str, (Op, Expr)> {
    tuple((parse_op, parse_term2))(i)
}

fn parse_expr2(i: &str) -> IResult<&str, Expr> {
    let (i, e) = parse_term2(i)?;
    let (i, e) = fold_many_m_n(0, 1, parse_infix2, e, |acc, (op, e)| {
        Expr::Op(Box::new(acc), op, Box::new(e))
    })(i)?;
    fold_many0(parse_infix2, e, |acc, (op, e)| match op {
        Op::Add => match acc {
            Expr::Constant(_) => panic!("invalid state"),
            Expr::Op(a, a_op_b, b) => Expr::Op(a, a_op_b, Box::new(Expr::Op(b, op, Box::new(e)))),
        },
        Op::Mul => Expr::Op(Box::new(acc), op, Box::new(e)),
    })(i)
}

pub fn run() {
    let text = std::fs::read_to_string("input/day18.txt").unwrap();
    let expr1: Vec<_> = text
        .lines()
        .map(|s| all_consuming(parse_expr1)(s).unwrap().1)
        .collect();

    println!(
        "day18: sum of values is {}",
        expr1.iter().map(|e| e.evaluate()).sum::<usize>()
    );

    let expr2: Vec<_> = text
        .lines()
        .map(|s| all_consuming(parse_expr2)(s).unwrap().1)
        .collect();

    println!(
        "day18: alt sum of values is {}",
        expr2.iter().map(|e| e.evaluate()).sum::<usize>()
    );
}
