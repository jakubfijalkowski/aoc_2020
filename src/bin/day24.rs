#![feature(map_into_keys_values)]
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Pos(isize, isize);

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Color {
    Black,
    White,
}

impl Pos {
    fn do_move(&self, dir: Direction) -> Pos {
        if self.1 & 1 == 1 {
            match dir {
                Direction::East => Pos(self.0 + 1, self.1),
                Direction::SouthEast => Pos(self.0 + 1, self.1 + 1),
                Direction::SouthWest => Pos(self.0, self.1 + 1),
                Direction::West => Pos(self.0 - 1, self.1),
                Direction::NorthWest => Pos(self.0, self.1 - 1),
                Direction::NorthEast => Pos(self.0 + 1, self.1 - 1),
            }
        } else {
            match dir {
                Direction::East => Pos(self.0 + 1, self.1),
                Direction::SouthEast => Pos(self.0, self.1 + 1),
                Direction::SouthWest => Pos(self.0 - 1, self.1 + 1),
                Direction::West => Pos(self.0 - 1, self.1),
                Direction::NorthWest => Pos(self.0 - 1, self.1 - 1),
                Direction::NorthEast => Pos(self.0, self.1 - 1),
            }
        }
    }
}

impl Direction {
    fn parse(mut s: &str) -> Vec<Direction> {
        let mut res = Vec::with_capacity(s.len());

        while !s.is_empty() {
            let (d, next) = Self::parse_single(s);
            res.push(d);
            s = next;
        }

        res
    }

    fn parse_single(s: &str) -> (Self, &str) {
        let mut chars = s.chars();
        match chars.next().unwrap() {
            'e' => (Self::East, &s[1..]),
            'w' => (Self::West, &s[1..]),
            's' => match chars.next().unwrap() {
                'e' => (Self::SouthEast, &s[2..]),
                'w' => (Self::SouthWest, &s[2..]),
                _ => panic!("unknown direction"),
            },
            'n' => match chars.next().unwrap() {
                'e' => (Self::NorthEast, &s[2..]),
                'w' => (Self::NorthWest, &s[2..]),
                _ => panic!("unknown direction"),
            },
            _ => panic!("unknown direction"),
        }
    }

    fn all() -> [Direction; 6] {
        [
            Self::East,
            Self::SouthEast,
            Self::SouthWest,
            Self::West,
            Self::NorthWest,
            Self::NorthEast,
        ]
    }
}

impl Color {
    fn opposite(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

fn run(mut map: HashMap<Pos, Color>, trail: &[Direction]) -> HashMap<Pos, Color> {
    let curr_pos = trail.iter().fold(Pos(0, 0), |p, d| p.do_move(*d));

    map.entry(curr_pos)
        .and_modify(|e| {
            *e = e.opposite();
        })
        .or_insert(Color::Black);

    map
}

fn sim_day(map: HashMap<Pos, Color>) -> HashMap<Pos, Color> {
    let mut next_map = HashMap::new();
    let to_check =
        map.iter()
            .filter(|(_, c)| **c == Color::Black)
            .fold(Vec::new(), |mut v, (&p, _)| {
                v.push(p);
                Direction::all().iter().for_each(|&x| v.push(p.do_move(x)));
                v
            });
    for pos in &to_check {
        let c = *map.get(pos).unwrap_or(&Color::White);
        let blacks: u32 = Direction::all()
            .iter()
            .map(|&x| {
                map.get(&pos.do_move(x))
                    .map_or(0, |&x| if x == Color::Black { 1 } else { 0 })
            })
            .sum();

        let pos = *pos;
        if c == Color::Black && (blacks == 0 || blacks > 2) {
            next_map.insert(pos, Color::White);
        } else if c == Color::White && blacks == 2 {
            next_map.insert(pos, Color::Black);
        } else {
            next_map.insert(pos, c);
        }
    }

    next_map
}

fn part1() {
    let data = include_str!("../../inputs/day24.txt");
    let dirs: Vec<_> = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(Direction::parse)
        .collect();
    let map = dirs.into_iter().fold(HashMap::new(), |map, r| run(map, &r));

    let blacks = map.into_values().filter(|&x| x == Color::Black).count();
    println!("Part 1: {}", blacks);
}

fn part2() {
    let data = include_str!("../../inputs/day24.txt");
    let dirs: Vec<_> = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(Direction::parse)
        .collect();
    let map = dirs.into_iter().fold(HashMap::new(), |map, r| run(map, &r));
    let map = (0..100).fold(map, |m, _| sim_day(m));
    let blacks = map.into_values().filter(|&x| x == Color::Black).count();
    println!("Part 2: {}", blacks);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves() {
        assert_eq!(Pos(2, 2).do_move(Direction::East), Pos(3, 2));
        assert_eq!(Pos(2, 2).do_move(Direction::SouthEast), Pos(2, 3));
        assert_eq!(Pos(2, 2).do_move(Direction::SouthWest), Pos(1, 3));
        assert_eq!(Pos(2, 2).do_move(Direction::West), Pos(1, 2));
        assert_eq!(Pos(2, 2).do_move(Direction::NorthWest), Pos(1, 1));
        assert_eq!(Pos(2, 2).do_move(Direction::NorthEast), Pos(2, 1));

        assert_eq!(Pos(3, 3).do_move(Direction::East), Pos(4, 3));
        assert_eq!(Pos(3, 3).do_move(Direction::SouthEast), Pos(4, 4));
        assert_eq!(Pos(3, 3).do_move(Direction::SouthWest), Pos(3, 4));
        assert_eq!(Pos(3, 3).do_move(Direction::West), Pos(2, 3));
        assert_eq!(Pos(3, 3).do_move(Direction::NorthWest), Pos(3, 2));
        assert_eq!(Pos(3, 3).do_move(Direction::NorthEast), Pos(4, 2));
    }
}
