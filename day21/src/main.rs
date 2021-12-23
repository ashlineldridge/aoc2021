use anyhow::{ensure, Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    hash::Hash,
    io::{self, Read},
};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let rules = GameRules {
        last_position: 10,
        winning_score: 1000,
    };

    let mut game = read_game(input, rules)?;
    let mut die = SimpleDie::new();

    let (winner, _) = game.play(&mut die).context("no winner")?;
    let losing_score = game.dead_scores[&winner.other()];

    println!("Part 1 answer: {}", die.rolls * losing_score);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let rules = GameRules {
        last_position: 10,
        winning_score: 21,
    };

    let mut game = read_game(input, rules)?;
    let mut die = QuantumDie::new();

    let (_, total_games) = game.play(&mut die).context("no winner")?;

    println!("Part 2 answer: {}", total_games);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    P1,
    P2,
}

impl Player {
    fn other(&self) -> Self {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }
}

type Position = u32;
type Score = usize;

struct GameRules {
    last_position: Position,
    winning_score: Score,
}

struct Game {
    rules: GameRules,
    live_scores: HashMap<PositionTuple, HashMap<ScoreTuple, usize>>,
    dead_scores: HashMap<Player, usize>,
    wins: HashMap<Player, usize>,
}

impl Game {
    fn new(p1: Position, p2: Position, rules: GameRules) -> Self {
        Self {
            rules,
            live_scores: [(
                PositionTuple::new(Player::P1, p1, p2),
                [(ScoreTuple::new(Player::P1, 0, 0), 1)]
                    .into_iter()
                    .collect(),
            )]
            .into_iter()
            .collect(),
            dead_scores: [(Player::P1, 0), (Player::P2, 0)].into_iter().collect(),
            wins: [(Player::P1, 0), (Player::P2, 0)].into_iter().collect(),
        }
    }

    fn play<D: Die>(&mut self, die: &mut D) -> Option<(Player, usize)> {
        let mut this_player = Player::P1;

        loop {
            let that_player = this_player.other();
            let rolls = die.roll();
            let mut new_scores: HashMap<PositionTuple, HashMap<ScoreTuple, usize>> = HashMap::new();

            for (&pos_tuple, pos_scores) in &self.live_scores {
                let (this_pos, that_pos) = pos_tuple.as_tuple(this_player, that_player);

                for (&score_tuple, &total_games) in pos_scores {
                    let (this_score, that_score) = score_tuple.as_tuple(this_player, that_player);

                    for roll in &rolls {
                        let new_this_pos = (this_pos + roll - 1) % self.rules.last_position + 1;
                        let new_this_score = this_score + new_this_pos as Score;

                        if new_this_score >= self.rules.winning_score {
                            let this_wins = self.wins.entry(this_player).or_default();
                            *this_wins += total_games;

                            let that_dead_points = self.dead_scores.entry(that_player).or_default();
                            *that_dead_points += total_games * that_score;
                        } else {
                            let new_pos_tuple =
                                PositionTuple::new(this_player, new_this_pos, that_pos);
                            let new_score_tuple =
                                ScoreTuple::new(this_player, new_this_score, that_score);

                            let new_pos_scores = new_scores.entry(new_pos_tuple).or_default();
                            let new_total_games =
                                new_pos_scores.entry(new_score_tuple).or_default();
                            *new_total_games += total_games;
                        }
                    }
                }
            }

            self.live_scores = new_scores;
            if self.live_scores.is_empty() {
                break;
            }

            this_player = that_player;
        }

        match (self.wins[&Player::P1], self.wins[&Player::P2]) {
            (w1, w2) if w1 > w2 => Some((Player::P1, w1)),
            (w1, w2) if w2 > w1 => Some((Player::P2, w2)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // TODO: Test hash
struct PositionTuple((Position, Position));

impl PositionTuple {
    fn new(player: Player, this: Position, that: Position) -> Self {
        match player {
            Player::P1 => Self((this, that)),
            Player::P2 => Self((that, this)),
        }
    }

    fn get(&self, player: Player) -> Position {
        match player {
            Player::P1 => self.0 .0,
            Player::P2 => self.0 .1,
        }
    }

    fn as_tuple(&self, px: Player, py: Player) -> (Position, Position) {
        (self.get(px), self.get(py))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // TODO: Test hash
struct ScoreTuple((Score, Score));

impl ScoreTuple {
    fn new(player: Player, this: Score, that: Score) -> Self {
        match player {
            Player::P1 => Self((this, that)),
            Player::P2 => Self((that, this)),
        }
    }

    fn get(&self, player: Player) -> Score {
        match player {
            Player::P1 => self.0 .0,
            Player::P2 => self.0 .1,
        }
    }

    fn as_tuple(&self, px: Player, py: Player) -> (Score, Score) {
        (self.get(px), self.get(py))
    }
}

type DieValue = u32;

trait Die {
    fn roll(&mut self) -> Vec<DieValue>;
}

#[derive(Clone)]
struct QuantumDie {}

lazy_static! {
    static ref QUANTUM_DIE_VALUES: Vec<DieValue> = vec![1, 2, 3];
    static ref QUANTUM_DIE_ROLL_SUMS: Vec<DieValue> = QUANTUM_DIE_VALUES
        .iter()
        .cartesian_product(QUANTUM_DIE_VALUES.iter())
        .cartesian_product(QUANTUM_DIE_VALUES.iter())
        .map(|v| v.0 .0 + v.0 .1 + v.1)
        .collect::<Vec<_>>();
}

impl QuantumDie {
    fn new() -> Self {
        Self {}
    }
}

impl Die for QuantumDie {
    fn roll(&mut self) -> Vec<DieValue> {
        QUANTUM_DIE_ROLL_SUMS.clone()
    }
}

#[derive(Clone)]
struct SimpleDie {
    rolls: usize,
}

lazy_static! {
    static ref SIMPLE_DIE_MAX_VALUE: DieValue = 100;
}

impl SimpleDie {
    fn new() -> Self {
        Self { rolls: 0 }
    }

    fn next(&mut self) -> DieValue {
        self.rolls += 1;
        (self.rolls as DieValue - 1) % *SIMPLE_DIE_MAX_VALUE + 1
    }
}

impl Die for SimpleDie {
    fn roll(&mut self) -> Vec<DieValue> {
        vec![self.next() + self.next() + self.next()]
    }
}

fn read_game(input: &str, rules: GameRules) -> Result<Game> {
    ensure!(input.lines().count() == 2, "game can only have two players");

    let start_positions = input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .context("bad input")
                .and_then(|(_, p)| p.parse().context("bad start position"))
        })
        .collect::<Result<Vec<Position>>>()?;

    ensure!(start_positions.len() == 2, "game requires two players");
    let p1 = *start_positions.get(0).unwrap();
    let p2 = *start_positions.get(1).unwrap();

    Ok(Game::new(p1, p2, rules))
}
