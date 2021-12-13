use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    io::{self, Read},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let graph: Graph = input.parse()?;

    part1(graph.clone());
    part2(graph);

    Ok(())
}

fn part1(mut graph: Graph) {
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += graph.step();
    }

    println!("Part 1 answer: {}", flashes);
}

fn part2(mut graph: Graph) {
    let mut step = 0;
    loop {
        step += 1;
        let flashes = graph.step();
        if flashes == graph.0.len() {
            break;
        }
    }

    println!("Part 2 answer: {}", step);
}

#[derive(Clone)]
struct Cell {
    energy: u8,
    state: CellState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellState {
    Idle,
    Flashing,
}

impl Cell {
    const MAX_ENERGY: u8 = 9;

    fn new(energy: u8) -> Self {
        Self {
            energy,
            state: CellState::Idle,
        }
    }

    fn is_flashing(&self) -> bool {
        self.state == CellState::Flashing
    }

    fn mutate(&mut self) -> CellState {
        match self.state {
            CellState::Flashing => {
                self.state = CellState::Idle;
                self.energy += 1;
            }
            CellState::Idle if self.energy == Self::MAX_ENERGY => {
                self.state = CellState::Flashing;
                self.energy = 0;
            }
            _ => {
                self.energy += 1;
            }
        }

        self.state
    }
}

#[derive(Clone)]
struct Graph(HashMap<Point, Cell>);

impl Graph {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn step(&mut self) -> usize {
        let mut flashes = 0;
        for cell in &mut self.0.values_mut() {
            if cell.mutate() == CellState::Flashing {
                flashes += 1;
            }
        }

        for (point, cell) in self.0.clone() {
            if cell.state == CellState::Flashing {
                flashes += self.proxy_mutate(point);
            }
        }

        flashes
    }

    fn proxy_mutate(&mut self, point: Point) -> usize {
        let mut flashes = 0;
        for point in point.adjacent() {
            match self.0.get_mut(&point) {
                Some(cell) if !cell.is_flashing() => {
                    if cell.mutate() == CellState::Flashing {
                        flashes += 1;
                        flashes += self.proxy_mutate(point);
                    }
                }
                _ => (),
            }
        }

        flashes
    }
}

impl FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Graph::new();
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let v: u8 = ch.to_string().parse()?;
                graph.0.insert(Point::new(x as i32, y as i32), Cell::new(v));
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

    fn adjacent(&self) -> HashSet<Point> {
        vec![
            Self::new(self.x, self.y - 1),     // Above.
            Self::new(self.x + 1, self.y - 1), // Above right.
            Self::new(self.x + 1, self.y),     // Right.
            Self::new(self.x + 1, self.y + 1), // Below right.
            Self::new(self.x, self.y + 1),     // Below.
            Self::new(self.x - 1, self.y + 1), // Below left.
            Self::new(self.x - 1, self.y),     // Left.
            Self::new(self.x - 1, self.y - 1), // Above left.
        ]
        .into_iter()
        .collect()
    }
}
