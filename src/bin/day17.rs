#![feature(min_const_generics)]
#![feature(iterator_fold_self)]
#![feature(test)]

extern crate test;

use std::{collections::HashSet, fmt::Debug, ops::Sub};
use std::{hash::Hash, ops::Add};

trait Position: Eq + Copy + Hash + Add<isize, Output = Self> + Sub<isize, Output = Self> {
    type IntoIter: Iterator<Item = Self>;

    fn fill(b: isize) -> Self;
    fn from_2d(x: isize, y: isize) -> Self;

    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;
    fn iterate_to(&self, to: &Self) -> Self::IntoIter;
    fn get_neighbors(&self) -> Self::IntoIter;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct PosND<const N: usize>([isize; N]);

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct PosIter<const N: usize> {
    from: PosND<N>,
    to: PosND<N>,
    current: PosND<N>,
}

impl<const N: usize> Add<isize> for PosND<N> {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        let mut copy = self.clone();
        for i in 0..N {
            copy.0[i] += rhs;
        }
        copy
    }
}

impl<const N: usize> Sub<isize> for PosND<N> {
    type Output = Self;

    fn sub(self, rhs: isize) -> Self::Output {
        let mut copy = self.clone();
        for i in 0..N {
            copy.0[i] -= rhs;
        }
        copy
    }
}

impl<const N: usize> Position for PosND<N> {
    type IntoIter = PosIter<N>;

    fn fill(v: isize) -> Self {
        Self([v; N])
    }

    fn from_2d(x: isize, y: isize) -> Self {
        assert!(N >= 2);
        let mut result = [0; N];
        result[0] = x;
        result[1] = y;
        Self(result)
    }

    fn min(&self, other: &Self) -> Self {
        let mut result = Self([0; N]);
        for i in 0..N {
            result.0[i] = self.0[i].min(other.0[i]);
        }
        result
    }

    fn max(&self, other: &Self) -> Self {
        let mut result = Self([0; N]);
        for i in 0..N {
            result.0[i] = self.0[i].max(other.0[i]);
        }
        result
    }

    fn iterate_to(&self, to: &Self) -> Self::IntoIter {
        PosIter::new(self, to)
    }

    fn get_neighbors(&self) -> Self::IntoIter {
        let min = *self - 1;
        let max = *self + 1;
        min.iterate_to(&max)
    }
}

impl<const N: usize> PosIter<N> {
    fn new(from: &PosND<N>, to: &PosND<N>) -> Self {
        let mut current = *from;
        current.0[N - 1] -= 1;
        PosIter {
            from: *from,
            to: *to,
            current,
        }
    }
}

impl<const N: usize> Iterator for PosIter<N> {
    type Item = PosND<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.0[0] > self.to.0[0] {
            None
        } else {
            for i in (0..=N - 1).rev() {
                self.current.0[i] += 1;
                if self.current.0[i] > self.to.0[i] && i > 0 {
                    self.current.0[i] = self.from.0[i];
                } else {
                    break;
                }
            }
            if self.current.0[0] > self.to.0[0] {
                None
            } else {
                Some(self.current)
            }
        }
    }
}

struct Board<T: Position> {
    map: HashSet<T>,
    min: T,
    max: T,
}

impl<T: Position> Board<T> {
    fn parse_initial(s: &str) -> Self {
        let map: HashSet<_> = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| T::from_2d(x as isize, y as isize))
            })
            .flatten()
            .collect();
        Board {
            min: map.iter().fold(T::fill(isize::MAX), |a, b| a.min(b)) - 1,
            max: map.iter().fold(T::fill(isize::MIN), |a, b| a.max(b)) + 1,
            map,
        }
    }

    fn iterate(&self) -> Self {
        let mut new_map = HashSet::new();
        let mut min: Option<T> = None;
        let mut max: Option<T> = None;

        for pos in self.min.iterate_to(&self.max) {
            let n = self.get_neighbors(&pos);
            let on_pos = self.map.get(&pos);
            if (on_pos.is_some() && (n == 2 || n == 3)) || (on_pos.is_none() && n == 3) {
                min = Some(min.map_or(pos, |m| m.min(&pos)));
                max = Some(max.map_or(pos, |m| m.max(&pos)));
                new_map.insert(pos);
            }
        }
        Board {
            map: new_map,
            min: min.unwrap() - 1,
            max: max.unwrap() + 1,
        }
    }

    fn get_neighbors(&self, pt: &T) -> usize {
        let mut result = 0;
        for pos in pt.get_neighbors() {
            if &pos != pt && self.map.get(&pos).is_some() {
                result += 1;
            }
        }
        result
    }
}

