use self::PairElem::*;
use anyhow::{bail, Context, Result};
use core::fmt;
use std::{
    fmt::Display,
    hash::Hash,
    io::{self, Read},
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let pairs = read_pairs(input)?;
    let pair = pairs
        .into_iter()
        .fold(None, |acc, p| match acc {
            None => Some(p),
            Some(acc) => Some(acc.add(&p)),
        })
        .context("no pairs")?;

    println!("Part 1 answer: {}", pair.magnitude());

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let pairs = read_pairs(input)?;

    let mut max_magnitude = 0;
    for p1 in &pairs {
        for p2 in &pairs {
            if p1 != p2 {
                let sum = p1.add(p2);
                let magnitude = sum.magnitude();
                max_magnitude = max_magnitude.max(magnitude);
            }
        }
    }

    println!("Part 2 answer: {}", max_magnitude);

    Ok(())
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pair {
    left: PairElem,
    right: PairElem,
}

impl Pair {
    fn new(left: PairElem, right: PairElem) -> Self {
        Self { left, right }
    }

    fn add(&self, rhs: &Pair) -> Pair {
        let mut result = Self::new(
            Pointer(Box::new(self.clone())),
            Pointer(Box::new(rhs.clone())),
        );

        while result.explode(4).is_some() || result.split().is_some() {}

        result
    }

    fn split(&mut self) -> Option<PairElemValue> {
        let left = Self::split_elem(&mut self.left);
        if left.is_some() {
            left
        } else {
            Self::split_elem(&mut self.right)
        }
    }

    fn split_elem(elem: &mut PairElem) -> Option<PairElemValue> {
        match elem {
            Literal(v) if *v >= 10 => {
                let v = *v;
                let left = v / 2; // Divide by 2 and round down.
                let right = (v - 1) / 2 + 1; // Divide by 2 and round up
                let pair = Pair::new(Literal(left), Literal(right));
                *elem = Pointer(Box::new(pair));
                Some(v)
            }
            Pointer(p) => p.split(),
            _ => None,
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(PairElemValue, PairElemValue)> {
        let left = match Self::explode_elem(&mut self.left, depth) {
            Some((left, right)) => {
                match &mut self.right {
                    Literal(v) => *v += right,
                    Pointer(p) => p.add_left(right),
                }
                Some((left, 0))
            }
            _ => None,
        };

        if left.is_some() {
            return left;
        }

        match Self::explode_elem(&mut self.right, depth) {
            Some((left, right)) => {
                match &mut self.left {
                    Literal(v) => *v += left,
                    Pointer(p) => p.add_right(left),
                }
                Some((0, right))
            }
            _ => None,
        }
    }

    fn explode_elem(elem: &mut PairElem, depth: usize) -> Option<(PairElemValue, PairElemValue)> {
        if let Pointer(pair) = elem {
            let depth = depth - 1;
            if depth == 0 {
                if let (Literal(left), Literal(right)) = (&pair.left, &pair.right) {
                    let (left, right) = (*left, *right);
                    *elem = Literal(0);
                    return Some((left, right));
                }
            } else {
                return pair.explode(depth);
            }
        }

        None
    }

    fn add_left(&mut self, value: PairElemValue) {
        match &mut self.left {
            Literal(v) => *v += value,
            Pointer(p) => p.add_left(value),
        }
    }

    fn add_right(&mut self, value: PairElemValue) {
        match &mut self.right {
            Literal(v) => *v += value,
            Pointer(p) => p.add_right(value),
        }
    }

    fn magnitude(&self) -> PairElemValue {
        let left = match &self.left {
            Literal(v) => 3 * v,
            Pointer(p) => 3 * p.magnitude(),
        };

        let right = match &self.right {
            Literal(v) => 2 * v,
            Pointer(p) => 2 * p.magnitude(),
        };

        left + right
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("[{},{}]", self.left, self.right))
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum PairElem {
    Literal(PairElemValue),
    Pointer(Box<Pair>),
}

type PairElemValue = u32;

impl PairElem {
    fn pointer(self) -> Option<Box<Pair>> {
        match self {
            Pointer(p) => Some(p),
            _ => None,
        }
    }
}

impl Display for PairElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal(v) => fmt::Display::fmt(v, f),
            Pointer(p) => fmt::Display::fmt(p.as_ref(), f),
        }
    }
}

fn read_pairs(input: &str) -> Result<Vec<Pair>> {
    input
        .lines()
        .map(|line| read_pair(line).map(|p| *p))
        .collect::<Result<Vec<_>>>()
}

fn read_pair(line: &str) -> Result<Box<Pair>> {
    fn build(s: &str) -> Result<PairElem> {
        if let Ok(v) = s.parse::<PairElemValue>() {
            return Ok(Literal(v));
        }

        let s = s.strip_prefix('[').context("invalid pair")?;
        let s = s.strip_suffix(']').context("invalid pair")?;

        let mut opens = 0;
        let mut closes = 0;
        let mut split = None;
        for (i, ch) in s.chars().enumerate() {
            match ch {
                '[' => opens += 1,
                ']' => closes += 1,
                ',' if opens == closes => {
                    split = Some((&s[0..i], &s[i + 1..]));
                    break;
                }
                _ => (),
            }
        }

        let elem = if let Some((left, right)) = split {
            Pointer(Box::new(Pair::new(build(left)?, build(right)?)))
        } else {
            bail!("invalid pair")
        };

        Ok(elem)
    }

    let elem = build(line)?;
    let pair = elem.pointer().context("invalid pair")?;

    Ok(pair)
}
