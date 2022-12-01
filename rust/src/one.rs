use std::collections::BinaryHeap;

pub fn solve1(input: Vec<String>) -> u64 {
    let mut elves = make_elves(input);
    elves.pop().unwrap()
}

fn make_elves(input: Vec<String>) -> BinaryHeap<u64> {
    let (h, _) = input.into_iter().map(|s| s.parse::<u64>().ok()).fold(
        (BinaryHeap::new(), 0u64),
        |(mut h, a), n| match n {
            None => {
                h.push(a);
                (h, 0)
            }
            Some(n) => (h, a + n),
        },
    );
    h
}

pub fn solve2(input: Vec<String>) -> u64 {
    let mut elves = make_elves(input);
    let mut sum = 0;
    for _ in 0..3 {
        sum += elves.pop().unwrap();
    }
    sum
}
