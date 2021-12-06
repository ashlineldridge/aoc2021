use std::{
    collections::VecDeque,
    io::{self, Read},
    iter,
    str::FromStr,
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let model: FishModel = input.parse()?;

    part1(model.clone());
    part2(model);

    Ok(())
}

fn part1(mut model: FishModel) {
    let final_population = model.run(80);
    println!("Part 1 answer: {}", final_population);
}

fn part2(mut model: FishModel) {
    let final_population = model.run(256);
    println!("Part 2 answer: {}", final_population);
}

const ADULT_RESET: usize = 6;
const CHILD_RESET: usize = 8;

#[derive(Clone)]
struct FishModel {
    // We'll use a circular buffer to organize the fish into bins, where the bin index
    // equals the number of days remaining before the fish in that bin can reproduce.
    bins: VecDeque<usize>,
}

impl FishModel {
    fn run(&mut self, days: usize) -> usize {
        for _ in 0..days {
            // Pop the zero bin and add it to the "reset bin". This is safe to unwrap since
            // we control the number of bins. This has the effect of rotating the buffer
            // forward and decrementing the fish's reproduction timer.
            let zero_bin = self.bins.pop_front().unwrap();
            let reset_bin = &mut self.bins[ADULT_RESET];
            *reset_bin += zero_bin;

            // Push the total fish in the zero bin to the back - these are the new children.
            self.bins.push_back(zero_bin);
        }

        self.total_population()
    }

    fn total_population(&self) -> usize {
        self.bins.iter().sum()
    }
}

impl FromStr for FishModel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .split(',')
            .map(|v| v.parse().context("bad input"))
            .collect::<Result<Vec<usize>>>()?;

        let init = iter::repeat(0).take(CHILD_RESET + 1);
        let mut bins = VecDeque::from_iter(init);

        for v in data {
            let bin = &mut bins[v];
            *bin += 1;
        }

        Ok(FishModel { bins })
    }
}
