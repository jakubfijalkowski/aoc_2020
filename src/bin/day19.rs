use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Rule {
    Single(char),
    Concatenation(Vec<usize>),
    Alternative(Vec<Rule>),
}

pub struct Puzzle {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

peg::parser! {
    grammar rules() for str {
        rule _() = [' ']*

        rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule single() -> Rule
            = _ "\"" c:$(['a'..='b']) "\"" _ { Rule::Single(c.chars().next().unwrap()) }

        rule concatenation() -> Rule
            = _ n:number() **<1, > " " _ { Rule::Concatenation(n) }

        rule alternative() -> Rule
            = c:concatenation() **<2,> "|" { Rule::Alternative(c) }

        rule expr() -> Rule
            = single()
            / alternative()
            / concatenation()

        rule message() -> String
            = s:$(['a'..='b']+) { s.to_owned() }

        pub rule parse() -> (usize, Rule)
            = _ n:number() _ ":" _ e:expr() { (n, e) }

        pub rule parse_list() -> HashMap<usize, Rule>
            = l:parse() ** "\n" { l.into_iter().collect() }

        pub rule parse_puzzle() -> Puzzle
            = rules:parse_list() "\n\n" messages:message() ** "\n" "\n"? { Puzzle { rules, messages } }
    }
}

fn try_match(rules: &HashMap<usize, Rule>, rule_no: usize, s: &str) -> Vec<usize> {
    try_match_rule(rules, rules.get(&rule_no).unwrap(), s)
}

fn try_match_concat(
    rules: &HashMap<usize, Rule>,
    result: &mut Vec<usize>,
    inner: &[usize],
    base_str: &str,
    offset: usize,
) {
    if inner.is_empty() {
        result.push(offset);
        return;
    } else if offset > base_str.len() {
        return;
    }

    let matches = try_match(rules, inner[0], &base_str[offset..]);
    for m in matches {
        try_match_concat(rules, result, &inner[1..], base_str, offset + m);
    }
}

fn try_match_rule(rules: &HashMap<usize, Rule>, rule: &Rule, s: &str) -> Vec<usize> {
    match rule {
        Rule::Single(c) => {
            if s.chars().next() == Some(*c) {
                vec![1]
            } else {
                vec![]
            }
        }
        Rule::Concatenation(inner) => {
            let mut result = Vec::new();
            try_match_concat(rules, &mut result, &inner[..], s, 0);
            result
        }
        Rule::Alternative(inner) => {
            let mut result = Vec::new();
            for r in inner {
                let matches = try_match_rule(rules, r, s);
                result.extend(matches.iter());
            }
            result
        }
    }
}

fn is_match(rules: &HashMap<usize, Rule>, rule_no: usize, s: &str) -> bool {
    try_match(rules, rule_no, s)
        .into_iter()
        .any(|l| l == s.len())
}

fn part1() {
    let s = include_str!("../../inputs/day19.txt");
    let puzzle = rules::parse_puzzle(s).unwrap();
    let cnt = puzzle
        .messages
        .iter()
        .filter(|m| is_match(&puzzle.rules, 0, m))
        .count();
    println!("Part 1: {}", cnt);
}

fn part2() {
    let s = include_str!("../../inputs/day19.txt");
    let mut puzzle = rules::parse_puzzle(s).unwrap();

    puzzle.rules.insert(
        8,
        Rule::Alternative(vec![
            Rule::Concatenation(vec![42]),
            Rule::Concatenation(vec![42, 8]),
        ]),
    );
    puzzle.rules.insert(
        11,
        Rule::Alternative(vec![
            Rule::Concatenation(vec![42, 31]),
            Rule::Concatenation(vec![42, 11, 31]),
        ]),
    );
    let cnt = puzzle
        .messages
        .iter()
        .filter(|m| is_match(&puzzle.rules, 0, m))
        .count();
    println!("Part 2: {}", cnt);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single() {
        let r = rules::parse("1: \"a\"").unwrap();

        assert_eq!((1, Rule::Single('a')), r);
    }

    #[test]
    fn parse_concatenation() {
        let r = rules::parse("3: 1 2 3").unwrap();

        assert_eq!((3, Rule::Concatenation(vec![1, 2, 3])), r);
    }

    #[test]
    fn parse_alternative() {
        let r = rules::parse("3: 1 2 | 4 5").unwrap();

        assert_eq!(
            (
                3,
                Rule::Alternative(vec![
                    Rule::Concatenation(vec![1, 2]),
                    Rule::Concatenation(vec![4, 5])
                ])
            ),
            r
        );
    }

    #[test]
    fn parse_list() {
        let r = rules::parse_list("1: \"a\"\n2: 1 1").unwrap();

        assert_eq!(
            vec![(1, Rule::Single('a')), (2, Rule::Concatenation(vec![1, 1]))]
                .into_iter()
                .collect::<HashMap<usize, Rule>>(),
            r
        );
    }

    #[test]
    fn parse_puzzle() {
        let r = rules::parse_puzzle("1: \"a\"\n2: 1 1\n\naabb\nabab").unwrap();

        assert_eq!(
            vec![(1, Rule::Single('a')), (2, Rule::Concatenation(vec![1, 1]))]
                .into_iter()
                .collect::<HashMap<usize, Rule>>(),
            r.rules
        );
        assert_eq!(vec!["aabb".to_string(), "abab".to_string()], r.messages);
    }

    #[test]
    fn try_match_single() {
        let mut rules = HashMap::new();
        rules.insert(0usize, Rule::Single('a'));
        rules.insert(1, Rule::Single('b'));

        assert_eq!(try_match(&rules, 0, "a"), vec![1]);
        assert_eq!(try_match(&rules, 1, "b"), vec![1]);
        assert_eq!(try_match(&rules, 1, "a"), vec![]);
        assert_eq!(try_match(&rules, 0, "b"), vec![]);
    }

    #[test]
    fn try_match_concatenation() {
        let rules =
            rules::parse_list("1: \"a\"\n2: \"b\"\n3: 1 1\n4: 1 2\n5: 2 1 2\n6: 3 4").unwrap();

        assert_eq!(try_match(&rules, 3, "aa"), vec![2]);
        assert_eq!(try_match(&rules, 4, "ab"), vec![2]);
        assert_eq!(try_match(&rules, 5, "bab"), vec![3]);
        assert_eq!(try_match(&rules, 6, "aaab"), vec![4]);
        assert_eq!(try_match(&rules, 5, "babbbb"), vec![3]);

        assert_eq!(try_match(&rules, 5, "aabbbb"), vec![]);
        assert_eq!(try_match(&rules, 6, "aaa"), vec![]);
    }

    #[test]
    fn try_match_alternative_simple() {
        let rules = rules::parse_list("1: \"a\"\n2: \"b\"\n3: 1 | 2\n4: 1 | 1").unwrap();

        assert_eq!(try_match(&rules, 3, "a"), vec![1]);
        assert_eq!(try_match(&rules, 3, "b"), vec![1]);
        assert_eq!(try_match(&rules, 4, "a"), vec![1, 1]);
        assert_eq!(try_match(&rules, 3, "c"), vec![]);
    }

    #[test]
    fn try_match_alternative_complex() {
        let rules = rules::parse_list("1: \"a\"\n2: \"b\"\n3: 1 2\n4: 2 1\n5: 3 | 4\n6: 3 3 | 4 4")
            .unwrap();

        assert_eq!(try_match(&rules, 5, "ab"), vec![2]);
        assert_eq!(try_match(&rules, 5, "ba"), vec![2]);
        assert_eq!(try_match(&rules, 5, "abb"), vec![2]);
        assert_eq!(try_match(&rules, 5, "baa"), vec![2]);

        assert_eq!(try_match(&rules, 6, "abab"), vec![4]);
        assert_eq!(try_match(&rules, 6, "abab"), vec![4]);
        assert_eq!(try_match(&rules, 6, "baba"), vec![4]);
        assert_eq!(try_match(&rules, 6, "abba"), vec![]);
    }
}
