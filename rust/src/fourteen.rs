use arr_macro::arr;
use std::{
    cell::{RefCell, RefMut},
    cmp::{max, min},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridState {
    Air,
    Rock,
    Sand,
}

const SAMPLE: bool = false;
const INITIAL_X: usize = 522;
const MAX_Y: usize = if SAMPLE { 9 + 2 + 1 } else { 162 + 2 + 1 };
const SAND_ORIGIN: (usize, usize) = (500, 0);

#[derive(Debug)]
struct Grid([RefCell<Vec<GridState>>; MAX_Y]);

impl Grid {
    fn new() -> Self {
        Grid(arr![RefCell::new({
            let mut v = Vec::with_capacity(1024);
            v.resize(INITIAL_X, GridState::Air);
            v
        }); 165])
    }

    fn check_grow(&self, x: usize) {
        if self.0[0].borrow().len() > x {
            return;
        }
        for (i, v) in self.0.iter().enumerate() {
            let len = v.borrow().len();
            v.borrow_mut().resize_with(len + 1, || {
                if i == MAX_Y - 1 {
                    GridState::Rock
                } else {
                    GridState::Air
                }
            });
        }
    }

    fn at(&self, (x, y): (usize, usize)) -> GridState {
        self.check_grow(x);
        self.0[y].borrow()[x]
    }

    fn at_mut(&mut self, (x, y): (usize, usize)) -> RefMut<'_, GridState> {
        self.check_grow(x);
        RefMut::map(self.0[y].borrow_mut(), |v| &mut v[x])
    }

    fn x(&self) -> usize {
        self.0[0].borrow().len()
    }

    fn y(&self) -> usize {
        self.0.len()
    }
}

// (minX,maxX), (minY,maxY):(462, 517),(13, 162)
fn parse_input(input: impl Iterator<Item = String>) -> Grid {
    let mut grid = Grid::new();
    input.for_each(|s| {
        s.split(" -> ")
            .map(|s| {
                let (x, y) = s.split_once(',').unwrap();
                let parse = |s: &str| s.parse::<usize>().unwrap();
                let (x, y) = (parse(x), parse(y));
                (x, y)
            })
            .reduce(|(ax, ay), (bx, by)| {
                if ax == bx {
                    let x = ax;
                    for y in min(ay, by)..=max(ay, by) {
                        *grid.at_mut((x, y)) = GridState::Rock;
                    }
                } else if ay == by {
                    let y = ay;
                    for x in min(ax, bx)..=max(ax, bx) {
                        *grid.at_mut((x, y)) = GridState::Rock;
                    }
                } else {
                    panic!("bad line coords: {:?} {:?}", (ax, ay), (bx, by));
                }
                (bx, by)
            });
    });
    for x in 0..INITIAL_X {
        *grid.at_mut((x, MAX_Y - 1)) = GridState::Rock; // draw floor
    }
    grid
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    use GridState::*;
    for x in 0..grid.x() {
        for y in 0..grid.y() {
            print!(
                "{}",
                if (x, y) == (500, 0) {
                    '+'
                } else {
                    match grid.at((x, y)) {
                        Air => '.',
                        Rock => '#',
                        Sand => 'o',
                    }
                }
            )
        }
        println!("")
    }
}

fn try_move_sand(grid: &Grid, (x, y): (usize, usize)) -> Option<(usize, usize)> {
    [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
        .into_iter()
        .find(|coord| grid.at(*coord) == GridState::Air)
}

fn draw_sand<F: Fn((usize, usize)) -> bool>(grid: &mut Grid, stop: F) -> usize {
    let mut sands = 0;
    'outer: loop {
        let mut sandpos = SAND_ORIGIN;
        while let Some(new_sandpos) = try_move_sand(&grid, sandpos) {
            sandpos = new_sandpos;
        }
        sands += 1;
        *grid.at_mut(sandpos) = GridState::Sand;
        if stop(sandpos) {
            break 'outer;
        }
    }
    sands
}

pub fn solve1(input: Vec<String>) -> usize {
    let mut grid = parse_input(input.into_iter());
    let n_sand = draw_sand(&mut grid, |(_, sand_y)| sand_y == MAX_Y - 2);
    n_sand - 1
}

pub fn solve2(input: Vec<String>) -> usize {
    let mut grid = parse_input(input.into_iter());
    let n_sand = draw_sand(&mut grid, |sandpos| sandpos == SAND_ORIGIN);
    n_sand
}
