use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, Read},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (polymer, rules) = read_input(&input)?;

    part1(polymer.clone(), &rules)?;
    part2(polymer, &rules)?;

    Ok(())
}

fn read_input(input: &str) -> Result<(Polymer, Vec<Rule>)> {
    let (head, tail) = input.split_once("\n\n").context("bad input")?;

    let polymer = head.parse()?;

    let mut rules = vec![];
    for line in tail.lines() {
        rules.push(line.parse()?);
    }

    Ok((polymer, rules))
}

fn part1(mut polymer: Polymer, rules: &[Rule]) -> Result<()> {
    polymer.repeat_apply(rules, 10);
    let (least, most) = polymer.freq_bounds().context("empty polymer")?;

    println!("Part 1 answer: {}", most - least);

    Ok(())
}

fn part2(mut polymer: Polymer, rules: &[Rule]) -> Result<()> {
    polymer.repeat_apply(rules, 40);
    let (least, most) = polymer.freq_bounds().context("empty polymer")?;

    println!("Part 2 answer: {}", most - least);

    Ok(())
}

type Element = char;
type ElementPair = (Element, Element);

#[derive(Clone)]
struct Polymer {
    pair_freqs: HashMap<ElementPair, usize>,
    elem_freqs: HashMap<Element, usize>,
}

impl Polymer {
    fn repeat_apply(&mut self, rules: &[Rule], times: usize) {
        for _ in 0..times {
            self.apply(rules);
        }
    }

    fn apply(&mut self, rules: &[Rule]) {
        let mut new_pair_freqs = HashMap::new();

        for (pair, freq) in &self.pair_freqs.clone() {
            for rule in rules {
                if *pair == rule.pair {
                    let left_pair = (rule.pair.0, rule.modifier);
                    let right_pair = (rule.modifier, rule.pair.1);

                    let left_freq = new_pair_freqs.entry(left_pair).or_default();
                    *left_freq += freq;

                    let right_freq = new_pair_freqs.entry(right_pair).or_default();
                    *right_freq += freq;

                    let elem_freq = self.elem_freqs.entry(rule.modifier).or_default();
                    *elem_freq += freq;

                    self.pair_freqs.remove(&rule.pair);

                    break;
                }
            }
        }

        for (new_pair, new_freq) in &new_pair_freqs {
            let freq = self.pair_freqs.entry(*new_pair).or_default();
            *freq += new_freq;
        }
    }

    fn freq_bounds(&self) -> Option<(usize, usize)> {
        if self.elem_freqs.is_empty() {
            return None;
        }

        let mut least = usize::MAX;
        let mut most = usize::MIN;
        for freq in self.elem_freqs.values() {
            least = least.min(*freq);
            most = most.max(*freq);
        }

        Some((least, most))
    }
}

impl FromStr for Polymer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        let mut pair_freqs = HashMap::new();
        for i in 1..bytes.len() {
            let pair = (bytes[i - 1] as Element, bytes[i] as Element);
            let freq = pair_freqs.entry(pair).or_default();
            *freq += 1;
        }

        let mut elem_freqs = HashMap::new();
        for b in bytes {
            let elem = *b as Element;
            let freq = elem_freqs.entry(elem).or_default();
            *freq += 1;
        }

        Ok(Polymer {
            pair_freqs,
            elem_freqs,
        })
    }
}

struct Rule {
    pair: ElementPair,
    modifier: Element,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<p1>[A-Z])(?P<p2>[A-Z]) -> (?P<m>[A-Z])$").unwrap();
        }

        let caps = RE.captures(s).ok_or_else(|| anyhow!("bad rule: {}", s))?;
        let pair = (caps["p1"].parse()?, caps["p2"].parse()?);
        let modifier = caps["m"].parse()?;

        Ok(Self { pair, modifier })
    }
}
