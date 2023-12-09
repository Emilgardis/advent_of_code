//! Advent of code 2023 4
//!
//! Link: <https://adventofcode.com/2023/day/4>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use std::collections::{BTreeSet, HashMap};

use aoc::{parts::*, Solver};
use eyre::{eyre, Report};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Card {
    winning_numbers: BTreeSet<usize>,
    numbers: BTreeSet<usize>,
    game: usize,
}

impl Card {
    pub fn points(&self) -> usize {
        let matches = self.winning_numbers.intersection(&self.numbers);
        let count = matches.count();
        let mut points = 0;
        if count > 0 {
            points += 1;
        }
        if count > 1 {
            for _ in 1..count {
                points *= 2;
            }
        }
        eprintln!("Card: {self:?} => {points:?}");
        points
    }
    pub fn matches(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }
}

impl Solver<Year2023, Day4, Part1> for Solution {
    type Input<'a> = Vec<Card>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        input
            .lines()
            .map(|s| s.trim())
            .map(|g| {
                let Some(g) = g.strip_prefix("Card ") else {
                    return Err(eyre!("Invalid game: {}", g));
                };
                let Some((game, numbers)) = g.split_once(": ") else {
                    return Err(eyre!("Invalid game: {}", g));
                };
                let Some((winning_numbers, numbers)) = numbers.split_once(" | ") else {
                    return Err(eyre!("Invalid game: {}", g));
                };
                let game = game.trim().parse()?;

                Ok(Card {
                    game,
                    numbers: numbers
                        .split_whitespace()
                        .map(|n| n.parse())
                        .try_collect()?,
                    winning_numbers: winning_numbers
                        .split_whitespace()
                        .map(|n| n.parse())
                        .try_collect()?,
                })
            })
            .collect()
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut sum = 0;
        for card in input {
            sum += card.points();
        }
        Ok(sum)
    }
}

impl Solver<Year2023, Day4, Part2> for Solution {
    type Input<'a> = <Self as Solver<2023, 4, Part1>>::Input<'a>;

    type Output = <Self as Solver<2023, 4, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2023, Day4, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut cards: HashMap<&Card, usize> = HashMap::new();
        for card in input {
            cards.insert(card, 1);
        }
        for card in input {
            let matches = card.matches();
            if matches > 0 {
                for i in card.game + 1..matches + card.game + 1 {
                    let card_num = *cards.get(card).unwrap();
                    println!(
                        "adding {}x card {i} to pile, due to card {} having {matches} matches",
                        card_num, card.game,
                    );
                    *cards.get_mut(&input[i - 1]).unwrap() += card_num;
                }
            }
        }
        Ok(cards.values().sum())
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day4, Part1>(input)?,
        13
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day4, Part2>(input)?,
        30
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day4, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day4, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
