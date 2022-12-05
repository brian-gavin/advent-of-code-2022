use std::{collections::VecDeque, mem};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Clone, Debug)]
struct Command {
    n: usize,
    from: usize,
    to: usize,
}

#[derive(Clone, Debug, Default)]
struct CrateMachine {
    commands: Vec<Command>,
    stacks: [VecDeque<char>; 9],
}

impl CrateMachine {
    fn execute9000(&mut self) {
        for c in self.commands.iter() {
            for _ in 0..c.n {
                let v = self.stacks[c.from].pop_back().unwrap();
                self.stacks[c.to].push_back(v);
            }
        }
    }

    fn execute9001(&mut self) {
        for c in self.commands.iter() {
            let vs = {
                let from = &mut self.stacks[c.from];
                from.rotate_right(c.n);
                let end = from.split_off(c.n);
                mem::replace(from, end)
            };
            self.stacks[c.to].extend(vs);
        }
    }

    fn tops(&self) -> Vec<char> {
        let mut v = Vec::with_capacity(9);
        for stack in self.stacks.iter() {
            v.push(*stack.back().unwrap());
        }
        v
    }
}

fn parse_machine(input: Vec<String>) -> CrateMachine {
    let mut stacks = <[VecDeque<char>; 9] as Default>::default();
    let mut input = input.into_iter();
    while let Some(mut s) = input.next() {
        if s.is_empty() {
            break;
        }
        s.push(' ');
        for i in 0..9 {
            let offset = i * 4;
            let s = &s[offset..offset + 4].trim_matches(&[' ', '[', ']'][..]);
            let Ok(c) = s.parse::<char>() else { continue };
            if !c.is_alphabetic() {
                continue;
            }
            stacks[i].push_front(c);
        }
    }
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"move (\d+) from (\d) to (\d)").unwrap());
    let commands = input
        .map(|s| {
            let captures = RE.captures(&s).unwrap();
            Command {
                n: captures[1].parse().unwrap(),
                from: captures[2].parse::<usize>().unwrap() - 1,
                to: captures[3].parse::<usize>().unwrap() - 1,
            }
        })
        .collect::<Vec<_>>();
    CrateMachine { commands, stacks }
}

pub fn solve1(input: Vec<String>) -> String {
    let mut machine = parse_machine(input);
    machine.execute9000();
    String::from_iter(machine.tops())
}

pub fn solve2(input: Vec<String>) -> String {
    let mut machine = parse_machine(input);
    machine.execute9001();
    String::from_iter(machine.tops())
}
