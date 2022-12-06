use std::collections::HashSet;

fn solve(b: &[u8], marker: usize) -> usize {
    let mut set = HashSet::with_capacity(marker);
    for window in 0..b.len() - marker {
        let n = window + marker;
        let seq = &b[window..n];
        set.extend(seq.iter().copied());
        if set.len() == marker {
            return n;
        }
        set.clear();
    }
    return 0;
}

pub fn solve1(input: Vec<String>) -> usize {
    solve(input[0].as_bytes(), 4)
}

pub fn solve2(input: Vec<String>) -> usize {
    solve(input[0].as_bytes(), 14)
}
