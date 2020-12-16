use lazy_static::lazy_static;
use regex::Regex;
use std::io::{prelude::*, BufReader};
use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum MaskBit {
    Zero,
    One,
    Unknown,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mask(Vec<MaskBit>),
    Write(u64, u64),
}

#[derive(Debug)]
struct State {
    mask: Vec<MaskBit>,
    mem: HashMap<u64, u64>,
    floating: Vec<(u64, u64)>,
}

impl MaskBit {
    fn parse(c: char) -> Self {
        match c {
            '0' => Self::Zero,
            '1' => Self::One,
            'X' => Self::Unknown,
            _ => panic!("unknown char"),
        }
    }
}

impl Instruction {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref MASK_REGEX: Regex = Regex::new("mask = ([X01]{36})").unwrap();
            static ref WRITE_REGEX: Regex = Regex::new("mem\\[(\\d+)\\] = (\\d+)").unwrap();
        }
        if let Some(mask) = MASK_REGEX.captures(s) {
            Self::Mask(
                mask.get(1)
                    .unwrap()
                    .as_str()
                    .chars()
                    .map(MaskBit::parse)
                    .collect(),
            )
        } else if let Some(caps) = WRITE_REGEX.captures(s) {
            Instruction::Write(
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            )
        } else {
            panic!("unknown instruction");
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            mask: vec![],
            mem: HashMap::new(),
            floating: vec![],
        }
    }

    fn execute_part1(&mut self, instr: Vec<Instruction>) {
        for i in instr.into_iter() {
            match i {
                Instruction::Mask(m) => self.set_mask(m),
                Instruction::Write(addr, val) => {
                    self.mem.insert(addr, self.mask_value(val));
                }
            }
        }
    }

    fn execute_part2(&mut self, instr: Vec<Instruction>) {
        let mut new_mem = self.mem.clone();
        for i in instr.into_iter() {
            match i {
                Instruction::Mask(m) => self.set_mask(m),
                Instruction::Write(addr, val) => {
                    for a in self.mask_address(addr) {
                        new_mem.insert(a, val);
                    }
                }
            }
        }
        self.mem = new_mem;
    }

    fn set_mask(&mut self, mut m: Vec<MaskBit>) {
        m.reverse();
        self.mask = m;
        self.floating = self
            .mask
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == MaskBit::Unknown)
            .map(|(i, _)| i)
            .enumerate()
            .map(|(s, d)| (s as u64, d as u64))
            .collect();
    }

    fn mask_value(&self, mut val: u64) -> u64 {
        for (i, v) in self.mask.iter().enumerate() {
            match v {
                MaskBit::Zero => {
                    val &= !(1 << i);
                }
                MaskBit::One => {
                    val |= 1 << i;
                }
                MaskBit::Unknown => {}
            }
        }
        val
    }

    fn mask_address<'a>(&'a self, mut addr: u64) -> impl Iterator<Item = u64> + 'a {
        for (i, v) in self.mask.iter().enumerate() {
            match v {
                MaskBit::One => {
                    addr |= 1 << i;
                }
                _ => {}
            }
        }
        let addr = addr; // Do not mutate from now on
        let max_val = 1 << self.floating.len();
        (0..max_val).map(move |val| {
            let mut res = addr;
            for (six, dix) in &self.floating {
                let bit = val & (1 << six);
                if bit == 0 {
                    res &= !(1 << dix);
                } else {
                    res |= 1 << dix;
                }
            }
            res
        })
    }
}

fn read_input() -> Vec<Instruction> {
    BufReader::new(fs::File::open("./inputs/day14.txt").unwrap())
        .lines()
        .map(|x| Instruction::parse(&x.unwrap()))
        .collect()
}

fn part1() {
    let instr = read_input();
    let mut state = State::new();
    state.execute_part1(instr);
    let result: u64 = state.mem.values().sum();
    println!("Part 1: {}", result);
}

fn part2() {
    let instr = read_input();
    let mut state = State::new();
    state.execute_part2(instr);
    let result: u64 = state.mem.values().sum();
    println!("Part 2: {}", result);
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
        assert_eq!(
            Instruction::Mask(vec![
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::One,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Unknown,
                MaskBit::Zero,
                MaskBit::Unknown,
            ]),
            Instruction::parse("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")
        );

        assert_eq!(Instruction::Write(8, 11), Instruction::parse("mem[8] = 11"));
    }

    #[test]
    fn mask_value() {
        let mut s = State {
            mask: vec![],
            mem: HashMap::new(),
            floating: vec![],
        };
        s.execute_part1(vec![Instruction::parse(
            "mask = 000000000000000000000000000000X1001X",
        )]);
        let mut addrs: Vec<_> = s.mask_address(42).collect();
        addrs.sort();

        assert_eq!(vec![26, 27, 58, 59], addrs);
    }

    #[test]
    fn mask_address() {
        let mut s = State {
            mask: vec![],
            mem: HashMap::new(),
            floating: vec![],
        };
        s.execute_part1(vec![Instruction::parse(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
        )]);

        assert_eq!(73, s.mask_value(11));
        assert_eq!(101, s.mask_value(101));
        assert_eq!(64, s.mask_value(0));
    }
}
