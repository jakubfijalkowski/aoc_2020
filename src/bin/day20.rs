#![feature(generators, generator_trait)]
use std::marker::Unpin;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use std::{collections::HashSet, fmt::Debug};

const TILE_SIZE: usize = 10;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Configuration {
    tile_id: u16,
    flipped_x: bool,
    flipped_y: bool,
    rotation: u8,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct TileConfiguration {
    top: u16,
    right: u16,
    bottom: u16,
    left: u16,
    cfg: Configuration,
}

struct Tile {
    id: u16,
    data: [bool; TILE_SIZE * TILE_SIZE],
    configs: Vec<TileConfiguration>,
}

impl TileConfiguration {
    fn from_normal(tile_id: u16, data: &[bool; TILE_SIZE * TILE_SIZE]) -> TileConfiguration {
        TileConfiguration {
            top: Self::generate(data, 0, TILE_SIZE, |x| x),
            right: Self::generate(data, 0, TILE_SIZE, |y| y * TILE_SIZE + TILE_SIZE - 1),
            bottom: Self::generate(data, TILE_SIZE, 0, |x| (TILE_SIZE - 1) * TILE_SIZE + x),
            left: Self::generate(data, TILE_SIZE, 0, |y| y * TILE_SIZE),
            cfg: Configuration {
                tile_id,
                flipped_x: false,
                flipped_y: false,
                rotation: 0,
            },
        }
    }

    fn from_flipped_x(tile_id: u16, data: &[bool; TILE_SIZE * TILE_SIZE]) -> TileConfiguration {
        TileConfiguration {
            top: Self::generate(data, 0, TILE_SIZE, |x| (TILE_SIZE - 1) * TILE_SIZE + x),
            right: Self::generate(data, TILE_SIZE, 0, |y| y * TILE_SIZE + TILE_SIZE - 1),
            bottom: Self::generate(data, TILE_SIZE, 0, |x| x),
            left: Self::generate(data, 0, TILE_SIZE, |y| y * TILE_SIZE),
            cfg: Configuration {
                tile_id,
                flipped_x: true,
                flipped_y: false,
                rotation: 0,
            },
        }
    }

    fn from_flipped_y(tile_id: u16, data: &[bool; TILE_SIZE * TILE_SIZE]) -> TileConfiguration {
        TileConfiguration {
            top: Self::generate(data, TILE_SIZE, 0, |x| x),
            right: Self::generate(data, 0, TILE_SIZE, |y| y * TILE_SIZE),
            bottom: Self::generate(data, 0, TILE_SIZE, |x| (TILE_SIZE - 1) * TILE_SIZE + x),
            left: Self::generate(data, 0, TILE_SIZE, |y| y * TILE_SIZE + TILE_SIZE - 1),
            cfg: Configuration {
                tile_id,
                flipped_x: false,
                flipped_y: true,
                rotation: 0,
            },
        }
    }

    fn from_flipped_xy(tile_id: u16, data: &[bool; TILE_SIZE * TILE_SIZE]) -> TileConfiguration {
        TileConfiguration {
            top: Self::generate(data, TILE_SIZE, 0, |x| (TILE_SIZE - 1) * TILE_SIZE + x),
            right: Self::generate(data, TILE_SIZE, 0, |y| y * TILE_SIZE),
            bottom: Self::generate(data, 0, TILE_SIZE, |x| x),
            left: Self::generate(data, 0, TILE_SIZE, |y| y * TILE_SIZE + TILE_SIZE - 1),
            cfg: Configuration {
                tile_id,
                flipped_x: true,
                flipped_y: true,
                rotation: 0,
            },
        }
    }

    fn rotate(&self, times: u8) -> Self {
        let r = |x: &TileConfiguration| Self {
            top: x.left,
            right: x.top,
            bottom: x.right,
            left: x.bottom,
            cfg: Configuration {
                tile_id: x.cfg.tile_id,
                flipped_x: x.cfg.flipped_x,
                flipped_y: x.cfg.flipped_y,
                rotation: x.cfg.rotation + 1,
            },
        };
        let mut result = *self;
        for _ in 0..times {
            result = r(&result);
        }

        result.bottom = Self::reverse(result.bottom);
        result.left = Self::reverse(result.left);
        result
    }

    fn generate<F>(data: &[bool; TILE_SIZE * TILE_SIZE], a0: usize, a1: usize, idx: F) -> u16
    where
        F: Fn(usize) -> usize,
    {
        if a0 < a1 {
            let mut result = 0;
            for a in a0..a1 {
                if data[idx(a)] {
                    result |= 1 << a;
                }
            }
            result
        } else {
            let mut result = 0;
            for (i, a) in (a1..a0).rev().enumerate() {
                if data[idx(a)] {
                    result |= 1 << i;
                }
            }
            result
        }
    }

    fn reverse(mut v: u16) -> u16 {
        let mut r = v;
        let mut s = 8 * 2 - 1;
        v >>= 1;
        while v != 0 {
            r <<= 1;
            r |= v & 1;
            v >>= 1;
            s -= 1;
        }

        r <<= s;
        r >> 6
    }
}

impl Debug for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to01 = |x: bool| if x { "1" } else { "0" };
        write!(
            f,
            "{} {}{}{}",
            self.tile_id,
            to01(self.flipped_x),
            to01(self.flipped_y),
            self.rotation,
        )
    }
}

