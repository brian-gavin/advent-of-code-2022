#![allow(dead_code)]

use once_cell::sync::Lazy;
use regex::Regex;

type Pair = (isize, isize);

fn parse_input(input: impl Iterator<Item = String>) -> impl Iterator<Item = (Pair, Pair)> {
    const RE_STR: &str =
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)";
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(RE_STR).unwrap());
    input.map(|s| {
        let cap = RE.captures(&s).unwrap();
        let parse = |n| cap.get(n).unwrap().as_str().parse::<isize>().unwrap();
        ((parse(1), parse(2)), (parse(3), parse(4)))
    })
}

fn manhattan_distance((x1, y1): &Pair, (x2, y2): &Pair) -> usize {
    x1.abs_diff(*x2) + y1.abs_diff(*y2)
}

pub fn solve1(_input: Vec<String>) -> usize {
    const _Y: isize = 10;
    todo!("solve)")
}

pub fn solve2(_input: Vec<String>) -> u64 {
    todo!("solve")
}
