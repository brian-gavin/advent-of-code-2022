use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_commands(input: impl Iterator<Item = String>) -> impl Iterator<Item = Direction> {
    use Direction::*;
    input.flat_map(|s| {
        let (c, n) = s.split_once(' ').unwrap();
        let (n, c) = (
            n.parse().unwrap(),
            match c {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                _ => unreachable!(),
            },
        );
        (0..n).map(move |_| c)
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point(isize, isize);

fn move_tail(h: &Point, t: Point, part2: bool) -> Point {
    let (Point(hx, hy), Point(tx, ty)) = (h, t);
    let (xd, yd) = (hx - tx, hy - ty);
    match (xd, yd) {
        (-2, 0) => Point(tx - 1, ty),
        (-2, 1) => Point(tx - 1, ty + 1),
        (-2, -1) => Point(tx - 1, ty - 1),
        (-1, -2) => Point(tx - 1, ty - 1),
        (-1, 2) => Point(tx - 1, ty + 1),
        (0, 2) => Point(tx, ty + 1),
        (0, -2) => Point(tx, ty - 1),
        (1, 2) => Point(tx + 1, ty + 1),
        (1, -2) => Point(tx + 1, ty - 1),
        (2, 0) => Point(tx + 1, ty),
        (2, 1) => Point(tx + 1, ty + 1),
        (2, -1) => Point(tx + 1, ty - 1),
        (-1, 1) | (0, 1) | (1, 1) | (-1, 0) | (0, 0) | (1, 0) | (-1, -1) | (0, -1) | (1, -1) => {
            Point(tx, ty)
        }
        (-2, -2) if part2 => Point(tx - 1, ty - 1),
        (-2, 2) if part2 => Point(tx - 1, ty + 1),
        (2, -2) if part2 => Point(tx + 1, ty - 1),
        (2, 2) if part2 => Point(tx + 1, ty + 1),
        p => unreachable!("{:?}", p),
    }
}

fn move_head(cmd: Direction, h: Point) -> Point {
    let Point(x, y) = h;
    match cmd {
        Direction::Up => Point(x, y + 1),
        Direction::Down => Point(x, y - 1),
        Direction::Left => Point(x - 1, y),
        Direction::Right => Point(x + 1, y),
    }
}

fn solve<const N: usize>(input: impl Iterator<Item = String>) -> usize {
    let (visited, ..) = parse_commands(input).fold(
        (HashSet::new(), [Point::default(); N]),
        |(mut visited, mut knots), dir| {
            knots[0] = move_head(dir, knots[0]);
            for i in 1..N {
                knots[i] = move_tail(&knots[i - 1], knots[i], N == 10);
            }
            visited.insert(knots[N - 1]);
            (visited, knots)
        },
    );
    visited.len()
}

pub fn solve1(input: Vec<String>) -> usize {
    solve::<2>(input.into_iter())
}

pub fn solve2(input: Vec<String>) -> usize {
    solve::<10>(input.into_iter())
}