impl Debug for TileConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:010b} {:010b} {:010b} {:010b} {:?}",
            self.top, self.right, self.bottom, self.left, self.cfg,
        )
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                if self.data[y * TILE_SIZE + x] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        for (i, b) in self.configs.iter().enumerate() {
            writeln!(f, "{:02}: {:?}", i, b)?;
        }
        Ok(())
    }
}

impl Tile {
    fn parse(s: &str) -> Self {
        let colon = s.find(':').unwrap();
        let id: u16 = s[5..colon].parse().unwrap();
        let mut data = [false; TILE_SIZE * TILE_SIZE];

        for (y, l) in s.split('\n').skip(1).enumerate() {
            l.chars()
                .enumerate()
                .for_each(|(x, c)| data[y * TILE_SIZE + x] = c == '#');
        }

        let normal = TileConfiguration::from_normal(id, &data);
        let fx = TileConfiguration::from_flipped_x(id, &data);
        let fy = TileConfiguration::from_flipped_y(id, &data);
        let fxy = TileConfiguration::from_flipped_xy(id, &data);
        Self {
            id,
            data,
            configs: vec![
                normal.rotate(0),
                normal.rotate(1),
                normal.rotate(2),
                normal.rotate(3),
                fx.rotate(0),
                fx.rotate(1),
                fx.rotate(2),
                fx.rotate(3),
                fy.rotate(0),
                fy.rotate(1),
                fy.rotate(2),
                fy.rotate(3),
                fxy.rotate(0),
                fxy.rotate(1),
                fxy.rotate(2),
                fxy.rotate(3),
            ],
        }
    }

    fn parse_all(s: &str) -> Vec<Tile> {
        s.split("\n\n").map(Self::parse).collect()
    }
}

struct Board {
    tiles: Vec<Tile>,
    borders: Vec<TileConfiguration>,
    candidates: Vec<Vec<TileConfiguration>>,
    size: usize,
}

impl Board {
    fn empty(tiles: Vec<Tile>) -> Self {
        let size = (tiles.len() as f64).sqrt() as usize;
        let borders = tiles
            .iter()
            .map(|t| &t.configs)
            .flatten()
            .copied()
            .collect();
        Board {
            tiles,
            borders,
            candidates: vec![Vec::new(); size * size],
            size,
        }
    }

    fn build_image(&self) -> Vec<TileConfiguration> {
        let mut result = Vec::new();
        let mut taken = HashSet::new();
        if !self.fill_one(&mut result, &mut taken, 0, 0) {
            panic!("Should not happen");
        }
        result
    }

