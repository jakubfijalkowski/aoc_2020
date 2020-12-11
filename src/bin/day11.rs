#![feature(generators, generator_trait)]

use std::marker::Unpin;
use std::ops::{Generator, GeneratorState};
use std::path::Path;
use std::pin::Pin;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position(isize, isize);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cell {
    Floor,
    EmptySeat,
    TakenSeat,
}

#[derive(Debug, Clone, PartialEq)]
struct GameOfLife {
    width: isize,
    height: isize,
    tiles: Vec<Cell>,
}

struct SeatMutation {
    can_change_to_empty: bool,
    can_change_to_taken: bool,
}

/// Normally, I would make `Rules` trait look like this:
/// ```
/// trait Rules {
///     fn can_change_to_empty(adjacent: impl Iterator<Item = Cell>) -> bool;
///     fn can_change_to_taken(adjacent: impl Iterator<Item = Cell>) -> bool;
/// }
/// ```
/// unfortunately, the `adjacent` iterator cannot be changed to an `IntoIterator` or even
/// `Copy`/`Clone` as it will be `GenIterator`-based and since generators can't be [copied] yet,
/// I can only iterate over it once. Hence the `fold` in implementations.
///
/// [copied]: https://github.com/rust-lang/rust/issues/57972
trait Rules {
    fn get_adjacent<'a>(
        game: &'a GameOfLife,
        at: Position,
    ) -> Box<dyn Generator<Yield = Cell, Return = ()> + Unpin + 'a>;
    fn can_mutate(adjacent: impl Iterator<Item = Cell>) -> SeatMutation;
}

struct Part1Rules;
struct Part2Rules;

impl Cell {
    fn parse(c: char) -> Option<Cell> {
        match c {
            '.' => Some(Self::Floor),
            'L' => Some(Self::EmptySeat),
            '#' => Some(Self::TakenSeat),
            _ => None,
        }
    }

    fn mutate<R: Rules, I: Iterator<Item = Cell>>(&self, adjacent: I) -> Self {
        let rules = R::can_mutate(adjacent);
        if *self == Self::EmptySeat && rules.can_change_to_taken {
            Self::TakenSeat
        } else if *self == Self::TakenSeat && rules.can_change_to_empty {
            Self::EmptySeat
        } else {
            *self
        }
    }
}

impl GameOfLife {
    fn parse(data: &str) -> Option<Self> {
        let width = data.find('\n')?;
        let data = data.replace("\n", "");
        let height = data.len() / width;
        let tiles: Option<Vec<_>> = data.chars().map(Cell::parse).collect();
        let tiles = tiles?;

        if tiles.len() == width * height {
            Some(Self {
                width: width as isize,
                height: height as isize,
                tiles,
            })
        } else {
            None
        }
    }

    fn read_file<P: AsRef<Path>>(p: P) -> Option<Self> {
        let data = std::fs::read_to_string(p).map_or_else(|_| None, Some)?;
        Self::parse(&data[..])
    }

    fn get_at(&self, pos: Position) -> Cell {
        let Position(x, y) = pos;
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            Cell::Floor
        } else {
            self.tiles[(y * self.width + x) as usize]
        }
    }

    fn iterate<R: Rules>(&mut self) -> bool {
        let mut new_tiles = self.tiles.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let adjacent = gather(R::get_adjacent(self, Position(x, y)));
                new_tiles[idx as usize] = self.get_at(Position(x, y)).mutate::<R, _>(adjacent);
            }
        }
        if self.tiles != new_tiles {
            self.tiles = new_tiles;
            true
        } else {
            false
        }
    }

    fn count_taken(&self) -> usize {
        self.tiles.iter().filter(|&x| *x == Cell::TakenSeat).count()
    }
}

impl Rules for Part1Rules {
    fn get_adjacent<'a>(
        game: &'a GameOfLife,
        at: Position,
    ) -> Box<dyn Generator<Yield = Cell, Return = ()> + Unpin + 'a> {
        Box::new(move || {
            let Position(x, y) = at;

            yield game.get_at(Position(x - 1, y - 1));
            yield game.get_at(Position(x + 0, y - 1));
            yield game.get_at(Position(x + 1, y - 1));
            yield game.get_at(Position(x - 1, y + 0));
            yield game.get_at(Position(x + 1, y + 0));
            yield game.get_at(Position(x - 1, y + 1));
            yield game.get_at(Position(x + 0, y + 1));
            yield game.get_at(Position(x + 1, y + 1));
        })
    }

    fn can_mutate(adjacent: impl Iterator<Item = Cell>) -> SeatMutation {
        let (x, a) = adjacent.fold((0, true), |(x, a), c| {
            (
                if c == Cell::TakenSeat { x + 1 } else { x },
                a && c != Cell::TakenSeat,
            )
        });
        SeatMutation {
            can_change_to_empty: x >= 4,
            can_change_to_taken: a,
        }
    }
}

