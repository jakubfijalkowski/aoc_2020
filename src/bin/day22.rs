use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Game {
    player1: Vec<u8>,
    player2: Vec<u8>,
}

impl Game {
    fn parse(s: &str) -> Game {
        let mut p = s.split("\n\n");
        Game {
            player1: Self::parse_deck(p.next().unwrap()),
            player2: Self::parse_deck(p.next().unwrap()),
        }
    }

    fn parse_deck(s: &str) -> Vec<u8> {
        s.split('\n')
            .skip(1)
            .filter_map(|x| x.parse().ok())
            .collect()
    }

    fn clone_for(&self, p1: u8, p2: u8) -> Game {
        let p1 = p1 as usize;
        let p2 = p2 as usize;
        Game {
            player1: self.player1.iter().take(p1).copied().collect(),
            player2: self.player2.iter().take(p2).copied().collect(),
        }
    }
}

fn part1() {
    let mut g = Game::parse(include_str!("../../inputs/day22.txt"));
    while !g.player1.is_empty() && !g.player2.is_empty() {
        let p1 = g.player1.remove(0);
        let p2 = g.player2.remove(0);
        if p1 > p2 {
            g.player1.push(p1);
            g.player1.push(p2);
        } else {
            g.player2.push(p2);
            g.player2.push(p1);
        }
    }
    let v = if g.player1.is_empty() {
        &g.player2
    } else {
        &g.player1
    };
    let res: u64 = v
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i as u64 + 1) * *x as u64)
        .sum();
    println!("Part 1: {}", res);
}

#[derive(Debug, PartialEq, Eq)]
enum Winner {
    Player1,
    Player2,
}

fn recursive_combat(mut g: Game) -> (Winner, Game) {
    let mut played = HashSet::new();
    while !g.player1.is_empty() && !g.player2.is_empty() {
        if played.contains(&g) {
            return (Winner::Player1, g);
        }
        played.insert(g.clone());

        let p1 = g.player1.remove(0);
        let p2 = g.player2.remove(0);
        let winner = if g.player1.len() >= p1 as usize && g.player2.len() >= p2 as usize {
            recursive_combat(g.clone_for(p1, p2)).0
        } else {
            if p1 > p2 {
                Winner::Player1
            } else {
                Winner::Player2
            }
        };
        if winner == Winner::Player1 {
            g.player1.push(p1);
            g.player1.push(p2);
        } else {
            g.player2.push(p2);
            g.player2.push(p1);
        }
    }

    if g.player1.is_empty() {
        (Winner::Player2, g)
    } else {
        (Winner::Player1, g)
    }
}

fn part2() {
    let g = Game::parse(include_str!("../../inputs/day22.txt"));
    let (winner, g) = recursive_combat(g);

    let v = if winner == Winner::Player2 {
        &g.player2
    } else {
        &g.player1
    };
    let res: u64 = v
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i as u64 + 1) * *x as u64)
        .sum();
    println!("Part 2: {}", res);
}

fn main() {
    part1();
    part2()
}
