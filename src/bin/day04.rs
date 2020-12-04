use std::collections::HashSet;

use lazy_static::lazy_static;
use thiserror::Error;

lazy_static! {
    static ref VALID_FIELDS: HashSet<&'static str> = {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
            .into_iter()
            .collect()
    };
    static ref MANDATORY_FIELDS: HashSet<&'static str> = {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .collect()
    };
    static ref EYE_COLORS: HashSet<&'static str> = {
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .into_iter()
            .collect()
    };
}

#[derive(Error, Debug, PartialEq)]
enum ParseError {
    #[error("The tag {0} is invalid.")]
    InvalidTagFormat(String),
}

#[derive(Debug, PartialEq)]
struct Tag<'a> {
    name: &'a str,
    value: &'a str,
}

impl<'a> Tag<'a> {
    fn new(name: &'a str, value: &'a str) -> Self {
        Tag { name, value }
    }

    fn parse(s: &'a str) -> Result<Self, ParseError> {
        let mut split = s.split(':');
        let name = split
            .next()
            .ok_or_else(|| ParseError::InvalidTagFormat(s.to_string()))?;
        let value = split
            .next()
            .ok_or_else(|| ParseError::InvalidTagFormat(s.to_string()))?;
        if let Some(_) = split.next() {
            Err(ParseError::InvalidTagFormat(s.to_string()))
        } else {
            Ok(Tag::new(name, value))
        }
    }

    fn is_valid_part1(&self) -> bool {
        VALID_FIELDS.contains(self.name)
    }

    fn is_valid_part2(&self) -> bool {
        let v = self.value;
        match self.name {
            "byr" => v.len() == 4 && v.parse::<u32>().map_or(false, |v| 1920 <= v && v <= 2002),
            "iyr" => v.len() == 4 && v.parse::<u32>().map_or(false, |v| 2010 <= v && v <= 2020),
            "eyr" => v.len() == 4 && v.parse::<u32>().map_or(false, |v| 2020 <= v && v <= 2030),
            "hgt" => self.validate_hgr(),
            "hcl" => {
                v.len() == 7
                    && v.chars().nth(0).unwrap() == '#'
                    && v[1..].chars().all(|c| c.is_ascii_hexdigit())
            }
            "ecl" => EYE_COLORS.contains(v),
            "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
            "cid" => true,
            _ => false,
        }
    }

    fn validate_hgr(&self) -> bool {
        let v = self.value;
        if v.ends_with("cm") && v.len() == 5 {
            v[0..3]
                .parse::<u32>()
                .map_or(false, |v| 150 <= v && v <= 193)
        } else if v.ends_with("in") && v.len() == 4 {
            v[0..2].parse::<u32>().map_or(false, |v| 59 <= v && v <= 76)
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq)]
struct Passport<'a> {
    tags: Vec<Tag<'a>>,
}

impl<'a> Passport<'a> {
    fn new(tags: Vec<Tag<'a>>) -> Self {
        Passport { tags }
    }

    fn parse(s: &'a str) -> Result<Self, ParseError> {
        let tags: Result<Vec<_>, _> = s
            .split(&[' ', '\n'][..])
            .filter(|x| x.len() > 0)
            .map(Tag::parse)
            .collect();
        tags.map(Passport::new)
    }

    fn parse_all(s: &'a str) -> Result<Vec<Self>, ParseError> {
        s.split("\n\n")
            .filter(|x| x.len() > 0)
            .map(Passport::parse)
            .collect()
    }

    fn is_valid_part1(&self) -> bool {
        let all_valid = self.tags.iter().all(|x| x.is_valid_part1());
        let all_mandatory = MANDATORY_FIELDS
            .iter()
            .all(|m| self.tags.iter().any(|x| &x.name == m));
        let all_unique: HashSet<&str> = self.tags.iter().map(|x| x.name).collect();
        all_valid && all_mandatory && (all_unique.len() == self.tags.len())
    }

    fn is_valid_part2(&self) -> bool {
        let all_valid = self.tags.iter().all(|x| x.is_valid_part2());
        let all_mandatory = MANDATORY_FIELDS
            .iter()
            .all(|m| self.tags.iter().any(|x| &x.name == m));
        let all_unique: HashSet<&str> = self.tags.iter().map(|x| x.name).collect();
        all_valid && all_mandatory && (all_unique.len() == self.tags.len())
    }
}

fn part1() {
    let data = std::fs::read_to_string("./inputs/day04.txt").unwrap();
    let valid = Passport::parse_all(&data)
        .unwrap()
        .iter()
        .filter(|x| x.is_valid_part1())
        .count();
    println!("Part 1: {}", valid);
}