impl Rules for Part2Rules {
    fn get_adjacent<'a>(
        game: &'a GameOfLife,
        at: Position,
    ) -> Box<dyn Generator<Yield = Cell, Return = ()> + Unpin + 'a> {
        Box::new(move || {
            yield Self::get_at_direction(game, at, -1, -1);
            yield Self::get_at_direction(game, at, -1, 0);
            yield Self::get_at_direction(game, at, -1, 1);
            yield Self::get_at_direction(game, at, 0, -1);
            yield Self::get_at_direction(game, at, 0, 1);
            yield Self::get_at_direction(game, at, 1, -1);
            yield Self::get_at_direction(game, at, 1, 0);
            yield Self::get_at_direction(game, at, 1, 1);
        })
    }

    fn can_mutate(adjacent: impl Iterator<Item = Cell>) -> SeatMutation {
        let (x, a) = adjacent.fold((0, true), |(x, a), c| {
            (
                if c == Cell::TakenSeat { x + 1 } else { x },
                a && c != Cell::TakenSeat,
            )
        });
        SeatMutation {
            can_change_to_empty: x >= 5,
            can_change_to_taken: a,
        }
    }
}

impl Part2Rules {
    fn get_at_direction(game: &GameOfLife, at: Position, xd: isize, yd: isize) -> Cell {
        let Position(mut x, mut y) = at;
        x = x + xd;
        y = y + yd;

        while 0 <= x && x < game.width && 0 <= y && y < game.height {
            let s = game.get_at(Position(x, y));
            if s != Cell::Floor {
                return s;
            }

            x = x + xd;
            y = y + yd;
        }
        Cell::Floor
    }
}

struct GenIterator<'a, Yield, Return> {
    generator: Box<dyn Generator<Yield = Yield, Return = Return> + Unpin + 'a>,
}

impl<'a, Yield, Return> Iterator for GenIterator<'a, Yield, Return> {
    type Item = Yield;

    fn next(&mut self) -> Option<Self::Item> {
        let mut generator = self.generator.as_mut();
        match Pin::new(&mut generator).resume(()) {
            GeneratorState::Yielded(c) => Some(c),
            GeneratorState::Complete(_) => None,
        }
    }
}

fn gather<'a, Yield: 'a, Return: 'a>(
    generator: Box<dyn Generator<Yield = Yield, Return = Return> + Unpin + 'a>,
) -> impl Iterator<Item = Yield> + 'a {
    GenIterator { generator }
}

fn part1() {
    let mut map = GameOfLife::read_file("./inputs/day11.txt").unwrap();
    while map.iterate::<Part1Rules>() {}
    println!("Part 1: {}", map.count_taken());
}

fn part2() {
    let mut map = GameOfLife::read_file("./inputs/day11.txt").unwrap();
    while map.iterate::<Part2Rules>() {}
    println!("Part 2: {}", map.count_taken());
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
        let m = GameOfLife::parse("LL\n..");
        assert_eq!(
            m,
            Some(GameOfLife {
                width: 2,
                height: 2,
                tiles: vec![Cell::EmptySeat, Cell::EmptySeat, Cell::Floor, Cell::Floor]
            })
        );
    }

    #[test]
    fn iterate_part1_example() {
        let mut a = GameOfLife::parse("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL").unwrap();
        let b = GameOfLife::parse("#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##").unwrap();

        assert_eq!(true, a.iterate::<Part1Rules>());
        assert_eq!(a, b);
    }

    #[test]
    fn iterate_part2_example() {
        let mut a = GameOfLife::parse("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL").unwrap();
        let b = GameOfLife::parse("#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##").unwrap();
        let c = GameOfLife::parse("#.LL.LL.L#\n#LLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLL#\n#.LLLLLL.L\n#.LLLLL.L#").unwrap();

        assert_eq!(true, a.iterate::<Part2Rules>());
        assert_eq!(a, b);

        assert_eq!(true, a.iterate::<Part2Rules>());
        assert_eq!(a, c);
    }

    #[test]
    fn gather_generator() {
        let g = Box::new(|| {
            yield Cell::EmptySeat;
            yield Cell::TakenSeat;
            yield Cell::Floor;
        });
        let all: Vec<_> = gather(g).collect();
        assert_eq!(vec![Cell::EmptySeat, Cell::TakenSeat, Cell::Floor], all);
    }
}
