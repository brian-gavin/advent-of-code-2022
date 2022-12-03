#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn beats(m: Move) -> Move {
        match m {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
    fn draws(m: Move) -> Move {
        m
    }
    fn loses_to(m: Move) -> Move {
        match m {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        use Move::*;
        match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("bad input"),
        }
    }
}

struct Round {
    mine: Move,
    opponent: Move,
}

impl Round {
    fn score(&self) -> u64 {
        use Move::*;
        match (&self.mine, &self.opponent) {
            (Rock, Rock) => 1 + 3,
            (Rock, Paper) => 1 + 0,
            (Rock, Scissors) => 1 + 6,
            (Paper, Rock) => 2 + 6,
            (Paper, Paper) => 2 + 3,
            (Paper, Scissors) => 2 + 0,
            (Scissors, Rock) => 3 + 0,
            (Scissors, Paper) => 3 + 6,
            (Scissors, Scissors) => 3 + 3,
        }
    }
}

fn parse_rounds<F: Fn(&str, Move) -> Move>(
    input: Vec<String>,
    make_mine: F,
) -> impl Iterator<Item = Round> {
    input.into_iter().map(move |s| {
        let mut s = s.split(' ');
        let opponent = Move::from(s.next().unwrap());
        let mine = make_mine(s.next().unwrap(), opponent);
        Round { mine, opponent }
    })
}

pub fn solve1(input: Vec<String>) -> u64 {
    parse_rounds(input, |s, _| Move::from(s))
        .map(|r| r.score())
        .sum()
}

pub fn solve2(input: Vec<String>) -> u64 {
    parse_rounds(input, |s, opponent| match s {
        "X" => Move::loses_to(opponent),
        "Y" => Move::draws(opponent),
        "Z" => Move::beats(opponent),
        _ => panic!("bad input"),
    })
    .map(|r| r.score())
    .sum()
}