impl<T: Position> Clone for Board<T> {
    fn clone(&self) -> Self {
        Board {
            map: self.map.clone(),
            min: self.min,
            max: self.max,
        }
    }
}

fn read_input<T: Position>() -> Board<T> {
    let data = include_str!("../../inputs/day17.txt");
    Board::parse_initial(&data)
}

fn part1() {
    let board = read_input::<PosND<3>>();
    let result = (0..6).fold(board, |b, _| b.iterate());
    println!("Part 1: {}", result.map.len());
}

fn part2() {
    let board = read_input::<PosND<4>>();
    let result = (0..6).fold(board, |b, _| b.iterate());
    println!("Part 2: {}", result.map.len());
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    type P1 = PosND<1>;
    type P2 = PosND<2>;

    #[test]
    fn pos1d() {
        let zero = P1::fill(0);
        let one = P1::fill(1);

        assert_eq!(PosND([0]), zero);
        assert_eq!(PosND([1]), one);

        assert_eq!(PosND([0]), one.min(&zero));
        assert_eq!(PosND([0]), zero.min(&one));

        assert_eq!(PosND([1]), one.max(&zero));
        assert_eq!(PosND([1]), zero.max(&one));
    }

    #[test]
    pub fn pos1d_iterate() {
        let from = P1::fill(0);
        let to = P1::fill(5);
        let mut iter = from.iterate_to(&to);
        assert_eq!(Some(PosND([0])), iter.next());
        assert_eq!(Some(PosND([1])), iter.next());
        assert_eq!(Some(PosND([2])), iter.next());
        assert_eq!(Some(PosND([3])), iter.next());
        assert_eq!(Some(PosND([4])), iter.next());
        assert_eq!(Some(PosND([5])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    pub fn pos2d_iterate() {
        let from = P2::from_2d(0, 0);
        let to = P2::from_2d(2, 2);
        let mut iter = from.iterate_to(&to);
        assert_eq!(Some(PosND([0, 0])), iter.next());
        assert_eq!(Some(PosND([0, 1])), iter.next());
        assert_eq!(Some(PosND([0, 2])), iter.next());
        assert_eq!(Some(PosND([1, 0])), iter.next());
        assert_eq!(Some(PosND([1, 1])), iter.next());
        assert_eq!(Some(PosND([1, 2])), iter.next());
        assert_eq!(Some(PosND([2, 0])), iter.next());
        assert_eq!(Some(PosND([2, 1])), iter.next());
        assert_eq!(Some(PosND([2, 2])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    pub fn parse_initial() {
        let board: Board<PosND<2>> = Board::parse_initial(".#.\n..#\n###");
        assert_eq!(5, board.map.len());
        assert!(board.map.get(&PosND([1, 0])).is_some());
        assert!(board.map.get(&PosND([2, 1])).is_some());
        assert!(board.map.get(&PosND([0, 2])).is_some());
        assert!(board.map.get(&PosND([1, 2])).is_some());
        assert!(board.map.get(&PosND([2, 2])).is_some());

        assert_eq!(PosND([-1, -1]), board.min);
        assert_eq!(PosND([3, 3]), board.max);
    }

    #[test]
    pub fn iterate() {
        let board: Board<PosND<2>> = Board::parse_initial(".#.\n..#\n###");
        let board = board.iterate();

        assert_eq!(5, board.map.len());
        assert!(board.map.get(&PosND([0, 1])).is_some());
        assert!(board.map.get(&PosND([2, 1])).is_some());
        assert!(board.map.get(&PosND([1, 2])).is_some());
        assert!(board.map.get(&PosND([2, 2])).is_some());
        assert!(board.map.get(&PosND([1, 3])).is_some());
        assert_eq!(PosND([-1, 0]), board.min);
        assert_eq!(PosND([3, 4]), board.max);
    }

    #[bench]
    pub fn iterate_2d(b: &mut Bencher) {
        let board: Board<PosND<2>> = Board::parse_initial(".#.\n..#\n###");
        b.iter(|| {
            let b = board.iterate();
            (0..9).fold(b, |b, _| b.iterate());
        });
    }

    #[bench]
    pub fn iterate_3d(b: &mut Bencher) {
        let board: Board<PosND<3>> = Board::parse_initial(".#.\n..#\n###");
        b.iter(|| {
            let b = board.iterate();
            (0..9).fold(b, |b, _| b.iterate());
        });
    }

    #[bench]
    pub fn iterate_4d(b: &mut Bencher) {
        let board: Board<PosND<4>> = Board::parse_initial(".#.\n..#\n###");
        b.iter(|| {
            let b = board.iterate();
            (0..9).fold(b, |b, _| b.iterate());
        });
    }
}
