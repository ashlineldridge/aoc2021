use std::io::{self, Read};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let values = read_values(&input)?;

    part1(&values);
    part2(&values);

    Ok(())
}

fn part1(values: &[u32]) {
    let min_cost = min_cost(values, simple_cost);
    println!("Part 1 answer: {}", min_cost);
}

fn part2(values: &[u32]) {
    let min_cost = min_cost(values, triangular_cost);
    println!("Part 2 answer: {}", min_cost);
}

fn min_cost<F>(values: &[u32], f: F) -> u32
where
    F: Fn(u32, &[u32]) -> u32,
{
    let min = *values.iter().min().unwrap();
    let max = *values.iter().max().unwrap();

    let mut min_cost = u32::MAX;
    for v in min..=max {
        let cost = f(v, values);
        if cost < min_cost {
            min_cost = cost;
        }
    }

    min_cost
}

fn simple_cost(value: u32, values: &[u32]) -> u32 {
    values
        .iter()
        .fold(0, |acc, x| acc + (*x as i32 - value as i32).abs()) as u32
}

// See: https://en.wikipedia.org/wiki/Triangular_number
fn triangular_cost(value: u32, values: &[u32]) -> u32 {
    values.iter().fold(0, |acc, x| {
        let n = (*x as i32 - value as i32).abs();
        acc + (n * (n + 1) / 2)
    }) as u32
}

fn read_values(input: &str) -> Result<Vec<u32>> {
    input
        .trim()
        .split(',')
        .map(|v| v.parse().context("bad input"))
        .collect()
}
