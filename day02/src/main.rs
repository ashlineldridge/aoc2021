use anyhow::{anyhow, bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    io::{self, Read},
    str::FromStr,
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let commands = parse_commands(&input)?;

    part1(&commands);
    part2(&commands);

    Ok(())
}

fn part1(commands: &[Command]) {
    let end = commands
        .iter()
        .fold(Position::origin(), |pos, c| c.run_simple(pos));

    println!("Part 1 answer: {}", end.x * end.y);
}

fn part2(commands: &[Command]) {
    let end = commands
        .iter()
        .fold(Position::origin(), |pos, c| c.run_aimed(pos));

    println!("Part 2 answer: {}", end.x * end.y);
}

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

impl Position {
    fn new(x: i32, y: i32, aim: i32) -> Self {
        Self { x, y, aim }
    }

    fn origin() -> Self {
        Self::new(0, 0, 0)
    }
}

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl Command {
    fn run_simple(&self, pos: Position) -> Position {
        match self {
            Command::Up(n) => Position::new(pos.x, pos.y - n, pos.aim),
            Command::Down(n) => Position::new(pos.x, pos.y + n, pos.aim),
            Command::Forward(n) => Position::new(pos.x + n, pos.y, pos.aim),
        }
    }

    fn run_aimed(&self, pos: Position) -> Position {
        match self {
            Command::Up(n) => Position::new(pos.x, pos.y, pos.aim - n),
            Command::Down(n) => Position::new(pos.x, pos.y, pos.aim + n),
            Command::Forward(n) => Position::new(pos.x + n, pos.y + pos.aim * n, pos.aim),
        }
    }
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<dir>[a-z]+) (?P<n>\d+)$").unwrap();
        }

        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow!("invalid instruction: {}", s))?;

        let n = caps["n"].parse()?;

        match &caps["dir"] {
            "up" => Ok(Self::Up(n)),
            "down" => Ok(Self::Down(n)),
            "forward" => Ok(Self::Forward(n)),
            _ => bail!("unknown instruction: {}", s),
        }
    }
}

fn parse_commands(input: &str) -> Result<Vec<Command>> {
    input.lines().map(|line| line.parse()).collect()
}
