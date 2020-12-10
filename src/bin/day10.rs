use std::{collections::HashMap, fs};
use std::io::{prelude::*, BufReader};

fn read_input() -> Vec<i64> {
    BufReader::new(fs::File::open("./inputs/day10.txt").unwrap())
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect()
}

fn part1() {
    let mut input = read_input();
    input.sort();
    let mut next: Vec<_> = input.iter().map(|x| *x).collect();

    input.insert(0, 0);
    next.push(next.iter().max().unwrap() + 3);

    let diffs: Vec<_> = input
        .iter()
        .zip(next.iter())
        .map(|(p, n)| *n - *p)
        .collect();
    let diff1 = diffs.iter().filter(|&x| *x == 1).count();
    let diff3 = diffs.iter().filter(|&x| *x == 3).count();

    println!("Part1: {}", diff1 * diff3);
}

fn count_possibilities(from: usize, data: &[i64], saved: &mut HashMap<usize, i64>) -> i64 {
    if from >= data.len() {
        return 0;
    }
    if let Some(s) = saved.get(&from) {
        return *s;
    }

    let v = data[from];
    let mut poss = count_possibilities(from + 1, data, saved);
    poss = poss
        + if from + 2 < data.len() && data[from + 2] - v <= 3 {
            1 + count_possibilities(from + 2, data, saved)
        } else {
            0
        };
    poss = poss
        + if from + 3 < data.len() && data[from + 3] - v <= 3 {
            1 + count_possibilities(from + 3, data, saved)
        } else {
            0
        };
    saved.insert(from, poss);
    poss
}

fn part2() {
    let mut input = read_input();
    input.sort();
    input.insert(0, 0);
    let mut saved = HashMap::new();
    let poss = 1 + count_possibilities(0, &input[..], &mut saved);
    println!("Part 2: {}", poss);
}

fn main() {
    part1();
    part2();
}
