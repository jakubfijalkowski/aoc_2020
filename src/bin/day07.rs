use std::collections::HashMap;
use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Color(String);

#[derive(Debug, PartialEq)]
struct Rule {
    color: Color,
    contain: Vec<(Color, u32)>,
}

impl Rule {
    fn parse(s: &str) -> Rule {
        let color_end = s.find("bags").unwrap();
        let contain_start = s.find("contain").unwrap();
        let color = Color(s[..color_end].trim().to_string());

        let contain_str = &s[contain_start + 8..s.len() - 1];

        let contain: Vec<_> = if contain_str == "no other bags" {
            vec![]
        } else {
            contain_str
                .split(',')
                .map(|s| Rule::parse_contain(s.trim()))
                .collect()
        };
        Rule { color, contain }
    }

    fn parse_contain(s: &str) -> (Color, u32) {
        let space = s.find(' ').unwrap();
        let bags = s.find("bag").unwrap();
        let color = s[space + 1..bags - 1].to_owned();
        let count = s[..space].parse().unwrap();
        (Color(color), count)
    }
}

fn read_input() -> HashMap<Color, Rule> {
    let lines: Vec<_> = BufReader::new(fs::File::open("./inputs/day07.txt").unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect();
    read_lines(&lines[..])
}

fn read_lines<T: AsRef<str>>(lines: &[T]) -> HashMap<Color, Rule> {
    lines
        .iter()
        .map(|x| Rule::parse(x.as_ref()))
        .map(|x| (x.color.clone(), x))
        .collect()
}

fn contains(map: &HashMap<Color, Rule>, color: &Color, what: &Color) -> bool {
    let rule = map.get(color).unwrap();
    let is_direct = rule.contain.iter().any(|(c, _)| c == what);
    is_direct || rule.contain.iter().any(|(c, _)| contains(map, c, what))
}

fn count_inner(map: &HashMap<Color, Rule>, color: &Color) -> u32 {
    let rule = map.get(color).unwrap();
    rule.contain
        .iter()
        .map(|(color, cnt)| cnt + cnt * count_inner(map, color))
        .sum()
}

fn part1() {
    const TARGET: &'static str = "shiny gold";
    let input = read_input();

    let what = Color(TARGET.to_string());
    let count = input
        .iter()
        .filter(|x| contains(&input, x.0, &what))
        .count();
    println!("Part 1: {}", count);
}

fn part2() {
    const TARGET: &'static str = "shiny gold";
    let input = read_input();

    let what = Color(TARGET.to_string());
    let count = count_inner(&input, &what);
    println!("Part 2: {}", count);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_parse() {
        let p = Rule::parse("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert_eq!(
            p,
            Rule {
                color: Color("light red".to_string()),
                contain: vec![
                    (Color("bright white".to_string()), 1),
                    (Color("muted yellow".to_string()), 2)
                ]
            }
        );

        let p = Rule::parse("faded blue bags contain no other bags.");
        assert_eq!(
            p,
            Rule {
                color: Color("faded blue".to_owned()),
                contain: vec![]
            }
        );
    }

    #[test]
    fn contains_singlerule() {
        let p = Rule::parse("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        let mut hm = HashMap::new();
        hm.insert(p.color.clone(), p);

        let search_in = Color("light red".to_string());
        assert_eq!(
            true,
            contains(&hm, &search_in, &Color("bright white".to_string()))
        );
        assert_eq!(
            true,
            contains(&hm, &search_in, &Color("bright white".to_string()))
        );
    }

    #[test]
    fn count_inner_example() {
        let hm = read_lines(&[
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bags.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ]);

        assert_eq!(0, count_inner(&hm, &Color("faded blue".to_owned())));
        assert_eq!(0, count_inner(&hm, &Color("dotted black".to_owned())));
        assert_eq!(11, count_inner(&hm, &Color("vibrant plum".to_owned())));
        assert_eq!(7, count_inner(&hm, &Color("dark olive".to_owned())));
        assert_eq!(32, count_inner(&hm, &Color("shiny gold".to_owned())));
    }
}
