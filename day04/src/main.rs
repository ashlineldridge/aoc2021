use std::{
    collections::HashSet,
    io::{self, Read},
    str::FromStr,
};

use anyhow::{ensure, Context, Result};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut game: BingoGame = input.parse()?;

    part1(&mut game.clone());
    part2(&mut game);

    Ok(())
}

fn part1(game: &mut BingoGame) {
    match game.play_first_winner() {
        Some(score) => println!("Part 1 answer: {}", score),
        _ => println!("Part 1: no winner found"),
    }
}

fn part2(game: &mut BingoGame) {
    match game.play_last_winner() {
        Some(score) => println!("Part 2 answer: {}", score),
        _ => println!("Part 2: no winner found"),
    }
}

const BINGO_SIZE: usize = 5;

type Num = u32;

#[derive(Clone)]
struct BingoGame {
    sequence: Vec<Num>,
    cards: Vec<BingoCard>,
}

impl BingoGame {
    fn play_first_winner(&mut self) -> Option<Num> {
        for num in &self.sequence {
            for card in &mut self.cards {
                if let BingoResult::Win(score) = card.play(*num) {
                    return Some(score);
                }
            }
        }

        None
    }

    fn play_last_winner(&mut self) -> Option<Num> {
        let mut last_score = None;
        let mut winning_cards = HashSet::new();
        for num in &self.sequence {
            for (i, card) in self.cards.iter_mut().enumerate() {
                if winning_cards.contains(&i) {
                    continue;
                }

                if let BingoResult::Win(score) = card.play(*num) {
                    last_score = Some(score);
                    winning_cards.insert(i);
                }
            }
        }

        last_score
    }
}

impl FromStr for BingoGame {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once("\n\n").context("bad input")?;

        let sequence = head
            .split(',')
            .map(|v| v.parse().context("bad bingo sequence"))
            .collect::<Result<Vec<_>>>()?;

        let cards = tail
            .split_terminator("\n\n")
            .map(|v| v.parse().context("bad bingo card"))
            .collect::<Result<Vec<_>>>()?;

        Ok(BingoGame { sequence, cards })
    }
}

#[derive(Clone)]
struct BingoCard {
    grid: [[BingoValue; BINGO_SIZE]; BINGO_SIZE],
}

impl BingoCard {
    fn new() -> Self {
        BingoCard {
            grid: [[BingoValue::new(0); BINGO_SIZE]; BINGO_SIZE],
        }
    }

    fn play(&mut self, num: Num) -> BingoResult {
        for row in &mut self.grid {
            for val in row {
                if val.num == num {
                    val.marked = true;
                }
            }
        }

        self.result(num)
    }

    fn result(&self, last_played: Num) -> BingoResult {
        let mut unmarked = 0;
        let mut col_wins = [true; BINGO_SIZE];
        let mut row_wins = [true; BINGO_SIZE];

        for (y, row) in self.grid.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if !val.marked {
                    unmarked += val.num;
                    col_wins[x] = false;
                    row_wins[y] = false;
                }
            }
        }

        if col_wins.contains(&true) || row_wins.contains(&true) {
            BingoResult::Win(last_played * unmarked)
        } else {
            BingoResult::NoWin
        }
    }
}

impl FromStr for BingoCard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut card = BingoCard::new();
        for (row, line) in s.lines().enumerate() {
            ensure!(row < BINGO_SIZE, "card has too many rows");
            for (col, v) in line.split_whitespace().enumerate() {
                ensure!(col < BINGO_SIZE, "card has too many columns");
                card.grid[row][col] = BingoValue::new(v.parse()?);
            }
        }

        Ok(card)
    }
}

#[derive(Clone, Copy)]
struct BingoValue {
    num: Num,
    marked: bool,
}

impl BingoValue {
    fn new(value: Num) -> Self {
        BingoValue {
            num: value,
            marked: false,
        }
    }
}

enum BingoResult {
    Win(Num),
    NoWin,
}
