use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
    ops::{Deref, DerefMut},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let graph: HeightGraph = input.parse()?;

    part1(&graph);
    part2(&graph);

    Ok(())
}

fn part1(graph: &HeightGraph) {
    let total_risk: u32 = graph.low_points().iter().map(|p| graph[p] as u32 + 1).sum();
    println!("Part 1 answer: {}", total_risk);
}

fn part2(graph: &HeightGraph) {
    let basin_multiple: u32 = graph
        .basins()
        .iter()
        .take(3)
        .fold(1, |acc, b| acc * b.len() as u32);
    println!("Part 2 answer: {}", basin_multiple);
}

struct HeightGraph(HashMap<Point, u8>);

impl HeightGraph {
    const MAX_HEIGHT: u8 = 9;

    fn new() -> Self {
        Self(HashMap::new())
    }

    fn low_points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        for (p, v) in self.iter() {
            let mut adjacent_values = vec![];
            for p in p.adjacent() {
                if let Some(v) = self.get(&p) {
                    adjacent_values.push(*v);
                }
            }

            if !adjacent_values.iter().any(|x| x <= v) {
                points.insert(*p);
            }
        }

        points
    }

    fn basins(&self) -> Vec<HashSet<Point>> {
        let mut basins = vec![];
        for p in self.low_points() {
            let mut basin = HashSet::new();
            self.walk_basin(p, &mut basin);

            basins.push(basin)
        }

        basins.sort_unstable_by_key(|b| -(b.len() as i32));

        basins
    }

    fn walk_basin(&self, point: Point, acc: &mut HashSet<Point>) {
        match self.get(&point) {
            Some(v) if *v < Self::MAX_HEIGHT => acc.insert(point),
            _ => return,
        };

        for p in point.adjacent().difference(&acc.clone()) {
            self.walk_basin(*p, acc);
        }
    }
}

impl Deref for HeightGraph {
    type Target = HashMap<Point, u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HeightGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for HeightGraph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = HeightGraph::new();
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let v: u8 = ch.to_string().parse()?;
                graph.insert(Point::new(x as i32, y as i32), v);
            }
        }

        Ok(graph)
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

    fn above(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn adjacent(&self) -> HashSet<Point> {
        vec![self.above(), self.below(), self.left(), self.right()]
            .into_iter()
            .collect()
    }
}
