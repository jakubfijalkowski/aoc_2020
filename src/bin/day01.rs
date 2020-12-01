use aoc_2020::pairwise::*;
use std::fs;
use std::io::{prelude::*, BufReader};

fn read_input() -> Vec<i32> {
    BufReader::new(fs::File::open("./inputs/day1.txt").unwrap())
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect()
}

fn part1() {
    const GOAL: i32 = 2020;

    let data = read_input();
    let (x, y) = Pairwise::from(&data, &data)
        .filter(|(&x, &y)| x + y == GOAL)
        .next()
        .unwrap();
    println!("Part 1: {} * {} = {}", x, y, x * y);
}

fn part2() {
    const GOAL: i32 = 2020;

    let data = read_input();
    let pairs = Pairwise::from(&data, &data);
    let ((x, y), z) = Pairwise::from(pairs, &data)
        .filter(|((&x, &y), &z)| x + y + z == GOAL)
        .next()
        .unwrap();
    println!("Part 2: {} * {} * {} = {}", x, y, z, x * y * z);
}

fn main() {
    part1();
    part2();
}
