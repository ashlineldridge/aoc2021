use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io::{self, Read};
use std::str::FromStr;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let graph: Graph = input.parse()?;

    part1(graph.clone());
    part2(graph);

    Ok(())
}

fn part1(graph: Graph) {
    let top_left = Point::new(0, 0);
    let bot_right = Point::new(graph.width as i32 - 1, graph.height as i32 - 1);

    let cost = graph.path_cost(top_left, bot_right);
    println!("Part 1 answer: {}", cost);
}

fn part2(mut graph: Graph) {
    graph.expand(5);

    let top_left = Point::new(0, 0);
    let bot_right = Point::new(graph.width as i32 - 1, graph.height as i32 - 1);

    let cost = graph.path_cost(top_left, bot_right);
    println!("Part 2 answer: {}", cost);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    // Returns points in range of this point in reading order.
    fn adjacent(&self) -> HashSet<Point> {
        vec![
            Point::new(self.x, self.y - 1), // Above.
            Point::new(self.x - 1, self.y), // Left.
            Point::new(self.x + 1, self.y), // Right.
            Point::new(self.x, self.y + 1), // Below.
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Clone)]
struct Graph {
    costs: HashMap<Point, u8>,
    width: usize,
    height: usize,
}

impl Graph {
    // Uses Dijkstra's algorithm to calculate the shortest path cost between the
    // specified points. See: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm.
    fn path_cost(&self, from: Point, to: Point) -> u32 {
        let mut path_costs = HashMap::new();

        // The shortest path from the starting point to itself is an empty path.
        path_costs.insert(from, 0);

        // Keep track of all the points on the grid that we have not yet visited. Visiting
        // a point means that we have calculated the shortest path to that point and we will
        // not consider it again.
        let mut unvisited = self.costs.keys().cloned().collect::<HashSet<_>>();

        // Set the current point to the starting point and loop until we have calculated the
        // shortest path to every point.
        let mut current = from;
        loop {
            // The neighbors of the current point which are empty.
            let neighbors = current.adjacent();

            // The neighbors of the current point which are empty and unvisited.
            let unvisited_neighbors = neighbors
                .intersection(&unvisited)
                .cloned()
                .collect::<HashSet<_>>();

            // The cost of the current path.
            let current_cost = path_costs[&current];

            // For each unvisited neighbor of the current point check whether the cost of the
            // path to the neighbor that runs through the current point is less than any previously
            // calculated tentative cost (i.e., the cost of the path that we previously
            // calculated for the neighbor when we last encountered it (or "infinity" / u32::MAX
            // if we have not encountered the neighbor before)). If the new cost is less than
            // the old one, record the new cost as the tentative smallest cost for the neighbor.
            for neighbor in &unvisited_neighbors {
                // The cost from the starting position to the neighbor through the current point.
                let neighbor_cost = current_cost + self.costs[neighbor] as u32;

                // Any previously calculated cost for the neighbor or MAX.
                let existing_neighbor_cost = *path_costs.get(neighbor).unwrap_or(&u32::MAX);

                if neighbor_cost < existing_neighbor_cost {
                    path_costs.insert(*neighbor, neighbor_cost);
                }
            }

            // Consider the current point to be "visited". The shortest path recorded for this
            // point is now final.
            unvisited.remove(&current);

            // Dijkstra's algorithm says to set the current point to the cheapest next point that
            // has been "evalulated" but that has not yet been visited.
            let mut cheapest_option = None;
            for (point, cost) in &path_costs {
                if unvisited.contains(point) {
                    cheapest_option = match cheapest_option {
                        None => Some((*point, *cost)),
                        Some((_, c)) if *cost < c => Some((*point, *cost)),
                        o => o,
                    }
                }
            }

            // Move to the next cheapest point or exit if we are done.
            if let Some((cheapest_point, _)) = cheapest_option {
                current = cheapest_point;
            } else {
                break;
            }
        }

        path_costs[&to]
    }

    fn expand(&mut self, factor: u8) {
        let mut new_costs = HashMap::new();
        for yi in 0..factor {
            for xi in 0..factor {
                for (p, c) in &self.costs {
                    let p = Point::new(
                        p.x + xi as i32 * self.width as i32,
                        p.y + yi as i32 * self.height as i32,
                    );

                    let new_cost = match (xi, yi) {
                        (0, 0) => *c,
                        (_, 0) => {
                            let left = Point::new(p.x - self.width as i32, p.y);
                            (1_u8).max((new_costs[&left] + 1) % 10)
                        }
                        _ => {
                            let above = Point::new(p.x, p.y - self.height as i32);
                            (1_u8).max((new_costs[&above] + 1) % 10)
                        }
                    };

                    new_costs.insert(p, new_cost);
                }
            }
        }

        self.costs = new_costs;
        self.width *= factor as usize;
        self.height *= factor as usize;
    }
}

impl FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut risks = HashMap::new();
        let mut x_max = 0;
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let risk = ch.to_string().parse()?;
                risks.insert(Point::new(x as i32, y as i32), risk);
                x_max = x_max.max(x);
            }
        }

        Ok(Graph {
            costs: risks,
            width: x_max + 1,
            height: s.lines().count(),
        })
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                buf += self.costs[&Point::new(x, y)].to_string().as_str();
            }
            buf += "\n";
        }

        f.write_str(buf.as_str())?;

        Ok(())
    }
}
