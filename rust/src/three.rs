use std::{collections::HashSet, hash::Hash};

fn priority(b: u8) -> u64 {
    if b.is_ascii_lowercase() {
        1 + (b - 'a' as u8) as u64
    } else {
        27 + (b - 'A' as u8) as u64
    }
}

fn to_set(b: &[u8]) -> HashSet<u8> {
    b.iter().copied().collect()
}

fn split_halves(s: &String) -> (&[u8], &[u8]) {
    let b = s.as_bytes();
    (&b[..b.len() / 2], &b[b.len() / 2..])
}

pub fn solve1(input: Vec<String>) -> u64 {
    input
        .into_iter()
        .flat_map(|s| {
            let (f, b) = split_halves(&s);
            (&to_set(f) & &to_set(b)).into_iter().map(|c| priority(c))
        })
        .sum::<u64>()
}

pub fn solve2(input: Vec<String>) -> u64 {
    input
        .chunks(3)
        .map(|c| c.into_iter().map(|s| to_set(s.as_bytes())))
        .map(|mut c| (c.next().unwrap(), c.next().unwrap(), c.next().unwrap()))
        .flat_map(|(a, b, c)| (&(&a & &b) & &c).into_iter().map(|c| priority(c)))
        .sum::<u64>()
}
