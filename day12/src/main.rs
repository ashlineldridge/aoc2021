use anyhow::{Context, Result};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::{self, Read},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map: CaveMap = input.parse()?;

    part1(&map)?;
    part2(&map)?;

    Ok(())
}

fn part1(map: &CaveMap) -> Result<()> {
    let paths = map.paths(0)?;
    println!("Part 1 answer: {}", paths.len());

    Ok(())
}

fn part2(map: &CaveMap) -> Result<()> {
    let paths = map.paths(1)?;
    println!("Part 2 answer: {}", paths.len());
    Ok(())
}

type CaveId = String;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Cave {
    id: CaveId,
    kind: CaveKind,
}

impl Cave {
    const START_ID: &'static str = "start";
    const END_ID: &'static str = "end";

    fn start() -> Self {
        Self {
            id: Self::START_ID.into(),
            kind: CaveKind::Start,
        }
    }
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            id: s.into(),
            kind: s.parse()?,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum CaveKind {
    Start,
    End,
    Big,
    Small,
}

impl FromStr for CaveKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Cave::START_ID => Ok(Self::Start),
            Cave::END_ID => Ok(Self::End),
            s if s.to_lowercase() == s => Ok(Self::Small),
            _ => Ok(Self::Big),
        }
    }
}

#[derive(Clone, Debug)]
struct CaveMap(HashMap<Cave, HashSet<Cave>>);

type CavePath = Vec<Cave>;

impl CaveMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn paths(&self, revisits: usize) -> Result<Vec<CavePath>> {
        let mut paths = vec![];
        let start_path = vec![Cave::start()];
        self.walk_paths(&start_path, &mut paths, revisits)?;

        Ok(paths)
    }

    fn walk_paths(&self, path: &CavePath, acc: &mut Vec<CavePath>, revisits: usize) -> Result<()> {
        let last = path.last().context("cave path is empty")?;
        if let Some(nexts) = self.0.get(last) {
            for next in nexts {
                if next.kind == CaveKind::Start {
                    continue;
                }

                let mut path = path.clone();

                if next.kind == CaveKind::Small {
                    let next_visits = path.iter().filter(|c| *c == next).count();
                    if next_visits > 0 {
                        let visited_small_caves = path
                            .iter()
                            .filter(|c| c.kind == CaveKind::Small)
                            .collect::<Vec<_>>();
                        let total_small_visits = visited_small_caves.len();
                        let unique_small_visits =
                            visited_small_caves.iter().collect::<HashSet<_>>().len();

                        if total_small_visits - unique_small_visits >= revisits {
                            continue;
                        }
                    }
                }

                path.push(next.clone());

                if next.kind == CaveKind::End {
                    acc.push(path);
                    continue;
                }

                self.walk_paths(&path, acc, revisits)?;
            }
        }

        Ok(())
    }
}

impl FromStr for CaveMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave_map = CaveMap::new();
        for line in s.lines() {
            let (from, to) = line.split_once('-').context("bad input")?;
            let from: Cave = from.parse()?;
            let to: Cave = to.parse()?;

            let from_next = cave_map.0.entry(from.clone()).or_default();
            from_next.insert(to.clone());

            let to_next = cave_map.0.entry(to).or_default();
            to_next.insert(from);
        }

        Ok(cave_map)
    }
}
