use std::io::{prelude::*, BufReader};
use std::{collections::HashMap, collections::HashSet, fs};

fn read_input() -> Vec<String> {
    BufReader::new(fs::File::open("./inputs/day06.txt").unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect()
}

fn part1() {
    let data = read_input();
    let mut total_count = 0;
    let mut set = HashSet::new();

    for line in data {
        if line == "" {
            total_count = total_count + set.len();
            set.clear();
        } else {
            line.chars().for_each(|c| {
                set.insert(c);
            });
        }
    }
    total_count = total_count + set.len();
    println!("Part 1: {}", total_count);
}

fn part2() {
    let data = read_input();
    let mut total_count = 0;
    let mut in_group = 0;
    let mut map = HashMap::new();

    for line in data {
        if line == "" {
            total_count = total_count + map.iter().filter(|(_, &v)| v == in_group).count();
            map.clear();
            in_group = 0;
        } else {
            in_group = in_group + 1;
            line.chars().for_each(|c| {
                if let Some(x) = map.get_mut(&c) {
                    *x = *x + 1;
                } else {
                    map.insert(c, 1);
                }
            });
        }
    }
    total_count = total_count + map.iter().filter(|(_, &v)| v == in_group).count();
    println!("Part 2: {}", total_count);
}

fn main() {
    part1();
    part2();
}
