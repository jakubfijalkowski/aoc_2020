#![feature(assoc_char_funcs)]

#[derive(Debug)]
struct RingBuffer(Vec<(usize, usize)>);

impl RingBuffer {
    fn new(max_value: usize) -> Self {
        let mut result: Vec<_> = (0..=max_value)
            .map(|i| (i.saturating_sub(1), i + 1))
            .collect();
        result[0] = (usize::MAX, usize::MAX); // Sentinel
        result[1].0 = max_value;
        result[max_value].1 = 1;
        RingBuffer(result)
    }

    fn reorder(&mut self, input: &[usize]) {
        for (idx, v) in input.iter().enumerate() {
            let prev = input[if idx == 0 { input.len() - 1 } else { idx - 1 }];
            let next = input[if idx == input.len() - 1 { 0 } else { idx + 1 }];
            self.0[*v] = (prev, next);
            self.0[prev].1 = *v;
            self.0[next].0 = *v;
        }

        if input.len() + 1 < self.0.len() {
            let last_idx = self.0.len() - 1;
            self.0[input[0]].0 = self.0.len() - 1;
            self.0[last_idx].1 = input[0];

            self.0[*input.last().unwrap()].1 = input.len() + 1;
            self.0[input.len() + 1].0 = *input.last().unwrap();
        }
    }

    fn prev(&self, v: usize) -> usize {
        self.0[v].0
    }

    fn next(&self, v: usize) -> usize {
        self.0[v].1
    }

    fn turn(&mut self, v: usize) -> usize {
        let f1 = self.next(v);
        let f2 = self.next(f1);
        let f3 = self.next(f2);

        let f4 = self.next(f3);
        self.0[v].1 = f4;
        self.0[f4].0 = v;

        let spliced = [f1, f2, f3];
        let mut dst = v - 1;
        while dst == 0 || spliced.contains(&dst) {
            if dst == 0 {
                dst = self.0.len() - 1;
            } else {
                dst -= 1;
            }
        }

        let prev_next = self.0[dst].1;
        self.0[dst].1 = f1;
        self.0[f1] = (dst, f2);
        self.0[f2] = (f1, f3);
        self.0[f3] = (f2, prev_next);
        self.0[prev_next].0 = f3;

        self.next(v)
    }
}

fn part1() {
    const MOVES: usize = 100;
    let initial_order: Vec<_> = include_str!("../../inputs/day23.txt")
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|x| x as usize - '0' as usize)
        .collect();
    let mut buffer = RingBuffer::new(9);
    buffer.reorder(&initial_order);
    (0..MOVES).fold(initial_order[0], |v, _| buffer.turn(v));
    let (result, _) = (1..9).fold((String::new(), buffer.next(1)), |(s, v), _| {
        (format!("{}{}", s, v), buffer.next(v))
    });

    println!("Part 1: {}", result);
}

fn part2() {
    const MAX_VALUE: usize = 1000000;
    const MOVES: usize = 10000000;
    let initial_order: Vec<_> = include_str!("../../inputs/day23.txt")
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|x| x as usize - '0' as usize)
        .collect();
    let mut buffer = RingBuffer::new(MAX_VALUE);
    buffer.reorder(&initial_order);
    (0..MOVES).fold(initial_order[0], |v, _| buffer.turn(v));
    let a = buffer.next(1);
    let b = buffer.next(a);
    println!("Part 2: {}", a * b);
}

fn main() {
    part1();
    part2();
}
