use std::{
    cmp::Ordering,
    iter::{self, Peekable},
    str::Chars,
};

#[derive(Debug, Eq, PartialEq, Ord, Clone)]
enum PacketElement {
    Int(u64),
    List(Vec<PacketElement>),
}

#[derive(PartialEq, Eq, Ord, Clone)]
struct Packet(Vec<PacketElement>);

fn parse_int(s: &mut Peekable<Chars>) -> PacketElement {
    let mut i = String::new();
    while let Some(c) = s.peek() {
        if !c.is_digit(10) {
            break;
        }
        i.push(s.next().unwrap());
    }
    PacketElement::Int(i.parse().unwrap())
}

fn parse_packet(s: &mut Peekable<Chars>) -> Packet {
    let mut v = vec![];
    let Some('[') = s.next() else { panic!("bad input") };
    while let Some(c) = s.peek() {
        let e = match c {
            '0'..='9' => parse_int(s),
            '[' => PacketElement::List(parse_packet(s).0),
            ']' => {
                let _ = s.next();
                break;
            }
            ',' => {
                let _ = s.next();
                continue;
            }
            _ => panic!("bad input"),
        };
        v.push(e);
    }
    Packet(v)
}

fn parse_packets(input: impl Iterator<Item = String>) -> impl Iterator<Item = Packet> {
    input.filter_map(|s| {
        let s = s;
        if s == "" {
            None
        } else {
            Some(parse_packet(&mut s.chars().peekable()))
        }
    })
}

fn slice_cmp(l: &[PacketElement], r: &[PacketElement]) -> Ordering {
    use PacketElement::*;
    let (mut l, mut r) = (l.iter(), r.iter());
    loop {
        match (l.next(), r.next()) {
            (Some(Int(a)), Some(Int(ref b))) => match a.cmp(b) {
                Ordering::Equal => (),
                ord => return ord,
            },
            (Some(List(l)), Some(List(r))) => match slice_cmp(l, r) {
                Ordering::Equal => (),
                ord => return ord,
            },
            (Some(List(l)), Some(Int(r))) => match slice_cmp(l, &vec![Int(*r)]) {
                Ordering::Equal => (),
                ord => return ord,
            },
            (Some(Int(l)), Some(List(r))) => match slice_cmp(&vec![Int(*l)], r) {
                Ordering::Equal => (),
                ord => return ord,
            },
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
        }
    }
}

impl PartialOrd for PacketElement {
    fn partial_cmp(&self, r: &Self) -> Option<Ordering> {
        use PacketElement::*;
        Some(match (self, r) {
            (Int(a), Int(b)) => a.cmp(b),
            (Int(l), List(r)) => slice_cmp(&[Int(*l)], r),
            (List(l), Int(r)) => slice_cmp(l, &[Int(*r)]),
            (List(l), List(r)) => slice_cmp(l, r),
        })
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(slice_cmp(&self.0, &other.0))
    }
}

pub fn solve1(input: Vec<String>) -> usize {
    let pairs: (Vec<_>, Vec<_>) = parse_packets(input.into_iter())
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);
    iter::zip(
        pairs.0.into_iter().map(|x| x.1),
        pairs.1.into_iter().map(|x| x.1),
    )
    .enumerate()
    .filter(|(_, (l, r))| l <= r)
    .map(|(i, _)| i + 1)
    .sum()
}

fn divider_packets() -> [Packet; 2] {
    use PacketElement::*;
    [
        Packet(vec![List(vec![Int(2)])]),
        Packet(vec![List(vec![Int(6)])]),
    ]
}

pub fn solve2(input: Vec<String>) -> usize {
    let mut p = parse_packets(input.into_iter())
        .chain(divider_packets())
        .collect::<Vec<_>>();
    p.sort();
    let [a, b] = divider_packets();
    p.into_iter()
        .enumerate()
        .filter_map(|(i, p)| if p == a || p == b { Some(i + 1) } else { None })
        .product::<usize>()
}
