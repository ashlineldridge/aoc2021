use anyhow::{Context, Result, anyhow, ensure};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    io::{self, Read},
    ops::RangeInclusive,
    str::FromStr,
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let target: Area = input.parse()?;

    part1(&target)?;
    part2(&target)?;

    Ok(())
}

fn part1(target: &Area) -> Result<()> {
    let mut y_maxes = vec![];

    for xv in 0..=200 {
        for yv in -150..=1000 {
            let mut probe = Probe::new(xv, yv);
            let mut y_max = 0;
            loop {
                let pos = probe.step();

                y_max = y_max.max(pos.y);

                if pos.dist_from(target) == 0 {
                    // We have hit the target.
                    y_maxes.push(y_max);
                    break;
                }

                if pos.is_beyond(target) {
                    // We have missed the target.
                    break;
                }
            }
        }
    }

    let y_max = y_maxes.iter().max().context("invalid area")?;
    println!("Part 1 answer: {}", y_max);

    Ok(())
}

fn part2(target: &Area) -> Result<()> {
    let mut total_hits = 0;

    for xv in 0..=200 {
        for yv in -150..=1000 {
            let mut probe = Probe::new(xv, yv);
            loop {
                let pos = probe.step();
                if pos.dist_from(target) == 0 {
                    // We have hit the target.
                    total_hits += 1;
                    break;
                }

                if pos.is_beyond(target) {
                    // We have missed the target.
                    break;
                }
            }
        }
    }

    println!("Part 2 answer: {}", total_hits);

    Ok(())
}

struct Probe {
    pos: Position,
    xv: i32,
    yv: i32,
}

impl Probe {
    fn new(xv: i32, yv: i32) -> Self {
        Self {
            pos: Position::origin(),
            xv,
            yv,
        }
    }

    fn step(&mut self) -> Position {
        self.pos.x += self.xv;
        self.pos.y += self.yv;

        self.xv -= self.xv.signum();
        self.yv -= 1;

        self.pos
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn origin() -> Self {
        Self::new(0, 0)
    }

    fn dist_from(&self, area: &Area) -> u32 {
        fn rd(p: i32, rs: i32, re: i32) -> i32 {
            if p < rs {
                (rs - p).abs()
            } else if p > re {
                (p - re).abs()
            } else {
                0
            }
        }

        let xd = rd(self.x, *area.xr.start(), *area.xr.end()) as f32;
        let yd = rd(self.y, *area.yr.start(), *area.yr.end()) as f32;

        (xd.powi(2) + yd.powi(2)).sqrt().round() as u32
    }

    fn is_beyond(&self, area: &Area) -> bool {
        (area.xr.end() >= &0 && &self.x > area.xr.end())
            || (area.xr.start() < &0 && &self.x < area.xr.start())
            || (&self.y < area.yr.start())
    }
}

#[derive(Debug)]
struct Area {
    xr: RangeInclusive<i32>,
    yr: RangeInclusive<i32>,
}

impl FromStr for Area {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)^
                target\ area:\s+
                x=(?P<xa>-?\d+)..(?P<xb>-?\d+),\s+
                y=(?P<ya>-?\d+)..(?P<yb>-?\d+)$"
            )
            .unwrap();
        }

        let s = s.trim();
        let caps = RE.captures(s).ok_or_else(|| anyhow!("bad input: {}", s))?;

        let xrs = caps["xa"].parse()?;
        let xre = caps["xb"].parse()?;
        let yrs = caps["ya"].parse()?;
        let yre = caps["yb"].parse()?;

        ensure!(xrs < xre && yrs < yre, "invalid area dimensions");

        Ok(Area { xr: xrs..=xre, yr: yrs..=yre })
    }
}