fn part2() {
    let data = std::fs::read_to_string("./inputs/day04.txt").unwrap();
    let valid = Passport::parse_all(&data)
        .unwrap()
        .iter()
        .filter(|x| x.is_valid_part2())
        .count();
    println!("Part 2: {}", valid);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_parse() {
        assert_eq!(Ok(Tag::new("1", "1")), Tag::parse("1:1"));
        assert_eq!(Ok(Tag::new("1123  ", "  1aa")), Tag::parse("1123  :  1aa"));

        assert_eq!(
            Err(ParseError::InvalidTagFormat("1".to_string())),
            Tag::parse("1")
        );
        assert_eq!(
            Err(ParseError::InvalidTagFormat("1:1:1".to_string())),
            Tag::parse("1:1:1")
        );
    }

    #[test]
    fn tag_is_valid_part2() {
        assert_eq!(true, Tag::new("byr", "2002").is_valid_part2());
        assert_eq!(false, Tag::new("byr", "20002").is_valid_part2());
        assert_eq!(false, Tag::new("byr", "2003").is_valid_part2());

        assert_eq!(true, Tag::new("iyr", "2010").is_valid_part2());
        assert_eq!(false, Tag::new("iyr", "20100").is_valid_part2());
        assert_eq!(false, Tag::new("iyr", "2021").is_valid_part2());

        assert_eq!(true, Tag::new("hgt", "150cm").is_valid_part2());
        assert_eq!(true, Tag::new("hgt", "59in").is_valid_part2());
        assert_eq!(false, Tag::new("hgt", "200cm").is_valid_part2());
        assert_eq!(false, Tag::new("hgt", "10in").is_valid_part2());
        assert_eq!(false, Tag::new("hgt", "10bel").is_valid_part2());

        assert_eq!(true, Tag::new("hcl", "#123abc").is_valid_part2());
        assert_eq!(true, Tag::new("hcl", "#ffffff").is_valid_part2());
        assert_eq!(false, Tag::new("hcl", "123123").is_valid_part2());
        assert_eq!(false, Tag::new("hcl", "#12").is_valid_part2());
        assert_eq!(false, Tag::new("hcl", "2").is_valid_part2());

        assert_eq!(true, Tag::new("ecl", "brn").is_valid_part2());
        assert_eq!(false, Tag::new("ecl", "wat").is_valid_part2());

        assert_eq!(true, Tag::new("pid", "000000001").is_valid_part2());
        assert_eq!(false, Tag::new("pid", "0123456789").is_valid_part2());
    }

    #[test]
    fn passport_parse() {
        assert_eq!(
            Ok(Passport::new(vec![Tag::new("1", "1"), Tag::new("2", "a")])),
            Passport::parse("1:1 2:a")
        );

        assert_eq!(
            Ok(Passport::new(vec![Tag::new("1", "1"), Tag::new("2", "a")])),
            Passport::parse("1:1\n2:a")
        );

        assert_eq!(
            Ok(Passport::new(vec![Tag::new("1", "1"), Tag::new("2", "a")])),
            Passport::parse("1:1  2:a")
        );

        assert_eq!(
            Ok(Passport::new(vec![Tag::new("1", "1"), Tag::new("2", "a")])),
            Passport::parse("  1:1  2:a  ")
        );

        assert_eq!(
            Err(ParseError::InvalidTagFormat("1:2:3".to_string())),
            Passport::parse("1:2 1:2:3")
        );
    }

    #[test]
    fn passport_parse_all() {
        assert_eq!(Passport::parse_all("1:1 2:2").unwrap().len(), 1);
        assert_eq!(Passport::parse_all("1:1 2:2\n").unwrap().len(), 1);
        assert_eq!(Passport::parse_all("1:1\n2:2\n").unwrap().len(), 1);
        assert_eq!(Passport::parse_all("1:1\n\n2:2\n").unwrap().len(), 2);
    }

    #[test]
    fn passport_is_valid_part1() {
        let p = Passport::parse(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
        )
        .unwrap();
        assert_eq!(true, p.is_valid_part1());

        let p =
            Passport::parse("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929")
                .unwrap();
        assert_eq!(false, p.is_valid_part1());

        let p = Passport::parse(
            "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm",
        )
        .unwrap();
        assert_eq!(true, p.is_valid_part1());

        let p = Passport::parse("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in")
            .unwrap();
        assert_eq!(false, p.is_valid_part1());
    }

    #[test]
    fn passport_is_valid_part2() {
        let p = Passport::parse(
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        )
        .unwrap();
        assert_eq!(false, p.is_valid_part2());

        let p = Passport::parse(
            "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
        )
        .unwrap();
        assert_eq!(false, p.is_valid_part2());

        let p = Passport::parse(
            "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        )
        .unwrap();
        assert_eq!(false, p.is_valid_part2());

        let p = Passport::parse(
            "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
        )
        .unwrap();
        assert_eq!(false, p.is_valid_part2());

        let p = Passport::parse(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
        )
        .unwrap();
        assert_eq!(true, p.is_valid_part2());

        let p = Passport::parse(
            "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        )
        .unwrap();
        assert_eq!(true, p.is_valid_part2());

        let p = Passport::parse(
            "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
        )
        .unwrap();
        assert_eq!(true, p.is_valid_part2());

        let p = Passport::parse(
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        )
        .unwrap();
        assert_eq!(true, p.is_valid_part2());
    }
}
