use std::collections::{BinaryHeap, VecDeque};

struct Monkey {
    items: VecDeque<u128>,
    op: Box<dyn Fn(u128) -> u128>,
    test: Box<dyn Fn(u128) -> bool>,
    throw_true: usize,
    throw_false: usize,
}

type Barrel<T> = Vec<T>;

fn divisible_by(n: u128) -> Box<dyn Fn(u128) -> bool> {
    Box::new(move |x| x % n == 0)
}

fn parse_barrel(input: impl Iterator<Item = String>) -> Barrel<Monkey> {
    let _ = input;
    vec![
        Monkey {
            items: VecDeque::from([61]),
            op: Box::new(|x| x * 11),
            test: divisible_by(5),
            throw_true: 7,
            throw_false: 4,
        },
        Monkey {
            items: VecDeque::from([76, 92, 53, 93, 79, 86, 81]),
            op: Box::new(|x| x + 4),
            test: divisible_by(2),
            throw_true: 2,
            throw_false: 6,
        },
        Monkey {
            items: VecDeque::from([91, 99]),
            op: Box::new(|x| x * 19),
            test: divisible_by(13),
            throw_true: 5,
            throw_false: 0,
        },
        Monkey {
            items: VecDeque::from([58, 67, 66]),
            op: Box::new(|x| x * x),
            test: divisible_by(7),
            throw_true: 6,
            throw_false: 1,
        },
        Monkey {
            items: VecDeque::from([94, 54, 62, 73]),
            op: Box::new(|x| x + 1),
            test: divisible_by(19),
            throw_true: 3,
            throw_false: 7,
        },
        Monkey {
            items: VecDeque::from([59, 95, 51, 58, 58]),
            op: Box::new(|x| x + 3),
            test: divisible_by(11),
            throw_true: 0,
            throw_false: 4,
        },
        Monkey {
            items: VecDeque::from([87, 69, 92, 56, 91, 93, 88, 73]),
            op: Box::new(|x| x + 8),
            test: divisible_by(3),
            throw_true: 5,
            throw_false: 2,
        },
        Monkey {
            items: VecDeque::from([71, 57, 86, 67, 96, 95]),
            op: Box::new(|x| x + 7),
            test: divisible_by(17),
            throw_true: 3,
            throw_false: 1,
        },
    ]
}

pub fn solve1(input: Vec<String>) -> usize {
    let mut monkies = parse_barrel(input.into_iter());
    let mut inspections = [0usize; 8];
    for _ in 0..20 {
        for i in 0..monkies.len() {
            while let Some(item) = monkies[i].items.pop_front() {
                inspections[i] += 1;
                let level = (monkies[i].op)(item) / 3;
                let throw_to = if (monkies[i].test)(level) {
                    monkies[i].throw_true
                } else {
                    monkies[i].throw_false
                };
                monkies[throw_to].items.push_back(level);
            }
        }
    }
    let mut inspections = inspections.into_iter().collect::<BinaryHeap<_>>();
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

pub fn solve2(input: Vec<String>) -> u128 {
    let mut monkies = parse_barrel(input.into_iter());
    let mut inspections = [0; 8];
    for _ in 0..10_000 {
        for i in 0..monkies.len() {
            while let Some(item) = monkies[i].items.pop_front() {
                inspections[i] += 1;
                let level = (monkies[i].op)(item);
                let throw_to = if (monkies[i].test)(level) {
                    monkies[i].throw_true
                } else {
                    monkies[i].throw_false
                };
                monkies[throw_to].items.push_back(level);
            }
        }
    }
    let mut inspections = inspections.into_iter().collect::<BinaryHeap<_>>();
    inspections.pop().unwrap() * inspections.pop().unwrap()
}
