use anyhow::{Context, Result};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    io::{self, Read},
    ops::Deref,
    ops::DerefMut,
};
use thiserror::Error;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut points = 0;
    for line in input.lines() {
        let mut chunks = ChunkVec::new();
        for ch in line.chars() {
            if chunks.consume(ch).is_err() {
                points += ILLEGAL_CHAR_POINTS[&ch];
                break;
            }
        }
    }

    println!("Part 1 answer: {}", points);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut line_points = vec![];
    'line: for line in input.lines() {
        let mut points = 0_u64;
        let mut chunks = ChunkVec::new();
        for ch in line.chars() {
            if chunks.consume(ch).is_err() {
                continue 'line;
            }
        }

        let mut closing_char = '}';
        while !chunks.is_complete() {
            match chunks.consume(closing_char) {
                Ok(_) => points = points * 5 + LEGAL_CHAR_POINTS[&closing_char] as u64,
                Err(ChunkError::ExpectedClosingChar(ch)) => closing_char = ch,
                e @ Err(_) => e?,
            }
        }

        line_points.push(points);
    }

    line_points.sort_unstable();
    let middle_score = line_points
        .get(line_points.len() / 2)
        .context("invalid input")?;

    println!("Part 2 answer: {:?}", middle_score);

    Ok(())
}

lazy_static! {
    // Mapping of opening to closing chars.
    static ref OPENING_TO_CLOSING: HashMap<char, char> =
        vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
            .into_iter()
            .collect();

    // Points map keyed by illegal char.
    static ref ILLEGAL_CHAR_POINTS: HashMap<char, u32> =
        vec![(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
            .into_iter()
        .collect();

    // Points map keyed by legal char.
    static ref LEGAL_CHAR_POINTS: HashMap<char, u32> =
        vec![(')', 1), (']', 2), ('}', 3), ('>', 4)]
            .into_iter()
            .collect();
}

struct Chunk {
    awaiting: char,
    closed: bool,
    children: ChunkVec,
}

struct ChunkVec(Vec<Chunk>);

impl Chunk {
    fn open(ch: char) -> Result<Self, ChunkError> {
        if let Some(&closing_char) = OPENING_TO_CLOSING.get(&ch) {
            Ok(Self {
                awaiting: closing_char,
                closed: false,
                children: ChunkVec::new(),
            })
        } else {
            Err(ChunkError::InvalidOpeningChar(ch))
        }
    }

    fn is_opening_char(ch: char) -> bool {
        OPENING_TO_CLOSING.get(&ch).is_some()
    }

    fn is_open(&self) -> bool {
        !self.closed
    }

    fn is_closed(&self) -> bool {
        self.closed
    }

    fn consume(&mut self, ch: char) -> Result<(), ChunkError> {
        if self.is_closed() {
            return Err(ChunkError::ChunkAlreadyClosed);
        }

        if Self::is_opening_char(ch) {
            return self.children.consume(ch);
        }

        if let Some(chunk) = self.children.last_mut() {
            if chunk.is_open() {
                return chunk.consume(ch);
            }
        }

        if ch == self.awaiting {
            self.closed = true;
        } else {
            return Err(ChunkError::ExpectedClosingChar(self.awaiting));
        }

        Ok(())
    }
}

impl ChunkVec {
    fn new() -> Self {
        Self(vec![])
    }

    fn consume(&mut self, ch: char) -> Result<(), ChunkError> {
        match self.last_mut() {
            Some(chunk) if chunk.is_open() => chunk.consume(ch),
            _ => {
                self.push(Chunk::open(ch)?);
                Ok(())
            }
        }
    }

    fn is_complete(&self) -> bool {
        if let Some(chunk) = self.last() {
            return chunk.is_closed();
        }

        true
    }
}

impl Deref for ChunkVec {
    type Target = Vec<Chunk>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChunkVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Error, Debug)]
enum ChunkError {
    #[error("chunk is already closed")]
    ChunkAlreadyClosed,

    #[error("invalid opening char {0}")]
    InvalidOpeningChar(char),

    #[error("expected closing char {0}")]
    ExpectedClosingChar(char),
}
