use anyhow::{anyhow, bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{Debug, Write},
    hash::Hash,
    io::{self, Read},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (paper, instructions) = read_input(&input)?;

    part1(paper.clone(), &instructions)?;
    part2(paper, &instructions)?;

    Ok(())
}

fn part1(mut paper: Paper, instructions: &Instructions) -> Result<()> {
    let fold = instructions.folds.first().context("no folds")?;
    paper.fold_in_place(*fold)?;

    println!("Part 1 answer: {}", paper.points.len());

    Ok(())
}

fn part2(mut paper: Paper, instructions: &Instructions) -> Result<()> {
    for fold in &instructions.folds {
        paper.fold_in_place(*fold)?;
    }

    println!("Part 2 answer:\n\n{:?}", paper);

    Ok(())
}

fn read_input(input: &str) -> Result<(Paper, Instructions)> {
    let (head, tail) = input.split_once("\n\n").context("bad input")?;

    let paper = head.parse()?;
    let instructions = tail.parse()?;

    Ok((paper, instructions))
}

#[derive(Clone)]
struct Paper {
    points: HashSet<Point>,
    width: u32,
    height: u32,
}

impl Paper {
    fn fold_in_place(&mut self, point: Point) -> Result<()> {
        let paper = self.fold(point)?;
        *self = paper;

        Ok(())
    }

    fn fold(&self, point: Point) -> Result<Paper> {
        let mut paper = Paper {
            points: HashSet::new(),
            width: 0,
            height: 0,
        };

        match (point.x, point.y) {
            (0, y) => {
                paper.width = self.width;
                for p in &self.points {
                    if let Some(p) = p.fold_y(y, self.height) {
                        paper.points.insert(p);
                        paper.height = paper.height.max(p.y + 1);
                    }
                }
            }
            (x, 0) => {
                paper.height = self.height;
                for p in &self.points {
                    if let Some(p) = p.fold_x(x, self.width) {
                        paper.points.insert(p);
                        paper.width = paper.width.max(p.x + 1);
                    }
                }
            }
            (x, y) => bail!("invalid fold: {},{}", x, y),
        };

        Ok(paper)
    }
}

impl FromStr for Paper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut paper = Paper {
            points: HashSet::new(),
            width: 0,
            height: 0,
        };

        for line in s.lines() {
            let point = line.parse()?;
            paper.points.insert(point);
            paper.width = paper.width.max(point.x + 1);
            paper.height = paper.height.max(point.y + 1);
        }

        Ok(paper)
    }
}

impl Debug for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let ch = match self.points.get(&Point::new(x, y)) {
                    Some(_) => '#',
                    _ => '.',
                };

                f.write_char(ch)?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn fold(v: u32, index: u32, length: u32) -> Option<u32> {
        assert!(index < length);
        let s = (length as i32 - 2 * index as i32 - 1).max(0) as u32;
        match v.cmp(&index) {
            Ordering::Equal => None,
            Ordering::Less => Some(v + s),
            Ordering::Greater => Some(2 * index + s - v),
        }
    }

    fn fold_y(&self, index: u32, height: u32) -> Option<Self> {
        Self::fold(self.y, index, height).map(|y| Point::new(self.x, y))
    }

    fn fold_x(&self, index: u32, width: u32) -> Option<Self> {
        Self::fold(self.x, index, width).map(|x| Point::new(x, self.y))
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').context(format!("bad point: {}", s))?;

        Ok(Point::new(x.parse()?, y.parse()?))
    }
}

struct Instructions {
    folds: Vec<Point>,
}

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^fold along (?P<axis>[xy])=(?P<v>\d+)$").unwrap();
        }

        let mut folds = vec![];

        for line in s.lines() {
            let caps = RE
                .captures(line)
                .ok_or_else(|| anyhow!("bad instruction: {}", s))?;

            let value = caps["v"].parse()?;
            let point = if &caps["axis"] == "x" {
                Point::new(value, 0)
            } else {
                Point::new(0, value)
            };

            folds.push(point);
        }

        Ok(Instructions { folds })
    }
}