    fn fill_one(
        &self,
        result: &mut Vec<TileConfiguration>,
        taken: &mut HashSet<u16>,
        mut x: usize,
        mut y: usize,
    ) -> bool {
        if x == self.size {
            x = 0;
            y += 1;
        }
        if y == self.size {
            return true;
        }

        let idx = y * self.size + x;
        for c in &self.candidates[idx] {
            let ix = x as isize;
            let iy = y as isize;
            if !taken.contains(&c.cfg.tile_id)
                && self.fits(result, &c, ix - 1, iy, |b| b.left, |b| b.right)
                && self.fits(result, &c, ix, iy - 1, |b| b.top, |b| b.bottom)
            {
                taken.insert(c.cfg.tile_id);
                result.push(*c);
                if self.fill_one(result, taken, x + 1, y) {
                    return true;
                }
                result.pop();
                taken.remove(&c.cfg.tile_id);
            }
        }

        false
    }

    fn prepare_candidates(&mut self) {
        self.prepare_corners();

        for y in 0..self.size {
            for x in 0..self.size {
                if self.candidates[y * self.size + x].is_empty() {
                    for b in &self.borders {
                        let ix = x as isize;
                        let iy = y as isize;
                        let left = self.matches(b, ix - 1, iy, |b| b.left, |b| b.right);
                        let top = self.matches(b, ix, iy - 1, |b| b.top, |b| b.bottom);
                        let right = self.matches(b, ix + 1, iy, |b| b.right, |b| b.left);
                        let bottom = self.matches(b, ix, iy + 1, |b| b.bottom, |b| b.top);
                        if left && top && right && bottom {
                            self.candidates[y * self.size + x].push(*b);
                        }
                    }
                }
            }
        }
    }

    fn fits<F1, F2>(
        &self,
        result: &Vec<TileConfiguration>,
        b: &TileConfiguration,
        wx: isize,
        wy: isize,
        get_self: F1,
        get_near: F2,
    ) -> bool
    where
        F1: Fn(&TileConfiguration) -> u16,
        F2: Fn(&TileConfiguration) -> u16,
    {
        debug_assert!(wx < self.size as isize && wy < self.size as isize);
        let idx = wy * self.size as isize + wx;
        if wx >= 0 && wx < self.size as isize && 0 <= wy && idx < result.len() as isize {
            let idx = idx as usize;
            let near = &result[idx];
            get_self(b) == get_near(near)
        } else {
            true
        }
    }

    fn matches<F1, F2>(
        &self,
        b: &TileConfiguration,
        wx: isize,
        wy: isize,
        get_self: F1,
        get_near: F2,
    ) -> bool
    where
        F1: Fn(&TileConfiguration) -> u16,
        F2: Fn(&TileConfiguration) -> u16,
    {
        if 0 <= wx && wx < self.size as isize && 0 <= wy && wy < self.size as isize {
            let wx = wx as usize;
            let wy = wy as usize;
            let near = &self.candidates[wy * self.size + wx];
            near.is_empty()
                || near
                    .iter()
                    .any(|b2| b.cfg.tile_id != b2.cfg.tile_id && get_self(b) == get_near(b2))
        } else {
            !self
                .borders
                .iter()
                .any(|b2| b.cfg.tile_id != b2.cfg.tile_id && get_self(b) == get_near(b2))
        }
    }

    fn prepare_corners(&mut self) {
        self.candidates[0] = self.find_corners(|b| b.left, |b| b.right, |b| b.top, |b| b.bottom);
        self.candidates[self.size - 1] =
            self.find_corners(|b| b.right, |b| b.left, |b| b.top, |b| b.bottom);
        self.candidates[(self.size - 1) * self.size + self.size - 1] =
            self.find_corners(|b| b.right, |b| b.left, |b| b.bottom, |b| b.top);
        self.candidates[(self.size - 1) * self.size] =
            self.find_corners(|b| b.left, |b| b.right, |b| b.bottom, |b| b.top);
    }

    fn find_corners<F1, F2, F3, F4>(
        &self,
        get_self_h: F1,
        get_near_h: F2,
        get_self_v: F3,
        get_near_v: F4,
    ) -> Vec<TileConfiguration>
    where
        F1: Fn(&TileConfiguration) -> u16,
        F2: Fn(&TileConfiguration) -> u16,
        F3: Fn(&TileConfiguration) -> u16,
        F4: Fn(&TileConfiguration) -> u16,
    {
        let mut result = Vec::new();
        for b in &self.borders {
            let matches_h = self
                .borders
                .iter()
                .any(|b2| b.cfg.tile_id != b2.cfg.tile_id && get_self_h(b) == get_near_h(b2));
            let matches_v = self
                .borders
                .iter()
                .any(|b2| b.cfg.tile_id != b2.cfg.tile_id && get_self_v(b) == get_near_v(b2));
            if !matches_h && !matches_v {
                result.push(*b);
            }
        }
        result
    }
}

