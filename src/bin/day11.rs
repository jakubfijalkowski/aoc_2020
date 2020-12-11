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

impl Cell {
    fn parse(c: char) -> Option<Cell> {
        match c {
            '.' => Some(Self::Floor),
            'L' => Some(Self::EmptySeat),
            '#' => Some(Self::TakenSeat),
            _ => None,
        }
    }

    fn mutate<R: Ruling>(&self, adjacent: &[Self]) -> Self {
        debug_assert!(adjacent.len() == 8);
        if *self == Self::EmptySeat && R::should_change_to_taken(adjacent) {
            Self::TakenSeat
        } else if *self == Self::TakenSeat && R::should_change_to_empty(adjacent) {
            Self::EmptySeat
        } else {
            *self
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct GameOfLife {
    width: isize,
    height: isize,
    tiles: Vec<Cell>,
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

    fn iterate<R: Ruling>(&mut self) -> bool {
        let mut new_tiles = self.tiles.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let adjacent = gather(R::get_adjacent(self, Position(x, y)));
                new_tiles[idx as usize] = self.get_at(Position(x, y)).mutate::<R>(&adjacent);
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

trait Ruling {
    fn get_adjacent<'a>(
        game: &'a GameOfLife,
        at: Position,
    ) -> Box<dyn Generator<Yield = Cell, Return = ()> + Unpin + 'a>;
    fn should_change_to_empty(adjacent: &[Cell]) -> bool;
    fn should_change_to_taken(adjacent: &[Cell]) -> bool;
}

fn gather<'a>(
    mut generator: Box<dyn Generator<Yield = Cell, Return = ()> + Unpin + 'a>,
) -> Vec<Cell> {
    let mut result = Vec::new();
    let mut generator = generator.as_mut();
    while let GeneratorState::Yielded(c) = Pin::new(&mut generator).resume(()) {
        result.push(c);
    }
    result
}

struct Part1;
struct Part2;

impl Ruling for Part1 {
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

    fn should_change_to_empty(adjacent: &[Cell]) -> bool {
        adjacent.iter().filter(|&x| *x == Cell::TakenSeat).count() >= 4
    }

    fn should_change_to_taken(adjacent: &[Cell]) -> bool {
        adjacent.iter().all(|&x| x != Cell::TakenSeat)
    }
}

impl Ruling for Part2 {
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

    fn should_change_to_empty(adjacent: &[Cell]) -> bool {
        adjacent.iter().filter(|&x| *x == Cell::TakenSeat).count() >= 5
    }

    fn should_change_to_taken(adjacent: &[Cell]) -> bool {
        adjacent.iter().all(|&x| x != Cell::TakenSeat)
    }
}

impl Part2 {
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

fn part1() {
    let mut map = GameOfLife::read_file("./inputs/day11.txt").unwrap();
    while map.iterate::<Part1>() {}
    println!("Part 1: {}", map.count_taken());
}

fn part2() {
    let mut map = GameOfLife::read_file("./inputs/day11.txt").unwrap();
    while map.iterate::<Part2>() {}
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

        assert_eq!(true, a.iterate::<Part1>());
        assert_eq!(a, b);
    }

    #[test]
    fn iterate_part2_example() {
        let mut a = GameOfLife::parse("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL").unwrap();
        let b = GameOfLife::parse("#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##").unwrap();
        let c = GameOfLife::parse("#.LL.LL.L#\n#LLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLL#\n#.LLLLLL.L\n#.LLLLL.L#").unwrap();

        assert_eq!(true, a.iterate::<Part2>());
        assert_eq!(a, b);

        assert_eq!(true, a.iterate::<Part2>());
        assert_eq!(a, c);
    }

    #[test]
    fn gather_generator() {
        let g = Box::new(|| {
            yield Cell::EmptySeat;
            yield Cell::TakenSeat;
            yield Cell::Floor;
        });
        let all = gather(g);
        assert_eq!(vec![Cell::EmptySeat, Cell::TakenSeat, Cell::Floor], all);
    }
}
