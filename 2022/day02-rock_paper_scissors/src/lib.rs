//! Advent of code 2022 2
//!
//!! Link: <https://adventofcode.com/2022/day/2>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Game1 {
    opponent: Fist,
    suggestion: Fist,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Game2 {
    opponent: Fist,
    suggestion: GameOutcome,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Fist {
    Rock,
    Paper,
    Scissor,
}

impl Fist {
    pub fn score(self) -> u64 {
        match self {
            Fist::Rock => 1,
            Fist::Paper => 2,
            Fist::Scissor => 3,
        }
    }
    pub fn outcome(self, opponent: Fist) -> GameOutcome {
        use self::GameOutcome::*;
        match (self, opponent) {
            (Fist::Rock, Fist::Scissor) => Win,
            (Fist::Paper, Fist::Rock) => Win,
            (Fist::Scissor, Fist::Paper) => Win,

            (a, b) if a == b => Draw,

            _ => Loss,
        }
    }

    fn pick_outcome(&self, suggestion: GameOutcome) -> Fist {
        match suggestion {
            GameOutcome::Draw => *self,
            GameOutcome::Win => match self {
                Fist::Rock => Fist::Paper,
                Fist::Paper => Fist::Scissor,
                Fist::Scissor => Fist::Rock,
            },
            GameOutcome::Loss => match self {
                Fist::Rock => Fist::Scissor,
                Fist::Paper => Fist::Rock,
                Fist::Scissor => Fist::Paper,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameOutcome {
    Draw,
    Win,
    Loss,
}

impl GameOutcome {
    fn score(self) -> u64 {
        match self {
            GameOutcome::Draw => 3,
            GameOutcome::Win => 6,
            GameOutcome::Loss => 0,
        }
    }
}

impl Game1 {
    pub fn from_line(line: &'_ str) -> Self {
        let (opponent, suggestion) = match line.split_once(' ') {
            Some((opponent, suggestion)) => (
                match opponent {
                    "A" => Fist::Rock,
                    "B" => Fist::Paper,
                    "C" => Fist::Scissor,
                    _ => panic!("invalid choice: {line}"),
                },
                match suggestion {
                    "X" => Fist::Rock,
                    "Y" => Fist::Paper,
                    "Z" => Fist::Scissor,
                    _ => panic!("invalid choice: {line}"),
                },
            ),
            None => panic!("invalid choices: {line}"),
        };
        Game1 {
            opponent,
            suggestion,
        }
    }
    pub fn score(&self) -> u64 {
        self.suggestion.score() + self.suggestion.outcome(self.opponent).score()
    }

    fn dbg(&self) {
        let sug = self.suggestion;
        let opp = self.opponent;
        let score = self.score();
        println!(
            "Game: {sug:?} - {opp:?} = {score} ({} + {:?} ({}))",
            self.suggestion.score(),
            self.suggestion.outcome(self.opponent),
            self.suggestion.outcome(self.opponent).score()
        )
    }
}

impl Game2 {
    pub fn from_line(line: &'_ str) -> Self {
        let (opponent, suggestion) = match line.split_once(' ') {
            Some((opponent, suggestion)) => (
                match opponent {
                    "A" => Fist::Rock,
                    "B" => Fist::Paper,
                    "C" => Fist::Scissor,
                    _ => panic!("invalid choice: {line}"),
                },
                match suggestion {
                    "X" => GameOutcome::Loss,
                    "Y" => GameOutcome::Draw,
                    "Z" => GameOutcome::Win,
                    _ => panic!("invalid choice: {line}"),
                },
            ),
            None => panic!("invalid choices: {line}"),
        };
        Game2 {
            opponent,
            suggestion,
        }
    }
    pub fn score(&self) -> u64 {
        self.opponent.pick_outcome(self.suggestion).score() + self.suggestion.score()
    }

    fn dbg(&self) {
        let sug = self.suggestion;
        let opp = self.opponent;
        let score = self.score();
        println!(
            "Game: {sug:?} - {opp:?} = {score} ({} + {:?} ({}))",
            self.suggestion.score(),
            self.suggestion,
            self.suggestion.score()
        )
    }
}

impl Solver<Year2022, Day2, Part1> for Solution {
    type Input<'a> = Vec<Game1>;

    type Output = u64;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input.trim().lines().map(Game1::from_line).collect())
    }

    fn solve(input: &Vec<Game1>) -> Result<Self::Output, Report> {
        let result = input.iter().fold(0, |r, game| {
            game.dbg();
            game.score() + r
        });
        Ok(result)
    }
}

impl Solver<Year2022, Day2, Part2> for Solution {
    type Input<'a> = Vec<Game2>;

    type Output = u64;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input.trim().lines().map(Game2::from_line).collect())
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let result = input.iter().fold(0, |r, game| {
            game.dbg();
            game.score() + r
        });
        Ok(result)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
A Y
B X
C Z
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day2, Part1>(input)?,
        15
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
A Y
B X
C Z
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day2, Part2>(input)?,
        12
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day2, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day2, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
