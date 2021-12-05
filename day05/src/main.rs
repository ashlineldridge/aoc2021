use anyhow::{bail, Context, Result};
use std::{
    collections::HashMap,
    io::{self, Read},
    ops::{Add, AddAssign},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines = read_lines(&input)?;

    part1(&lines);
    part2(&lines);

    Ok(())
}

fn part1(lines: &[Line]) {
    let lines = lines
        .iter()
        .cloned()
        .filter(|line| line.kind != LineKind::Diagonal)
        .collect::<Vec<_>>();
    let grid = Grid::new(&lines);
    let count = grid.vents.values().filter(|v| **v > 1).count();

    println!("Part 1 answer: {}", count);
}

fn part2(lines: &[Line]) {
    let grid = Grid::new(lines);
    let count = grid.vents.values().filter(|v| **v > 1).count();

    println!("Part 2 answer: {}", count);
}

struct Grid {
    vents: HashMap<Point, usize>,
}

impl Grid {
    fn new(lines: &[Line]) -> Grid {
        let mut vents = HashMap::new();
        for line in lines {
            for point in line.iter() {
                let count = vents.entry(point).or_insert(0);
                *count += 1;
            }
        }

        Grid { vents }
    }
}

#[derive(Clone)]
struct Line {
    kind: LineKind,
    from: Point,
    to: Point,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LineKind {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Line {
    fn iter(&self) -> LineIter {
        LineIter {
            line: self.clone(),
            curr: self.from,
            done: false,
        }
    }
}

fn read_lines(input: &str) -> Result<Vec<Line>> {
    input.lines().map(|line| line.parse()).collect()
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once(" -> ").context("bad line")?;

        let from: Point = from.parse()?;
        let to: Point = to.parse()?;

        let kind = if from.y == to.y {
            LineKind::Horizontal
        } else if from.x == to.x {
            LineKind::Vertical
        } else if (to.x - from.x).abs() == (to.y - from.y).abs() {
            LineKind::Diagonal
        } else {
            bail!("invalid line type: {:?} -> {:?}", from, to)
        };

        Ok(Line { kind, from, to })
    }
}

struct LineIter {
    line: Line,
    curr: Point,
    done: bool,
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let curr = self.curr;

        if self.curr == self.line.to {
            self.done = true;
            return Some(curr);
        }

        let x_delta = (self.line.to.x - curr.x).max(-1).min(1);
        let y_delta = (self.line.to.y - curr.y).max(-1).min(1);

        self.curr += Point::new(x_delta, y_delta);

        Some(curr)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").context("bad point")?;

        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
