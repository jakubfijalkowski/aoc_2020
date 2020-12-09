use aoc_2020::pairwise::*;
use std::fs;
use std::io::{prelude::*, BufReader};

fn read_input() -> Vec<i64> {
    BufReader::new(fs::File::open("./inputs/day09.txt").unwrap())
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect()
}

fn try_find_pair(data: &[i64], goal: i64) -> bool {
    Pairwise::from(data, data).any(|(a, b)| a != b && a + b == goal)
}

fn try_match_contiguous_list(data: &[i64], goal: i64, start_at: usize) -> Option<&[i64]> {
    let mut sum = data[start_at];
    let mut idx = start_at + 1;
    while sum < goal && idx < data.len() {
        sum = sum + data[idx];
        if sum == goal {
            return Some(&data[start_at..=idx]);
        }
        idx = idx + 1;
    }
    None
}

fn part1() {
    const PREAMBLE: usize = 25;
    let input = read_input();
    let first_non_matching_number = input
        .iter()
        .enumerate()
        .skip(PREAMBLE)
        .filter(|(i, v)| !try_find_pair(&input[i - PREAMBLE..*i], **v))
        .next()
        .unwrap()
        .1;
    println!("Part 1: {}", first_non_matching_number);
}

fn part2() {
    const PREAMBLE: usize = 25;
    let input = read_input();
    let first_non_matching_number = *input
        .iter()
        .enumerate()
        .skip(PREAMBLE)
        .filter(|(i, v)| !try_find_pair(&input[i - PREAMBLE..*i], **v))
        .next()
        .unwrap()
        .1;
    let list = (0..input.len())
        .filter_map(|i| try_match_contiguous_list(&input, first_non_matching_number, i))
        .next()
        .unwrap();
    let min = list.iter().min().unwrap();
    let max = list.iter().max().unwrap();
    println!("Part 2: {}", min + max);
}

fn main() {
    part1();
    part2();
}
