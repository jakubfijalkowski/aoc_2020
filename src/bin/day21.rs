use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl Food {
    fn parse(s: &str) -> Food {
        let paren = s.find('(').unwrap();
        let ingredients: Vec<_> = s[..paren - 1].split(' ').map(|x| x.to_string()).collect();
        let allergens: Vec<_> = s[paren + "(contains ".len()..s.len() - 1]
            .split(", ")
            .map(|x| x.to_string())
            .collect();
        Food {
            ingredients,
            allergens,
        }
    }
}

fn part1() {
    let foods: Vec<_> = include_str!("../../inputs/day21.txt")
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(Food::parse)
        .collect();
    let mut allergen_freq: HashMap<_, HashMap<&str, u32>> = foods
        .iter()
        .map(|x| &x.allergens)
        .flatten()
        .map(|x| (x, HashMap::new()))
        .collect();
    for f in &foods {
        for allergen in &f.allergens {
            let freqs = allergen_freq.get_mut(allergen).unwrap();
            for ing in &f.ingredients {
                if let Some(freq) = freqs.get_mut(&ing[..]) {
                    *freq += 1;
                } else {
                    freqs.insert(ing, 1);
                }
            }
        }
    }

    let reduced: HashSet<_> = allergen_freq
        .iter()
        .map(|(_, freqs)| {
            let max_freq = *freqs.values().max().unwrap();
            freqs
                .iter()
                .filter(move |(_, f)| **f == max_freq)
                .map(|(ing, _)| ing)
        })
        .flatten()
        .copied()
        .collect();

    let all_ings: HashSet<_> = foods
        .iter()
        .map(|x| x.ingredients.iter().map(|x| &x[..]))
        .flatten()
        .collect();
    let wo_allergens: HashSet<_> = all_ings.difference(&reduced).collect();

    let count: usize = foods
        .iter()
        .map(|x| x.ingredients.iter().map(|x| &x[..]))
        .flatten()
        .filter(|x| wo_allergens.contains(x))
        .count();

    println!("Part 1: {}", count);
}

fn select(
    reduced: &[(&str, Vec<&str>)],
    taken: &mut HashSet<String>,
    result: &mut Vec<(String, String)>,
) -> bool {
    if reduced.len() == result.len() {
        return true;
    }

    let (allergen, ings) = &reduced[result.len()];
    for ing in ings {
        let ing = ing.to_string();
        if !taken.contains(&ing) {
            taken.insert(ing.to_string());
            result.push((allergen.to_string(), ing.to_string()));
            if select(reduced, taken, result) {
                return true;
            }
            result.pop();
            taken.remove(&ing);
        }
    }

    false
}

fn part2() {
    let foods: Vec<_> = include_str!("../../inputs/day21.txt")
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(Food::parse)
        .collect();
    let mut allergen_freq: HashMap<_, HashMap<&str, u32>> = foods
        .iter()
        .map(|x| &x.allergens)
        .flatten()
        .map(|x| (x, HashMap::new()))
        .collect();
    for f in &foods {
        for allergen in &f.allergens {
            let freqs = allergen_freq.get_mut(allergen).unwrap();
            for ing in &f.ingredients {
                if let Some(freq) = freqs.get_mut(&ing[..]) {
                    *freq += 1;
                } else {
                    freqs.insert(ing, 1);
                }
            }
        }
    }

    let reduced: Vec<_> = allergen_freq
        .iter()
        .map(|(allergen, freqs)| {
            let max_freq = *freqs.values().max().unwrap();
            let possibilities: Vec<_> = freqs
                .iter()
                .filter(move |(_, f)| **f == max_freq)
                .map(|(ing, _)| *ing)
                .collect();
            (&allergen[..], possibilities)
        })
        .collect();


    let mut result = Vec::new();
    if !select(&reduced[..], &mut HashSet::new(), &mut result) {
        panic!("no result");
    }
    result.sort_by(|a, b| a.0.cmp(&b.0));
    let result: Vec<_> = result.into_iter().map(|x| x.1).collect();
    let result = &result[..].join(",");
    println!("Part 2: {}", result);
}

fn main() {
    part1();
    part2();
}
