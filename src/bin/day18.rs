use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Number(i64),
    Sum(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
}

peg::parser! {
    grammar arithmetic() for str {
        rule number() -> Expression
            = n:$(['0'..='9']+) { Expression::Number(n.parse().unwrap()) }

        rule _() = [' ' | '\n']*

        pub rule expression_flat() -> Expression = precedence!{
            x:(@) _ o:$(['+' | '*']) _ y:@ { if o == "+" { Expression::Sum(Box::new(x), Box::new(y)) } else { Expression::Product(Box::new(x), Box::new(y)) }}
            --
            "(" v:expression_flat() ")" { v }
            n:number() { n }
        }

        pub rule expression_reverse() -> Expression = precedence!{
            x:(@) _ "*" _ y:@ { Expression::Product(Box::new(x), Box::new(y)) }
            --
            x:(@) _ "+" _ y:@ { Expression::Sum(Box::new(x), Box::new(y)) }
            --
            "(" v:expression_reverse() ")" { v }
            n:number() { n }
        }
    }
}

impl Expression {
    pub fn evaluate(&self) -> i64 {
        match self {
            Self::Number(n) => *n,
            Self::Sum(a, b) => a.evaluate() + b.evaluate(),
            Self::Product(a, b) => a.evaluate() * b.evaluate(),
        }
    }
}

fn read_input_flat() -> Vec<Expression> {
    BufReader::new(fs::File::open("./inputs/day18.txt").unwrap())
        .lines()
        .map(|x| arithmetic::expression_flat(&x.unwrap()).unwrap())
        .collect()
}

fn read_input_reverse() -> Vec<Expression> {
    BufReader::new(fs::File::open("./inputs/day18.txt").unwrap())
        .lines()
        .map(|x| arithmetic::expression_reverse(&x.unwrap()).unwrap())
        .collect()
}

fn part1() {
    let inputs = read_input_flat();
    let res: i64 = inputs.iter().map(|x| x.evaluate()).sum();
    println!("Part 1: {}", res);
}

fn part2() {
    let inputs = read_input_reverse();
    let res: i64 = inputs.iter().map(|x| x.evaluate()).sum();
    println!("Part 2: {}", res);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn airthmetic_parse() {
        assert_eq!(
            arithmetic::expression_flat("1+1"),
            Ok(Expression::Sum(
                Box::new(Expression::Number(1)),
                Box::new(Expression::Number(1))
            ))
        );
        assert_eq!(
            arithmetic::expression_flat("1*1"),
            Ok(Expression::Product(
                Box::new(Expression::Number(1)),
                Box::new(Expression::Number(1))
            ))
        );
        assert_eq!(
            arithmetic::expression_flat("1+1+2"),
            Ok(Expression::Sum(
                Box::new(Expression::Sum(
                    Box::new(Expression::Number(1)),
                    Box::new(Expression::Number(1)),
                )),
                Box::new(Expression::Number(2))
            ))
        );

        assert_eq!(
            arithmetic::expression_flat("2+3*4"),
            Ok(Expression::Product(
                Box::new(Expression::Sum(
                    Box::new(Expression::Number(2)),
                    Box::new(Expression::Number(3))
                )),
                Box::new(Expression::Number(4))
            ))
        );

        assert_eq!(
            arithmetic::expression_flat("2+(3*4)"),
            Ok(Expression::Sum(
                Box::new(Expression::Number(2)),
                Box::new(Expression::Product(
                    Box::new(Expression::Number(3)),
                    Box::new(Expression::Number(4))
                ))
            ))
        );
    }

    #[test]
    fn evaluate_flat() {
        assert_eq!(
            71,
            arithmetic::expression_flat("1 + 2 * 3 + 4 * 5 + 6")
                .unwrap()
                .evaluate()
        );

        assert_eq!(
            51,
            arithmetic::expression_flat("1 + (2 * 3) + (4 * (5 + 6))")
                .unwrap()
                .evaluate()
        );
    }

    #[test]
    fn evaluate_rev() {
        assert_eq!(
            231,
            arithmetic::expression_reverse("1 + 2 * 3 + 4 * 5 + 6")
                .unwrap()
                .evaluate()
        );

        assert_eq!(
            51,
            arithmetic::expression_reverse("1 + (2 * 3) + (4 * (5 + 6))")
                .unwrap()
                .evaluate()
        );

        assert_eq!(
            669060,
            arithmetic::expression_reverse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
                .unwrap()
                .evaluate()
        );
    }
}
