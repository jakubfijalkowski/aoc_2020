use std::convert::{TryFrom, TryInto};
use thiserror::Error;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Slope(usize, usize);
#[derive(Copy, Clone, PartialEq, Debug)]
struct Position(usize, usize);

#[derive(Copy, Clone, PartialEq)]
enum Field {
    Empty,
    Tree,
}

impl Field {
    fn parse(value: char) -> Option<Self> {
        match value {
            '.' => Some(Self::Empty),
            '#' => Some(Self::Tree),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("."),
            Self::Tree => f.write_str("#"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    width: usize,
    height: usize,
    data: Vec<Field>,
}

#[derive(Error, Debug, PartialEq)]
enum ParseError {
    #[error("Unknown char {c} at line {line}, position {pos}")]
    UnknownCharAt { c: char, line: usize, pos: usize },
    #[error("The shape of line {0} is invalid")]
    InvalidShape(usize),
}

impl TryFrom<&str> for Map {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut output = Vec::with_capacity(value.len());
        let width = value.find('\n').unwrap_or_else(|| value.len());

        let mut pos = 0;
        let mut line = 0;
        for c in value.chars() {
            if (pos == width && c != '\n') || (pos < width && c == '\n') {
                return Err(ParseError::InvalidShape(line));
            } else if c == '\n' {
                pos = 0;
                line = line + 1;
            } else {
                let f = Field::parse(c).ok_or(ParseError::UnknownCharAt { c, line, pos })?;
                output.push(f);

                pos = pos + 1;
            }
        }

        if pos != 0 && pos != width {
            Err(ParseError::InvalidShape(line))
        } else {
            Ok(Map {
                width,
                height: line + if pos == 0 { 0 } else { 1 },
                data: output,
            })
        }
    }
}

impl TryFrom<String> for Map {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        (&value[..]).try_into()
    }
}

impl Map {
    fn proceed(&self, pos: Position, slope: Slope) -> Position {
        let Position(x, y) = pos;
        let Slope(xm, ym) = slope;
        Position((x + xm) % self.width, y + ym)
    }

    fn test(&self, pos: Position) -> Option<Field> {
        if pos.0 < self.width && pos.1 < self.height {
            let idx = pos.1 * self.width + pos.0;
            Some(self.data[idx])
        } else {
            None
        }
    }

    fn play(&self, slope: Slope) -> usize {
        let mut pos = Position(0, 0);
        let mut trees = 0;
        loop {
            match self.test(pos) {
                Some(Field::Tree) => trees = trees + 1,
                Some(Field::Empty) => {}
                None => break,
            }
            pos = self.proceed(pos, slope);
        }
        trees
    }
}

fn part1() {
    let data = std::fs::read_to_string("./inputs/day03.txt").unwrap();
    let map = Map::try_from(data).unwrap();

    let slope = Slope(3, 1);
    let trees = map.play(slope);
    println!("Part 1: {}", trees);
}

fn part2() {
    let data = std::fs::read_to_string("./inputs/day03.txt").unwrap();
    let map = Map::try_from(data).unwrap();

    let t1 = map.play(Slope(1, 1));
    let t2 = map.play(Slope(3, 1));
    let t3 = map.play(Slope(5, 1));
    let t4 = map.play(Slope(7, 1));
    let t5 = map.play(Slope(1, 2));
    println!("Part 2: {}", t1 * t2 * t3 * t4 * t5);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_parse() {
        assert_eq!(Field::parse('.'), Some(Field::Empty));
        assert_eq!(Field::parse('#'), Some(Field::Tree));
        assert_eq!(Field::parse('!'), None);
    }

    #[test]
    fn map_parse_success() {
        let raw_map = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let map = Map::try_from(raw_map).unwrap();
        assert_eq!(11, map.width);
        assert_eq!(11, map.height);
    }

    #[test]
    fn map_parse_success_long() {
        let raw_map = ".\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.";
        let map = Map::try_from(raw_map).unwrap();
        assert_eq!(1, map.width);
        assert_eq!(21, map.height);

        let raw_map = ".\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n";
        let map = Map::try_from(raw_map).unwrap();
        assert_eq!(1, map.width);
        assert_eq!(21, map.height);
    }

    #[test]
    fn map_parse_success_short() {
        let raw_map = "..##.......";
        let map = Map::try_from(raw_map).unwrap();
        assert_eq!(11, map.width);
        assert_eq!(1, map.height);
    }

    #[test]
    fn map_parse_failure_shape_second() {
        let raw_map = r#"..##.......
#...#...#.
..........."#;
        let map = Map::try_from(raw_map);

        assert_eq!(map, Err(ParseError::InvalidShape(1)));
    }

    #[test]
    fn map_parse_failure_shape_last() {
        let raw_map = r#"..##.......
#...#...#."#;
        let map = Map::try_from(raw_map);

        assert_eq!(map, Err(ParseError::InvalidShape(1)));
    }

    #[test]
    fn map_parse_failure_char() {
        let raw_map = r#"..##.......
#...#...#.!"#;
        let map = Map::try_from(raw_map);

        assert_eq!(
            map,
            Err(ParseError::UnknownCharAt {
                c: '!',
                line: 1,
                pos: 10
            })
        );
    }

    #[test]
    fn map_move() {
        let raw_map = "...\n...\n...\n...";
        let map = Map::try_from(raw_map).unwrap();

        let pos = Position(0, 0);
        let slope = Slope(1, 1);

        let pos = map.proceed(pos, slope);
        assert_eq!(Position(1, 1), pos);
        let pos = map.proceed(pos, slope);
        assert_eq!(Position(2, 2), pos);
        let pos = map.proceed(pos, slope);
        assert_eq!(Position(0, 3), pos);
        let pos = map.proceed(pos, slope);
        assert_eq!(Position(1, 4), pos);
    }

    #[test]
    fn map_test() {
        let raw_map = ".#.\n..#";
        let map = Map::try_from(raw_map).unwrap();

        assert_eq!(Some(Field::Empty), map.test(Position(0, 0)));
        assert_eq!(Some(Field::Tree), map.test(Position(1, 0)));
        assert_eq!(Some(Field::Empty), map.test(Position(2, 0)));
        assert_eq!(Some(Field::Empty), map.test(Position(0, 1)));
        assert_eq!(Some(Field::Empty), map.test(Position(1, 1)));
        assert_eq!(Some(Field::Tree), map.test(Position(2, 1)));

        assert_eq!(None, map.test(Position(0, 2)));
        assert_eq!(None, map.test(Position(3, 0)));
    }

    #[test]
    fn map_play() {
        let raw_map = ".#\n#.\n#.";
        let map = Map::try_from(raw_map).unwrap();

        assert_eq!(2, map.play(Slope(0, 1)));
        assert_eq!(1, map.play(Slope(1, 1)));
        assert_eq!(1, map.play(Slope(1, 1)));
        assert_eq!(0, map.play(Slope(1, 2)));
    }

    #[test]
    fn map_play_example() {
        let raw_map = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let map = Map::try_from(raw_map).unwrap();

        assert_eq!(2, map.play(Slope(1, 1)));
        assert_eq!(7, map.play(Slope(3, 1)));
        assert_eq!(3, map.play(Slope(5, 1)));
        assert_eq!(4, map.play(Slope(7, 1)));
        assert_eq!(2, map.play(Slope(1, 2)));
    }
}
