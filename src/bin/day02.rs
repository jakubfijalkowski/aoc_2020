use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq)]
struct PasswordRule(usize, usize, char);
#[derive(Debug, PartialEq)]
struct Password(PasswordRule, String);

impl PasswordRule {
    fn matches_old(&self, s: &str) -> bool {
        let &PasswordRule(min, max, q) = self;
        let cnt = s.chars().filter(|&c| c == q).count();
        min <= cnt && cnt <= max
    }

    fn matches_new(&self, s: &str) -> bool {
        let &PasswordRule(min, max, q) = self;
        let min = min - 1;
        let max = max - 1;
        let a = min < s.len() && s.chars().nth(min).unwrap() == q;
        let b = max < s.len() && s.chars().nth(max).unwrap() == q;
        a ^ b
    }
}

impl Password {
    fn parse(s: &str) -> Password {
        let hypen = s.find('-').unwrap();
        let space = s.find(' ').unwrap();
        let colon = s.find(':').unwrap();

        let rule = PasswordRule(
            s[0..hypen].parse().unwrap(),
            s[hypen + 1..space].parse().unwrap(),
            s.chars().nth(space + 1).unwrap(),
        );
        Password(rule, s[colon + 2..].to_string())
    }
}

fn read_input() -> Vec<Password> {
    BufReader::new(fs::File::open("./inputs/day02.txt").unwrap())
        .lines()
        .map(|x| Password::parse(&x.unwrap()))
        .collect()
}

fn part1() {
    let pass = read_input()
        .iter()
        .filter(|Password(rule, p)| rule.matches_old(&p))
        .count();
    println!("Day 1: {}", pass);
}

fn part2() {
    let pass = read_input()
        .iter()
        .filter(|Password(rule, p)| rule.matches_new(&p))
        .count();
    println!("Day 2: {}", pass);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let p = Password::parse(&"2-4 q: qqqq");
        assert_eq!(Password(PasswordRule(2, 4, 'q'), "qqqq".to_string()), p);
    }

    #[test]
    fn rule_matches_old() {
        assert_eq!(true, PasswordRule(1, 2, 'q').matches_old(&"q"));
        assert_eq!(true, PasswordRule(1, 2, 'q').matches_old(&"qq"));
        assert_eq!(true, PasswordRule(1, 2, 'q').matches_old(&"abbq123q"));
        assert_eq!(true, PasswordRule(1, 2, 'q').matches_old(&"abbq123q"));

        assert_eq!(false, PasswordRule(1, 2, 'q').matches_old(&""));
        assert_eq!(false, PasswordRule(1, 2, 'q').matches_old(&"qqq"));
        assert_eq!(false, PasswordRule(1, 2, 'q').matches_old(&"qqq"));
        assert_eq!(false, PasswordRule(1, 2, 'q').matches_old(&"aqbbq123q"));
    }

    #[test]
    fn rule_matches_new() {
        assert_eq!(true, PasswordRule(1, 3, 'a').matches_new(&"abcde"));
        assert_eq!(false, PasswordRule(1, 3, 'b').matches_new(&"cdefg"));
        assert_eq!(false, PasswordRule(2, 9, 'c').matches_new(&"ccccccccc"));

        assert_eq!(true, PasswordRule(1, 3, 'a').matches_new(&"ab"));
        assert_eq!(false, PasswordRule(1, 3, 'a').matches_new(&"ba"));
    }
}
