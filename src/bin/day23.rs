#![feature(assoc_char_funcs)]

#[derive(Debug)]
struct RingBuffer<T>(Vec<T>);

impl<T> RingBuffer<T> {
    fn new(v: Vec<T>) -> Self {
        RingBuffer(v)
    }

    fn get(&self, idx: usize) -> &T {
        let idx = idx % self.0.len();
        &self.0[idx]
    }

    fn remove_after(&mut self, idx: usize) -> [T; 3]
    where
        T: Copy,
    {
        let idx1 = self.coerce(idx + 1);
        let idx2 = self.coerce(idx + 2);
        let idx3 = self.coerce(idx + 3);
        let res = [self.0[idx1], self.0[idx2], self.0[idx3]];
        if idx3 > idx1 {
            self.0.drain(idx1..=idx3);
        } else if idx2 > idx1 {
            self.0.remove(idx2);
            self.0.remove(idx1);
            self.0.remove(idx3);
        } else {
            self.0.remove(idx1);
            self.0.remove(idx3);
            self.0.remove(idx2);
        }
        res
    }

    fn insert_after(&mut self, idx: usize, data: [T; 3])
    where
        T: Copy,
    {
        let idx = idx + 1;
        self.0.splice(idx..idx, data.iter().copied());
    }

    fn find_index(&self, v: &T, start_at: usize) -> Option<usize>
    where
        T: PartialEq,
    {
        for i in 0..self.0.len() {
            if self.get(start_at + i).eq(v) {
                return Some(start_at + i);
            }
        }
        None
    }

    fn coerce(&self, idx: usize) -> usize {
        idx % self.0.len()
    }
}

fn find_dest(buffer: &RingBuffer<u32>, mut cnt: u32, removed: &[u32; 3], max: u32) -> usize {
    cnt -= 1;
    while cnt == 0 || removed.contains(&cnt) {
        if cnt == 0 {
            cnt = max;
        } else {
            cnt -= 1;
        }
    }
    buffer.find_index(&cnt, 0).unwrap()
}

fn turn(buffer: &mut RingBuffer<u32>, idx: usize, max: u32) -> usize {
    let cnt = *buffer.get(idx);
    let nums = buffer.remove_after(idx);

    let dst_idx = find_dest(buffer, cnt, &nums, max);
    buffer.insert_after(dst_idx, nums);
    buffer.coerce(buffer.find_index(&cnt, idx - 3).unwrap() + 1)
}

fn part1() {
    const MOVES: usize = 100;
    let buffer: Vec<u32> = include_str!("../../inputs/day23.txt")
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|x| x as u32 - '0' as u32)
        .collect();
    let mut buffer = RingBuffer::new(buffer);
    (0..MOVES).fold(0, |idx, _| turn(&mut buffer, idx, 9));
    let one = buffer.find_index(&1, 0).unwrap();
    let result = (1..9).fold(String::new(), |mut s, i| {
        s.push(char::from_u32(buffer.get(one + i) + '0' as u32).unwrap());
        s
    });

    println!("Part 1: {}", result);
}

fn part2() {
    const MAX_VALUE: u32 = 1000000;
    const MOVES: usize = 10000000;
    let mut buffer: Vec<u32> = include_str!("../../inputs/day23.txt")
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|x| x as u32 - '0' as u32)
        .collect();
    let max = buffer.iter().max().unwrap();
    for i in max + 1..=MAX_VALUE {
        buffer.push(i);
    }

    let mut buffer = RingBuffer::new(buffer);
    (0..MOVES).fold(0, |idx, i| {
        if i % 100000 == 0 {
            println!("Iter: {}", i);
        }
        turn(&mut buffer, idx, MAX_VALUE)
    });
    let one = buffer.find_index(&1, 0).unwrap();
    let result = (*buffer.get(one + 1) as u64) * (*buffer.get(one + 1) as u64);
    println!("Part 2: {}", result);
}

fn main() {
    part1();
    part2();
}
