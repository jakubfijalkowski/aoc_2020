use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    North(i64),
    South(i64),
    West(i64),
    East(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position(i64, i64, Direction);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ship {
    ship: Position,
    waypoint: Position,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::West => Self::North,
            Self::South => Self::West,
            Self::East => Self::South,
        }
    }

    fn to_instruction(&self, off: i64) -> Instruction {
        match self {
            Self::North => Instruction::North(off),
            Self::West => Instruction::West(off),
            Self::South => Instruction::South(off),
            Self::East => Instruction::East(off),
        }
    }
}

impl Instruction {
    fn parse(s: &str) -> Option<Self> {
        let off: i64 = s[1..].parse().map_or(None, Some)?;
        match s.chars().next()? {
            'L' => Some(Instruction::Left(off)),
            'R' => Some(Instruction::Right(off)),
            'F' => Some(Instruction::Forward(off)),
            'N' => Some(Instruction::North(off)),
            'W' => Some(Instruction::West(off)),
            'S' => Some(Instruction::South(off)),
            'E' => Some(Instruction::East(off)),
            _ => None,
        }
    }
}

impl Position {
    fn apply(&self, i: &Instruction) -> Self {
        let Position(x, y, d) = *self;
        match i {
            Instruction::North(off) => Position(x, y + off, d),
            Instruction::South(off) => Position(x, y - off, d),
            Instruction::West(off) => Position(x - off, y, d),
            Instruction::East(off) => Position(x + off, y, d),
            Instruction::Left(90) => Position(x, y, d.left()),
            Instruction::Left(180) => Position(x, y, d.left().left()),
            Instruction::Left(270) => Position(x, y, d.left().left().left()),
            Instruction::Right(90) => Position(x, y, d.right()),
            Instruction::Right(180) => Position(x, y, d.right().right()),
            Instruction::Right(270) => Position(x, y, d.right().right().right()),
            Instruction::Left(0) | Instruction::Right(0) => *self,
            Instruction::Forward(off) => self.apply(&d.to_instruction(*off)),
            _ => {
                panic!("unknown rotation")
            }
        }
    }

    fn left_by(&self, d: i64) -> Self {
        match d {
            0 => *self,
            90 => self.left(),
            180 => self.left().left(),
            270 => self.left().left().left(),
            _ => panic!("unknown rotation"),
        }
    }

    fn right_by(&self, d: i64) -> Self {
        match d {
            0 => *self,
            90 => self.right(),
            180 => self.right().right(),
            270 => self.right().right().right(),
            _ => panic!("unknown rotation"),
        }
    }

    fn left(&self) -> Self {
        let Position(x, y, d) = *self;
        Position(-y, x, d)
    }

    fn right(&self) -> Self {
        let Position(x, y, d) = *self;
        Position(y, -x, d)
    }
}

impl Ship {
    fn apply(&self, i: &Instruction) -> Self {
        let Position(x, y, d) = self.ship;
        match i {
            Instruction::North(_)
            | Instruction::South(_)
            | Instruction::West(_)
            | Instruction::East(_) => Ship {
                waypoint: self.waypoint.apply(i),
                ship: self.ship,
            },
            Instruction::Left(r) => Ship {
                waypoint: self.waypoint.left_by(*r),
                ship: self.ship,
            },
            Instruction::Right(r) => Ship {
                waypoint: self.waypoint.right_by(*r),
                ship: self.ship,
            },
            Instruction::Forward(off) => Ship {
                waypoint: self.waypoint,
                ship: Position(x + self.waypoint.0 * off, y + self.waypoint.1 * off, d),
            },
        }
    }
}

fn read_input() -> Option<Vec<Instruction>> {
    BufReader::new(fs::File::open("./inputs/day12.txt").unwrap())
        .lines()
        .map(|x| {
            x.map(|x| Instruction::parse(&x[..]))
                .map_or(None, Some)
                .flatten()
        })
        .collect()
}

fn part1() {
    let pos = Position(0, 0, Direction::East);
    let instr = read_input().unwrap();
    let pos = instr.iter().fold(pos, |p, i| p.apply(i));
    println!("Part 1: {}", pos.0.abs() + pos.1.abs());
}

fn part2() {
    let ship = Ship {
        waypoint: Position(10, 1, Direction::East),
        ship: Position(0, 0, Direction::East),
    };
    let instr = read_input().unwrap();
    let ship = instr.iter().fold(ship, |p, i| p.apply(i));
    println!("Part 2: {}", ship.ship.0.abs() + ship.ship.1.abs());
}

fn main() {
    part1();
    part2();
}
