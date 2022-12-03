//! Advent of code 2022 2
//!
//! https://adventofcode.com/2022/day/2
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant [Rock Paper Scissors](https://en.wikipedia.org/wiki/Rock_paper_scissors) tournament is already in progress.
//!
//! Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.
//!
//! Appreciative of your help yesterday, one Elf gives you an *encrypted strategy guide* (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: `A` for Rock, `B` for Paper, and `C` for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
//!
//! The second column, you reason, must be what you should play in response: `X` for Rock, `Y` for Paper, and `Z` for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
//!
//! The winner of the whole tournament is the player with the highest score. Your *total score* is the sum of your scores for each round. The score for a single round is the score for the *shape you selected* (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the *outcome of the round* (0 if you lost, 3 if the round was a draw, and 6 if you won).
//!
//! Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.
//!
//! For example, suppose you were given the following strategy guide:
//!
//! ```
//! A Y
//! B X
//! C Z
//!
//! ```
//!
//! This strategy guide predicts and recommends the following:
//!
//! * In the first round, your opponent will choose Rock (`A`), and you should choose Paper (`Y`). This ends in a win for you with a score of *8* (2 because you chose Paper + 6 because you won).
//! * In the second round, your opponent will choose Paper (`B`), and you should choose Rock (`X`). This ends in a loss for you with a score of *1* (1 + 0).
//! * The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = *6*.
//!
//! In this example, if you were to follow the strategy guide, you would get a total score of `*15*` (8 + 1 + 6).
//!
//! *What would your total score be if everything goes exactly according to your strategy guide?*
//!
//! Your puzzle answer was `14827`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: `X` means you need to lose, `Y` means you need to end the round in a draw, and `Z` means you need to win. Good luck!"
//!
//! The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:
//!
//! * In the first round, your opponent will choose Rock (`A`), and you need the round to end in a draw (`Y`), so you also choose Rock. This gives you a score of 1 + 3 = *4*.
//! * In the second round, your opponent will choose Paper (`B`), and you choose Rock so you lose (`X`) with a score of 1 + 0 = *1*.
//! * In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = *7*.
//!
//! Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of `*12*`.
//!
//! Following the Elf's instructions for the second column, *what would your total score be if everything goes exactly according to your strategy guide?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](2/input).
//! <!---ENDOFDESCRIPTION--->
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

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
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{}", s))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day2, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{}", s))
}
