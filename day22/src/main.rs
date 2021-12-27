use anyhow::{anyhow, bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    io::{self, Read},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let bounds = Cuboid::new(Point::new(-50, -50, -50), Point::new(50, 50, 50))?;
    let steps = read_steps(input)?;
    let steps = steps
        .into_iter()
        .filter(|s| bounds.contains(&s.cuboid))
        .collect::<Vec<_>>();
    let cuboids = Step::run_all(&steps);
    let volume: usize = cuboids.iter().map(|c| c.volume()).sum();

    println!("Part 1 answer: {}", volume);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let steps = read_steps(input)?;
    let cuboids = Step::run_all(&steps);
    let volume: usize = cuboids.iter().map(|c| c.volume()).sum();

    println!("Part 2 answer: {}", volume);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cuboid {
    bottom_left: Point,
    top_right: Point,
}

impl Cuboid {
    fn new(bottom_left: Point, top_right: Point) -> Result<Self> {
        if top_right.x >= bottom_left.x
            && top_right.y >= bottom_left.y
            && top_right.z >= bottom_left.z
        {
            Ok(Self {
                bottom_left,
                top_right,
            })
        } else {
            bail!("invalid cuboid dimensions");
        }
    }

    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let bottom_left = Point::new(
            self.bottom_left.x.max(other.bottom_left.x),
            self.bottom_left.y.max(other.bottom_left.y),
            self.bottom_left.z.max(other.bottom_left.z),
        );
        let top_right = Point::new(
            self.top_right.x.min(other.top_right.x),
            self.top_right.y.min(other.top_right.y),
            self.top_right.z.min(other.top_right.z),
        );

        Cuboid::new(bottom_left, top_right).ok()
    }

    fn subtract(&self, other: &Cuboid) -> Vec<Cuboid> {
        let mut remains = vec![];

        if other.bottom_left.x > self.bottom_left.x {
            remains.push(
                Cuboid::new(
                    self.bottom_left,
                    Point::new(other.bottom_left.x - 1, self.top_right.y, self.top_right.z),
                )
                .unwrap(),
            );
        }

        if other.top_right.x < self.top_right.x {
            remains.push(
                Cuboid::new(
                    Point::new(
                        other.top_right.x + 1,
                        self.bottom_left.y,
                        self.bottom_left.z,
                    ),
                    self.top_right,
                )
                .unwrap(),
            )
        }

        if other.top_right.y < self.top_right.y {
            remains.push(
                Cuboid::new(
                    Point::new(
                        other.bottom_left.x,
                        other.top_right.y + 1,
                        self.bottom_left.z,
                    ),
                    Point::new(other.top_right.x, self.top_right.y, self.top_right.z),
                )
                .unwrap(),
            );
        }

        if other.bottom_left.y > self.bottom_left.y {
            remains.push(
                Cuboid::new(
                    Point::new(other.bottom_left.x, self.bottom_left.y, self.bottom_left.z),
                    Point::new(other.top_right.x, other.bottom_left.y - 1, self.top_right.z),
                )
                .unwrap(),
            );
        }

        if other.bottom_left.z > self.bottom_left.z {
            remains.push(
                Cuboid::new(
                    Point::new(other.bottom_left.x, other.bottom_left.y, self.bottom_left.z),
                    Point::new(
                        other.top_right.x,
                        other.top_right.y,
                        other.bottom_left.z - 1,
                    ),
                )
                .unwrap(),
            );
        }

        if other.top_right.z < self.top_right.z {
            remains.push(
                Cuboid::new(
                    Point::new(
                        other.bottom_left.x,
                        other.bottom_left.y,
                        other.top_right.z + 1,
                    ),
                    Point::new(other.top_right.x, other.top_right.y, self.top_right.z),
                )
                .unwrap(),
            );
        }

        remains
    }

    fn contains(&self, other: &Cuboid) -> bool {
        other.subtract(self).is_empty()
    }

    fn volume(&self) -> usize {
        (self.top_right.x - self.bottom_left.x + 1) as usize
            * (self.top_right.y - self.bottom_left.y + 1) as usize
            * (self.top_right.z - self.bottom_left.z + 1) as usize
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Step {
    on: bool,
    cuboid: Cuboid,
}

impl Step {
    fn run_all(steps: &[Step]) -> Vec<Cuboid> {
        let mut cuboids: Vec<Cuboid> = vec![];
        for step in steps {
            if !cuboids.is_empty() {
                let mut new_cuboids = vec![];
                for existing_cuboid in &cuboids {
                    if let Some(overlap) = existing_cuboid.intersection(&step.cuboid) {
                        let mut diff = existing_cuboid.subtract(&overlap);
                        new_cuboids.append(&mut diff);
                    } else {
                        new_cuboids.push(existing_cuboid.clone());
                    }
                }

                cuboids = new_cuboids;
            }

            if step.on {
                cuboids.push(step.cuboid.clone());
            }
        }

        cuboids
    }
}

impl FromStr for Step {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)^
                (?P<i>on|off)\s
                x=(?P<xa>-?\d+)..(?P<xb>-?\d+),
                y=(?P<ya>-?\d+)..(?P<yb>-?\d+),
                z=(?P<za>-?\d+)..(?P<zb>-?\d+)$"
            )
            .unwrap();
        }

        let caps = RE.captures(s).ok_or_else(|| anyhow!("bad input: {}", s))?;

        let on = &caps["i"] == "on";
        let xa = caps["xa"].parse()?;
        let xb = caps["xb"].parse()?;
        let ya = caps["ya"].parse()?;
        let yb = caps["yb"].parse()?;
        let za = caps["za"].parse()?;
        let zb = caps["zb"].parse()?;

        let cuboid = Cuboid::new(Point::new(xa, ya, za), Point::new(xb, yb, zb))?;

        Ok(Step { on, cuboid })
    }
}

fn read_steps(input: &str) -> Result<Vec<Step>> {
    input
        .lines()
        .map(|line| line.parse().context("bad input"))
        .collect()
}
