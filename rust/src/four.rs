pub fn solve1(input: Vec<String>) -> u64 {
    solve(input.into_iter(), total_covers)
}

pub fn solve2(input: Vec<String>) -> u64 {
    solve(input.into_iter(), partial_covers)
}

fn solve<F>(input: impl Iterator<Item = String>, covers: F) -> u64
where
    F: Fn(&(u64, u64), &(u64, u64)) -> bool,
{
    input
        .into_iter()
        .map(parse)
        .filter(|(elf1, elf2)| covers(elf1, elf2) || covers(elf2, elf1))
        .count() as _
}

fn parse(s: String) -> ((u64, u64), (u64, u64)) {
    s.split_once(',')
        .map(|(a, b)| (range(a), range(b)))
        .unwrap()
}

fn range(s: &str) -> (u64, u64) {
    s.split_once('-')
        .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
        .unwrap()
}

fn total_covers(elf1: &(u64, u64), elf2: &(u64, u64)) -> bool {
    elf1.0 <= elf2.0 && elf1.1 >= elf2.1
}

fn partial_covers(elf1: &(u64, u64), elf2: &(u64, u64)) -> bool {
    let (elf1, elf2) = (elf1.0..=elf1.1, elf2.0..=elf2.1);
    elf1.into_iter().any(|n| elf2.contains(&n))
}
