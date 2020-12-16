use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Debug, PartialEq, Clone)]
pub struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<u64>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket(Vec<u64>);

#[derive(Debug, PartialEq, Clone)]
pub struct Database {
    rules: Vec<Rule>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

peg::parser! {
    grammar database_parser() for str {
        rule number() -> u64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule range() -> RangeInclusive<u64>
            = n1:number() "-" n2:number() { n1..=n2 }

        rule range_list() -> Vec<RangeInclusive<u64>>
            = r:range() ** " or " { r }

        rule your_ticket() -> Ticket
            = "your ticket:\n" t:parse_ticket() { t }

        rule nearby_tickets() -> Vec<Ticket>
            = "nearby tickets:\n" t:parse_ticket() ** "\n" { t }

        pub rule parse_rule() -> Rule
            = name:$(['a'..='z' | 'A'..='Z' | ' ']+) ": " ranges:range_list() { Rule { name: name.to_string(), ranges } }

        pub rule parse_rule_list() -> Vec<Rule>
            = r:parse_rule() ** "\n" { r }

        pub rule parse_ticket() -> Ticket
            = n:number() **<1,> "," { Ticket(n) }

        pub rule parse_database() -> Database
            = r:parse_rule_list() "\n\n" y:your_ticket() "\n\n" n:nearby_tickets() "\n"? { Database { rules: r, your_ticket: y, nearby_tickets:n  } }
    }
}

impl Rule {
    fn matches(&self, v: u64) -> bool {
        self.ranges.iter().any(|r| r.contains(&v))
    }
}

impl Ticket {
    fn is_valid(&self, rules: &Vec<Rule>) -> bool {
        self.0.iter().all(|x| rules.iter().any(|r| r.matches(*x)))
    }

    fn calculate_error_rate(&self, rules: &Vec<Rule>) -> u64 {
        self.0
            .iter()
            .filter(|x| !rules.iter().any(|r| r.matches(**x)))
            .sum()
    }
}

impl Database {
    fn calculate_error_rate(&self) -> u64 {
        self.nearby_tickets
            .iter()
            .map(|t| t.calculate_error_rate(&self.rules))
            .sum()
    }

    fn discard_invalid_tickets(self) -> Self {
        let rules = self.rules;
        let nearby_tickets: Vec<_> = self
            .nearby_tickets
            .into_iter()
            .filter(|x| x.is_valid(&rules))
            .collect();
        Self {
            rules,
            your_ticket: self.your_ticket,
            nearby_tickets,
        }
    }

    fn find_columns_that_match(&self, r: &Rule) -> Vec<usize> {
        let total_columns = self.your_ticket.0.len();
        let mut result = Vec::new();
        for i in 0..total_columns {
            if self.nearby_tickets.iter().all(|t| r.matches(t.0[i])) {
                result.push(i);
            }
        }
        result
    }
}

fn read_input() -> Database {
    let data = std::fs::read_to_string("./inputs/day16.txt").unwrap();
    database_parser::parse_database(&data).unwrap()
}

fn part1() {
    let db = read_input();
    let err_rate = db.calculate_error_rate();
    println!("Part 1: {}", err_rate);
}

fn part2() {
    let db = read_input().discard_invalid_tickets();
    let possible_columns: Vec<_> = db
        .rules
        .iter()
        .map(|r| (r, db.find_columns_that_match(r)))
        .collect();
    let mut selected_columns = HashMap::new();
    loop {
        let singular = possible_columns.iter().find(|(_, cols)| {
            cols.iter()
                .filter(|c| !selected_columns.contains_key(c))
                .count()
                == 1
        });
        if let Some((r, c)) = singular {
            let c = c
                .iter()
                .filter(|x| !selected_columns.contains_key(x))
                .next()
                .unwrap();
            selected_columns.insert(c, r);
        } else {
            break;
        }
    }

    let mut result = 1;
    for (k, r) in &selected_columns {
        if r.name.starts_with("departure") {
            result *= db.your_ticket.0[**k];
        }
    }
    println!("Part 2: {}", result);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rule() {
        let r = database_parser::parse_rule("test: 1-2 or 10-12");
        assert_eq!(
            r,
            Ok(Rule {
                name: "test".to_string(),
                ranges: vec![1..=2, 10..=12]
            })
        );

        let r = database_parser::parse_rule("test with spaces: 1-2");
        assert_eq!(
            r,
            Ok(Rule {
                name: "test with spaces".to_string(),
                ranges: vec![1..=2]
            })
        );
    }

    #[test]
    fn parse_ticket() {
        let r = database_parser::parse_ticket("1,2,100");
        assert_eq!(r, Ok(Ticket(vec![1, 2, 100])));

        let r = database_parser::parse_ticket("2");
        assert_eq!(r, Ok(Ticket(vec![2])));
    }

    #[test]
    fn parse_database() {
        let r = database_parser::parse_database(
            r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#,
        )
        .unwrap();

        assert_eq!(
            r,
            Database {
                rules: vec![
                    Rule {
                        name: "class".to_string(),
                        ranges: vec![1..=3, 5..=7],
                    },
                    Rule {
                        name: "row".to_string(),
                        ranges: vec![6..=11, 33..=44],
                    },
                    Rule {
                        name: "seat".to_string(),
                        ranges: vec![13..=40, 45..=50],
                    },
                ],
                your_ticket: Ticket(vec![7, 1, 14]),
                nearby_tickets: vec![
                    Ticket(vec![7, 3, 47]),
                    Ticket(vec![40, 4, 50]),
                    Ticket(vec![55, 2, 20]),
                    Ticket(vec![38, 6, 12]),
                ],
            },
        )
    }
}
