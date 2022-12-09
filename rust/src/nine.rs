use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Command {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

fn parse_commands(input: impl Iterator<Item = String>) -> impl Iterator<Item = Command> {
    use Command::*;
    input.map(|s| {
        let (c, n) = s.split_once(' ').unwrap();
        match c {
            "U" => Up(n.parse().unwrap()),
            "D" => Down(n.parse().unwrap()),
            "L" => Left(n.parse().unwrap()),
            "R" => Right(n.parse().unwrap()),
            _ => unreachable!(),
        }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point(isize, isize);

fn move_up(Point(x, y): Point) -> Point {
    Point(x, y + 1)
}
fn move_down(Point(x, y): Point) -> Point {
    Point(x, y - 1)
}
fn move_left(Point(x, y): Point) -> Point {
    Point(x - 1, y)
}
fn move_right(Point(x, y): Point) -> Point {
    Point(x + 1, y)
}

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

fn unwrap_cmd(cmd: Command) -> (fn(Point) -> Point, usize) {
    match cmd {
        Command::Up(n) => (move_up, n),
        Command::Down(n) => (move_down, n),
        Command::Left(n) => (move_left, n),
        Command::Right(n) => (move_right, n),
    }
}

pub fn solve1(input: Vec<String>) -> usize {
    let (visited, _, _) = parse_commands(input.into_iter()).fold(
        (HashSet::new(), Point::default(), Point::default()),
        |(mut visited, mut h, mut t), cmd| {
            let (move_head, n) = unwrap_cmd(cmd);
            for _ in 0..n {
                h = move_head(h);
                t = move_tail(&h, t, false);
                visited.insert(t);
            }
            (visited, h, t)
        },
    );
    visited.len()
}

pub fn solve2(input: Vec<String>) -> usize {
    let (visited, ..) = parse_commands(input.into_iter()).fold(
        (HashSet::new(), [Point::default(); 10]),
        |(mut visited, mut knots), cmd| {
            let (move_head, n) = unwrap_cmd(cmd);
            for _ in 0..n {
                knots[0] = move_head(knots[0]);
                for i in 1..10 {
                    knots[i] = move_tail(&knots[i - 1], knots[i], true);
                }
                visited.insert(knots[9]);
            }
            (visited, knots)
        },
    );
    visited.len()
}
