use std::collections::HashMap;

fn read_input() -> Vec<usize> {
    let data = include_str!("../../inputs/day15.txt");
    data.split(',').map(|x| x.trim().parse().unwrap()).collect()
}

fn algo(target: usize) -> usize {
    let data = read_input();
    let mut last_seen = HashMap::new();
    let mut last_number = *data.last().unwrap();

    for i in 0..data.len() - 1 {
        last_seen.insert(data[i], i);
    }

    for i in data.len()..target {
        if let Some(&prev) = last_seen.get(&last_number) {
            last_seen.insert(last_number, i - 1);
            last_number = i - 1 - prev;
        } else {
            last_seen.insert(last_number, i - 1);
            last_number = 0;
        }
    }
    last_number
}

fn part1() {
    println!("Part 1: {}", algo(2020));
}

fn part2() {
    println!("Part 2: {}", algo(30000000));
}

fn main() {
    part1();
    part2();
}
