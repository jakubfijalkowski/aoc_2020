use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq)]
struct Range(usize, usize);

impl Range {
    fn position(&self) -> Option<usize> {
        if self.0 == self.1 {
            Some(self.0)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Half {
    Lower,
    Upper,
}

impl Half {
    fn parse_row(c: char) -> Option<Self> {
        match c {
            'F' => Some(Self::Lower),
            'B' => Some(Self::Upper),
            _ => None,
        }
    }

    fn parse_column(c: char) -> Option<Self> {
        match c {
            'L' => Some(Self::Lower),
            'R' => Some(Self::Upper),
            _ => None,
        }
    }

    fn apply(&self, r: Range) -> Range {
        let Range(l, u) = r;
        let size = u - l + 1;
        let half = size / 2;
        match self {
            Self::Lower => Range(l, u - half),
            Self::Upper => Range(l + half, u),
        }
    }
}

#[derive(Debug, PartialEq)]
struct BoardingPass(Vec<Half>, Vec<Half>);

impl BoardingPass {
    fn parse(s: &str) -> Option<Self> {
        if s.len() == 10 {
            let rows: Option<Vec<_>> = (&s[0..7]).chars().map(Half::parse_row).collect();
            let columns: Option<Vec<_>> = (&s[7..]).chars().map(Half::parse_column).collect();
            match (rows, columns) {
                (Some(rows), Some(columns)) => Some(BoardingPass(rows, columns)),
                _ => None,
            }
        } else {
            None
        }
    }

    fn get_seat(&self) -> Option<(usize, usize)> {
        let row = self
            .0
            .iter()
            .fold(Range(0, 127), |acc, x| x.apply(acc))
            .position();
        let column = self
            .1
            .iter()
            .fold(Range(0, 7), |acc, x| x.apply(acc))
            .position();

        match (row, column) {
            (Some(row), Some(column)) => Some((row, column)),
            _ => None,
        }
    }

    fn get_seat_id(&self) -> Option<usize> {
        match self.get_seat() {
            Some((r, c)) => Some(r * 8 + c),
            _ => None,
        }
    }
}

fn read_input() -> Vec<BoardingPass> {
    BufReader::new(fs::File::open("./inputs/day05.txt").unwrap())
        .lines()
        .map(|x| BoardingPass::parse(&x.unwrap()).unwrap())
        .collect()
}

fn part1() {
    let max_id = read_input().iter().map(|b| b.get_seat_id().unwrap()).max();
    println!("Part 1: {}", max_id.unwrap());
}

fn part2() {
    let mut seats: Vec<_> = read_input()
        .iter()
        .map(|b| b.get_seat_id().unwrap())
        .collect();
    seats.sort();

    let offset: Vec<_> = seats.clone().into_iter().skip(1).collect();
    let santa_seat = seats
        .into_iter()
        .zip(offset)
        .filter(|(s1, s2)| s1 + 2 == *s2 && s1 + 1 > 7 && s1 + 1 < 127 * 8)
        .map(|(s1, _)| s1 + 1)
        .next()
        .unwrap();

    println!("Part 2: {}", santa_seat);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_apply() {
        let r = Range(0, 127);

        let r = Half::Lower.apply(r);
        assert_eq!(Range(0, 63), r);

        let r = Half::Upper.apply(r);
        assert_eq!(Range(32, 63), r);

        let r = Half::Lower.apply(r);
        assert_eq!(Range(32, 47), r);

        let r = Half::Upper.apply(r);
        assert_eq!(Range(40, 47), r);

        let r = Half::Upper.apply(r);
        assert_eq!(Range(44, 47), r);

        let r = Half::Lower.apply(r);
        assert_eq!(Range(44, 45), r);

        let r = Half::Lower.apply(r);
        assert_eq!(Range(44, 44), r);

        assert_eq!(Some(44), r.position());
    }

    #[test]
    fn boardingpass_parse() {
        let l = Half::Lower;
        let u = Half::Upper;

        assert_eq!(
            Some(BoardingPass(vec![l, u, l, u, u, l, l], vec![u, l, u])),
            BoardingPass::parse("FBFBBFFRLR")
        );
    }

    #[test]
    fn boardingpass_seat() {
        assert_eq!(
            Some((44, 5)),
            BoardingPass::parse("FBFBBFFRLR").unwrap().get_seat()
        );
    }
}