fn flip_x(size: usize, data: &[bool]) -> Vec<bool> {
    let mut result = vec![false; data.len()];
    for y in 0..size {
        for x in 0..size {
            let dst = y * size + x;
            let src = (size - y - 1) * size + x;
            result[dst] = data[src];
        }
    }
    result
}

fn flip_y(size: usize, data: &[bool]) -> Vec<bool> {
    let mut result = vec![false; data.len()];
    for y in 0..size {
        for x in 0..size {
            let dst = y * size + (size - x - 1);
            let src = y * size + x;
            result[dst] = data[src];
        }
    }
    result
}

fn rotate_once(size: usize, data: &[bool]) -> Vec<bool> {
    let mut result = vec![false; data.len()];
    for y in 0..size {
        for x in 0..size {
            let src = y * size + x;
            let dst = x * size + (size - y - 1);
            result[dst] = data[src];
        }
    }
    result
}

fn rotate(size: usize, times: u8, mut data: Vec<bool>) -> Vec<bool> {
    for _ in 0..times {
        data = rotate_once(size, &data);
    }
    data
}

fn cut_and_place(
    old_size: usize,
    new_size: usize,
    data: Vec<bool>,
    result: &mut Vec<bool>,
    x: usize,
    y: usize,
) {
    for oy in 1..old_size - 1 {
        for ox in 1..old_size - 1 {
            let src = oy * old_size + ox;
            let dst = (oy + y - 1) * new_size + ox + x - 1;
            result[dst] = data[src];
        }
    }
}

struct Picture {
    size: usize,
    data: Vec<bool>,
}

impl Picture {
    fn from_image(img: Vec<TileConfiguration>, board: &Board) -> Self {
        let tile = TILE_SIZE - 2;
        let size = board.size * tile;
        let mut data = vec![false; size * size];
        for y in 0..board.size {
            for x in 0..board.size {
                Self::place_tile(
                    size,
                    &mut data,
                    &img[y * board.size + x],
                    board,
                    x * tile,
                    y * tile,
                );
            }
        }
        Picture { size, data }
    }

    fn place_tile(
        new_size: usize,
        result: &mut Vec<bool>,
        tile: &TileConfiguration,
        board: &Board,
        x: usize,
        y: usize,
    ) {
        let mut tile_data: Vec<_> = board
            .tiles
            .iter()
            .filter(|t| t.id == tile.cfg.tile_id)
            .next()
            .unwrap()
            .data
            .iter()
            .copied()
            .collect();

        if tile.cfg.flipped_x {
            tile_data = flip_x(TILE_SIZE, &tile_data);
        }
        if tile.cfg.flipped_y {
            tile_data = flip_y(TILE_SIZE, &tile_data);
        }
        if tile.cfg.rotation > 0 {
            tile_data = rotate(TILE_SIZE, tile.cfg.rotation, tile_data);
        }
        cut_and_place(TILE_SIZE, new_size, tile_data, result, x, y);
    }

    fn gen(&self, cfg: &Configuration) -> Self {
        let mut data = self.data.clone();

        if cfg.flipped_x {
            data = flip_x(TILE_SIZE, &data);
        }
        if cfg.flipped_y {
            data = flip_y(TILE_SIZE, &data);
        }
        if cfg.rotation > 0 {
            data = rotate(TILE_SIZE, cfg.rotation, data);
        }
        Self {
            data,
            size: self.size,
        }
    }

    fn try_match_with<F, I>(&self, gen: F) -> bool
    where
        I: Iterator<Item = (usize, usize)>,
        F: Fn() -> I,
    {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.try_match_at(x, y, gen()) {
                    return true;
                }
            }
        }

        false
    }

    fn mask_all<F, I>(&mut self, gen: F)
    where
        I: Iterator<Item = (usize, usize)>,
        F: Fn() -> I,
    {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.try_match_at(x, y, gen()) {
                    for (ox, oy) in gen() {
                        let idx = (y + oy) * self.size + x + ox;
                        self.data[idx] = false;
                    }
                }
            }
        }
    }

    fn try_match_at(&self, x: usize, y: usize, pts: impl Iterator<Item = (usize, usize)>) -> bool {
        for (ox, oy) in pts {
            let x = x + ox;
            let y = y + oy;
            if x >= self.size || y >= self.size || !self.data[y * self.size + x] {
                return false;
            }
        }
        true
    }
}

