use std::io::{self, Read};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let depths = parse_depths(&input)?;

    part1(&depths);
    part2(&depths);

    Ok(())
}

fn part1(depths: &[u32]) {
    let total = depth_increases(depths, 1);
    println!("Part 1 answer: {}", total);
}

fn part2(depths: &[u32]) {
    let total = depth_increases(depths, 3);
    println!("Part 2 answer: {}", total);
}

fn depth_increases(depths: &[u32], window_size: usize) -> usize {
    let (_, total) = depths
        .windows(window_size)
        .map(|window| window.iter().sum::<u32>())
        .fold((None, 0), |(prev, total), x| match prev {
            Some(prev) if x > prev => (Some(x), total + 1),
            _ => (Some(x), total),
        });

    total
}

fn parse_depths(input: &str) -> Result<Vec<u32>> {
    input
        .lines()
        .map(|line| line.parse().context("bad input"))
        .collect()
}
