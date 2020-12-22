use serde::Deserialize;
use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Deserialize, Debug)]
struct WolframResponse {
    queryresult: WolframQueryResult,
}

#[derive(Deserialize, Debug)]
struct WolframQueryResult {
    pods: Vec<WolframPod>,
}

#[derive(Deserialize, Debug)]
struct WolframPod {
    subpods: Vec<WolframSubpod>,
}

#[derive(Deserialize, Debug)]
struct WolframSubpod {
    plaintext: String,
}

fn read_input() -> (u64, Vec<(u64, u64)>) {
    let mut lines = BufReader::new(fs::File::open("./inputs/day13.txt").unwrap()).lines();
    let timestamp: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    let depart_times: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, x)| x.parse().ok().map(|x| (i as u64, x)))
        .collect();
    (timestamp, depart_times)
}

fn format_query(schedule: &[(u64, u64)]) -> String {
    let base = schedule[0].1;

    let mut query = String::new();
    query += "solve ";
    for (i, (o, m)) in schedule.iter().enumerate().skip(1) {
        query += &format!(
            "{base}x_1+{o}={m}x_{i},",
            base = base,
            o = o,
            m = m,
            i = i + 1
        );
    }
    query += " over the integers";
    query
}

fn call_wolfram(schedule: &[(u64, u64)]) -> String {
    let appid = std::env::var("WOLFRAM_APPID").unwrap();
    let client = reqwest::blocking::Client::new();
    let response: WolframResponse = client
        .get("https://api.wolframalpha.com/v2/query")
        .query(&[
            ("appid", &appid[..]),
            ("output", "json"),
            ("format", "plaintext"),
            ("includepodid", "ResultOverTheIntegers"),
            ("scanner", "solve"),
            ("input", &format_query(schedule)[..]),
        ])
        .send()
        .unwrap()
        .json()
        .unwrap();
    response.queryresult.pods[0].subpods[0].plaintext.clone()
}

fn extract_result(s: &str) -> u64 {
    s.splitn(3, &['+', '='][..])
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

fn part1() {
    let (timestamp, departs) = read_input();
    let will_depart_at = departs
        .iter()
        .map(|&(_, x)| {
            (
                x,
                (((timestamp as f64) / (x as f64)).ceil() * (x as f64)) as u64,
            )
        })
        .min_by_key(|(_, x)| *x)
        .unwrap();
    let result = (will_depart_at.1 - timestamp) * will_depart_at.0;
    println!("Part 1: {}", result);
}

/// This is a tough one to brute-force. :)
/// However, with some number theory and integer programming help it can be solved rather easily :)
///
/// You can present the problem as follows:
/// Let `i` be the number of bus, starting at 0.
/// Given `d_i` be the ID of the bus `i`
/// Given `t_i` be the time offset for bus `i` (i.e. `t_0 = 0`)
/// We can define `x_i` as being the number of iteration (i.e. how many times it was at the
/// sea port minus 1, i.e. if it is the third time that bus `i` starts the ride at port, `x_i = 2`)
/// of bus `i`. This means that `d_i * x_i` is the timestamp when the `x_i`th iteration of bus `i`
/// starts. `x_i` are our unknowns.
/// We can then represent the task condition as a series of equations like:
///   for every bus `i`, d_0 * x_0 + t_i = d_i * x_i
/// This is roughly the same as saying that "Bus `i` ID `d_i` departs `t_i` minutes after timestamp
/// `t`" for all the buses in the task input, since `t = d_0 * x_0` for some iteration `x_0` (for
/// bus 0).
/// Since we know all possible `i`s (and we know it is a finite set :) ), we have `i` equations (we
/// can skip the first one, `d_0 * x_0 + t_0 = d_0 * x_0` as it is trivial). These are diophantine
/// equations to be precise, and we can be pretty sure that they have recurrent solution (of the
/// form `x_i = an + b` but I'm not going to prove that :P) but that is of no interest for now.
///
/// Having these equations, we need to solve them for any of the `x_i` (we need to know only `x_0`
/// but we can trivially get it from any other). Solving a system of diophantine equations is
/// relatively easy (https://en.wikipedia.org/wiki/Diophantine_equation#System_of_linear_Diophantine_equations),
/// but I don't really want to implement (stable & correct) matrix operations here. And I also
/// wasn't able to find any for Rust. We could represent it as a linear programming problem
/// (minimize `x_0` knowing the above constraints) but most of the Rust solvers don't specialize in
/// integer solutions as far as I know.
/// But we do have one solver, available quite freely - WolframAlpha - and we can use it here,
/// making the solution only a single HTTP request away. :)

fn part2() {
    let (_, schedule) = read_input();
    let response = call_wolfram(&schedule);
    let result = extract_result(&response[..]);
    println!("Part 2: {}", result * schedule[0].1);
}

fn main() {
    part1();
    part2();
}