impl Debug for Picture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                write!(
                    f,
                    "{}",
                    if self.data[y * self.size + x] {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

struct GenIterator<'a, Yield, Return> {
    generator: Box<dyn Generator<Yield = Yield, Return = Return> + Unpin + 'a>,
}

impl<'a, Yield, Return> Iterator for GenIterator<'a, Yield, Return> {
    type Item = Yield;

    fn next(&mut self) -> Option<Self::Item> {
        let mut generator = self.generator.as_mut();
        match Pin::new(&mut generator).resume(()) {
            GeneratorState::Yielded(c) => Some(c),
            GeneratorState::Complete(_) => None,
        }
    }
}

fn gather<'a, Yield: 'a, Return: 'a>(
    generator: Box<dyn Generator<Yield = Yield, Return = Return> + Unpin + 'a>,
) -> impl Iterator<Item = Yield> + 'a {
    GenIterator { generator }
}

fn get_sea_monster() -> impl Iterator<Item = (usize, usize)> {
    gather(Box::new(|| {
        yield (0, 1);
        yield (1, 2);
        yield (4, 2);
        yield (5, 1);
        yield (6, 1);
        yield (7, 2);
        yield (10, 2);
        yield (11, 1);
        yield (12, 1);
        yield (13, 2);
        yield (16, 2);
        yield (17, 1);
        yield (18, 0);
        yield (18, 1);
        yield (19, 1);
    }))
}

fn get_possible_configs() -> impl Iterator<Item = Configuration> {
    fn gen(x: u8, y: u8, r: u8) -> Configuration {
        Configuration {
            tile_id: 0,
            flipped_x: x == 1,
            flipped_y: y == 1,
            rotation: r,
        }
    };
    gather(Box::new(|| {
        yield gen(0, 0, 0);
        yield gen(0, 0, 1);
        yield gen(0, 0, 2);
        yield gen(0, 0, 3);
        yield gen(1, 0, 0);
        yield gen(1, 0, 1);
        yield gen(1, 0, 2);
        yield gen(1, 0, 3);
        yield gen(0, 1, 0);
        yield gen(0, 1, 1);
        yield gen(0, 1, 2);
        yield gen(0, 1, 3);
        yield gen(1, 1, 0);
        yield gen(1, 1, 1);
        yield gen(1, 1, 2);
        yield gen(1, 1, 3);
    }))
}

fn find_with_sea_monster(mut p: Picture) -> Picture {
    for c in get_possible_configs() {
        p = p.gen(&c);
        if p.try_match_with(|| get_sea_monster()) {
            return p;
        }
    }
    panic!("no results")
}

fn part1() {
    let data = include_str!("../../inputs/day20.txt");
    let tiles = Tile::parse_all(data);
    let mut board = Board::empty(tiles);
    board.prepare_candidates();

    let img = board.build_image();
    let tl = img[0].cfg.tile_id as u64;
    let tr = img[board.size - 1].cfg.tile_id as u64;
    let bl = img[(board.size - 1) * board.size].cfg.tile_id as u64;
    let br = img[(board.size - 1) * board.size + board.size - 1]
        .cfg
        .tile_id as u64;
    println!("Part 1: {}", tl * tr * bl * br);
}

fn part2() {
    let data = include_str!("../../inputs/day20.txt");
    let tiles = Tile::parse_all(data);
    let mut board = Board::empty(tiles);
    board.prepare_candidates();
    let img = board.build_image();

    let picture = Picture::from_image(img, &board);
    let mut picture = find_with_sea_monster(picture);
    picture.mask_all(|| get_sea_monster());
    println!("Part 2: {}", picture.data.iter().filter(|x| **x).count());
}

fn main() {
    part1();
    part2();
}
