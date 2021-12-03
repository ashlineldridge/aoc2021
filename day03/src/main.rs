use std::io::{self, Read};

use anyhow::{ensure, Result};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let report = read_report(&input)?;

    part1(&report);
    part2(&report);

    Ok(())
}

fn part1(report: &Report) {
    let (gamma, epsilon) = report.gamma_epsilson();
    println!("Part 1 answer: {}", gamma * epsilon);
}

fn part2(report: &Report) {
    let (oxygen, co2) = report.oxygen_co2();
    println!("Part 2 answer: {}", oxygen * co2);
}

type Sample = u32;

struct Report {
    samples: Vec<Sample>,
    width: usize,
}

impl Report {
    fn gamma_epsilson(&self) -> (Sample, Sample) {
        let mut gamma = 0;
        for bit in 0..self.width {
            let mask = 1 << (self.width - bit - 1);
            let ones = self
                .samples
                .iter()
                .fold(0, |acc, s| if s & mask > 0 { acc + 1 } else { acc });

            if ones > self.samples.len() / 2 {
                gamma |= mask;
            }
        }

        let epsilon = !gamma & !(Sample::MAX << self.width);

        (gamma, epsilon)
    }

    fn oxygen_co2(&self) -> (Sample, Sample) {
        (self.reduce(true), self.reduce(false))
    }

    fn reduce(&self, oxygen: bool) -> Sample {
        let mut active = self.samples.clone();
        for bit in 0..self.width {
            if active.len() == 1 {
                break;
            }

            let mask = 1 << (self.width - bit - 1);

            let mut ones = vec![];
            let mut zeroes = vec![];
            for sample in active {
                if sample & mask > 0 {
                    ones.push(sample);
                } else {
                    zeroes.push(sample);
                }
            }

            active = match (ones.len(), zeroes.len(), oxygen) {
                // Oxygen criteria:
                (i, j, true) if i == j => ones,
                (i, j, true) if i > j => ones,
                (i, j, true) if i < j => zeroes,

                // CO2 criteria:
                (i, j, false) if i == j => zeroes,
                (i, j, false) if i > j => zeroes,
                (i, j, false) if i < j => ones,

                // Rust can't figure out that the match criteria above is complete.
                _ => panic!("algorithm error"),
            };
        }

        // The algorithm guarantees that as long as we started with at least a single
        // sample, one sample will remain.
        assert_eq!(active.len(), 1);
        *active.first().unwrap()
    }
}

fn read_report(input: &str) -> Result<Report> {
    let mut width = 0;
    let mut samples = vec![];

    for line in input.lines() {
        width = width.max(line.len());
        ensure!(
            width <= Sample::BITS as usize,
            "sample size bigger than expected"
        );

        samples.push(Sample::from_str_radix(line, 2)?);
    }

    Ok(Report { samples, width })
}
